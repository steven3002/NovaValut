// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{ alloy_primitives::{ U256, U32, Address, U64 }, prelude::*, msg, evm, block };

use alloy_sol_types::sol;

// const ADMIN: &str = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

sol_storage! {
    #[entrypoint]
    pub struct Subject {
        mapping(uint256 => Gallery) gallery;
        // this will be mapping the ticket to the user that have paid for it
        mapping(address => mapping(uint256 => bool)) ticket_index;
        // this will be where the list of gallery one has created will be;
        mapping(address => UserData) state;

        // this address is the address of the contract allowed to buy tickets;
        address allowed_contract;
        address admin;

        // unused index for creation of gallery
        uint256 available_index;
    }

    pub struct Gallery{
        string name;
        string meta_data;
        uint256 price;
        // this will give the user the right to the admin to set the functionalities 
        // and to also recive funds from the gallery
        address owner;
        uint32 attendes;
        uint64 created_at;
        // uint256 total_votes;
        // this will be the total nfts in this gallery
        // uint32 total_nfts;
        // uint32[] leaderboard;

        VotingCondition conditions;
    }

    pub struct VotingCondition {
        uint64 voting_start;
        uint64 voting_end;
        uint256 minimum_staking_amount;
    }

    pub struct UserData{
        uint256[] created_gallery;
        uint256[] joined_gallery;
    }

    
}

//  ======   Errors and Events  =========  //

sol! {
    // event to show that a new gallary have been created
    event NewGallery(address indexed creator, string name, uint256 gallery_index, uint256 indexed price);

    // event to show that a user has joined a gallery
    event JoinedGallery(uint256 indexed gallery_index, address indexed member);


    // my error
    // error to show invalid parameter
    error InvalidParameter(uint8 point);

    error NoData();

    error DeniedAccess(uint256 gallery_index);
    error InSufficientAllowance(uint256 gallery_index);
}

#[derive(SolidityError)]
pub enum GalleryError {
    InvalidParameter(InvalidParameter),
    DeniedAccess(DeniedAccess),
    InSufficientAllowance(InSufficientAllowance),
    NoData(NoData),
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Subject {
    pub fn create_gallery(
        &mut self,
        name: String,
        meta_data: String,
        price: U256,
        voting_start: u64,
        voting_end: u64,
        minimum_staking_amount: U256
    ) -> Result<(), GalleryError> {
        if
            name.is_empty() ||
            meta_data.is_empty() ||
            voting_start < block::timestamp() ||
            voting_end < block::timestamp() ||
            voting_start >= voting_end
        {
            return Err(
                GalleryError::InvalidParameter(InvalidParameter {
                    point: 0,
                })
            );
        }

        // starting index from 1 to reduce paralax error
        let available_index = self.available_index.get() + U256::from(1);
        let mut new_gallery = self.gallery.setter(available_index);
        new_gallery.name.set_str(name.clone());
        new_gallery.meta_data.set_str(meta_data);
        new_gallery.price.set(price);
        new_gallery.owner.set(msg::sender());
        new_gallery.created_at.set(U64::from(block::timestamp()));
        new_gallery.conditions.voting_start.set(U64::from(voting_start));
        new_gallery.conditions.voting_end.set(U64::from(voting_end));
        new_gallery.conditions.minimum_staking_amount.set(minimum_staking_amount);

        let mut state = self.state.setter(msg::sender());
        state.created_gallery.push(available_index);

        let mut ticket = self.ticket_index.setter(msg::sender());
        let mut point = ticket.setter(available_index);

        point.set(true);

        evm::log(NewGallery {
            creator: msg::sender(),
            name,
            gallery_index: available_index,
            price,
        });

        self.available_index.set(available_index);
        Ok(())
    }

    pub fn buy_ticket(&mut self, gallery_index: U256, user: Address) -> Result<(), GalleryError> {
        if msg::sender() != self.allowed_contract.get() {
            return Err(
                GalleryError::DeniedAccess(DeniedAccess {
                    gallery_index,
                })
            );
        }

        let mut gallery = self.gallery.setter(gallery_index);
        let attendes = gallery.attendes.get();
        gallery.attendes.set(attendes + U32::from(1));

        // this will set the list of joined attendance
        let mut user_list = self.state.setter(user);
        user_list.joined_gallery.push(gallery_index);

        evm::log(JoinedGallery {
            gallery_index,
            member: user,
        });

        // this will give them the ticket
        let mut ticket = self.ticket_index.setter(user);
        let mut point = ticket.setter(gallery_index);
        point.set(true);
        Ok(())
    }

    // this function is the function used to set the allowed function;
    // here there will be an address constant that will be responsible for this
    pub fn set_a_c(&mut self, cn_address: Address) -> Result<(), GalleryError> {
        // let admin = Address::parse_checksummed(ADMIN, None).expect("Invalid address");
        // so using the above makes the file 27.3kb and can not deploy.
        // so this is to work around it
        //  the function is to check if the admin have been changed
        let default_x = Address::from([0x00; 20]);
        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                GalleryError::InvalidParameter(InvalidParameter {
                    point: 9,
                })
            );
        } else {
            self.admin.set(msg::sender());
        }

        self.allowed_contract.set(cn_address);
        Ok(())
    }

    //  ===== view funtions  ====== //
    pub fn get_last_index(&self) -> U256 {
        self.available_index.get()
    }

    pub fn get_gallery(
        &self,
        gallery_index: U256
    ) -> Result<(Address, String, String, u32, u64, U256, u64, u64, U256), GalleryError> {
        if gallery_index > self.available_index.get() {
            return Err(
                GalleryError::InvalidParameter(InvalidParameter {
                    point: 1,
                })
            );
        }

        let gallery = self.gallery.getter(gallery_index);

        return Ok((
            gallery.owner.get(),
            gallery.name.get_string(),
            gallery.meta_data.get_string(),
            gallery.attendes.get().to::<u32>(),
            gallery.created_at.get().to::<u64>(),
            gallery.price.get(),
            gallery.conditions.voting_end.get().to::<u64>(),
            gallery.conditions.voting_start.get().to::<u64>(),
            gallery.conditions.minimum_staking_amount.get(),
        ));
    }

    pub fn get_user_status(&self, gallery_index: U256, user: Address) -> bool {
        let user_ticket_status = self.ticket_index.getter(user);
        let status = user_ticket_status.getter(gallery_index);
        status.get()
    }

    pub fn in_session(&self, gallery_index: U256) -> bool {
        let condition = self.gallery.getter(gallery_index);
        let start = condition.conditions.voting_start.get();
        let end = condition.conditions.voting_end.get();

        if start <= U64::from(block::timestamp()) && end >= U64::from(block::timestamp()) {
            return true;
        }
        false
    }

    // this to get the minimum staking amount
    pub fn get_mim_s_a(&self, gallery_index: U256) -> U256 {
        let condition = self.gallery.getter(gallery_index);
        condition.conditions.minimum_staking_amount.get()
    }

    //     // this is the function that get the user created gallery or joined gallary
    pub fn get_uc(&self, index: u64, user: Address, state: u8) -> Result<U256, GalleryError> {
        let state_data = self.state.getter(user);

        let data = match state {
            0 => state_data.created_gallery.get(index as usize), // get the gallery that was created by the user
            1 => state_data.joined_gallery.get(index as usize), // get the gallery that was joined by the uesr
            _ => None, // Return None if the state is not recognized
        };

        if let Some(data) = data {
            return Ok(data);
        } else {
            return Err(GalleryError::NoData(NoData {}));
        }
    }

    // this is to get the lenght of the user created gallery or joined_gallay
    pub fn get_len_uc(&self, user: Address, state: u8) -> Result<U256, GalleryError> {
        match state {
            0 => {
                // Get the length of the created gallery
                Ok(U256::from(self.state.getter(user).created_gallery.len()))
            }
            1 => {
                // Get the length of the joined gallery
                Ok(U256::from(self.state.getter(user).joined_gallery.len()))
            }
            _ => {
                // Return an error if the state is not 0 or 1
                Err(GalleryError::InvalidParameter(InvalidParameter { point: 3 }))
            }
        }
    }
}
