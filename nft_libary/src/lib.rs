#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U8, U256 };
use stylus_sdk::{ prelude::*, msg, block, evm };
use core::{ marker::PhantomData };
use alloy_sol_types::sol;
use stylus_sdk::call::Call;

sol_storage! {
    #[entrypoint]
    pub struct Mainx {
        // this will be mapping the gallary inmdex with the 
        // mapping of the nfts with status; so the data will be transfered to the safe mapping
        mapping(uint256 => Concept) gallary_data;
        uint256 total_nft;
        address gallery_c;
        address nft_submit;
        address admin;


        // PhantomData<NftData> phantom;
    }

    pub struct Nft{
        // this status will be as follows
        // 0 undergoing review
        // 1 accepted 
        // 2 rejected
        uint8 status;
        address owner;
        uint256 data;
    }

    pub struct Concept{
        // the data_x is the raw data of the gallery
        mapping(uint256 => Nft) data_x;
        // this maps the accepted nft to a new index; for easy mapping in the staking cotract
        mapping(uint256 => uint256) accepted;
        uint256 available_index;
        uint256 av_accepted_index;

       
    }
}

// a nft index will be (gallery index, nft, id )
sol_interface! {
    interface ISubject {
        function getGallery(uint256 gallery_index) external view returns (address, string memory, string memory, uint32, uint64, uint256, uint64, uint64, uint256);
        function getLastIndex() external view returns (uint256);
        function getUserStatus(uint256 gallery_index, address user) external view returns (bool);
    }    
}

sol! {

    

    event AcceptedNft(address indexed creator, uint256 gallery_id, uint256 approved_nft_id);
    event RegetedNft(address indexed creator, uint256 gallery_id, uint256 nft_id);
    event SubmitedNft(uint256 indexed gallery_id, address creator, uint256 nft_index);

  
    // my error
    // error to show invalid parameter
    error InvalidParameter(uint8 point);

    error NoData();

    error InSufficientAllowance(uint256 gallery_index);


    error DeniedAccess(uint8  point);
}

#[derive(SolidityError)]
pub enum NftError {
    InvalidParameter(InvalidParameter),
    InSufficientAllowance(InSufficientAllowance),
    NoData(NoData),
    DeniedAccess(DeniedAccess),
}

#[public]
impl Mainx {
    // submiting of nft will come from the nft_submit contract as this contract can not hold any more data
    // nft_data is the index of the data of the nft being stored in the nft_submit libary
    pub fn submit_nft(
        &mut self,
        gallery_id: U256,
        user: Address,
        nft_data: U256
    ) -> Result<(), NftError> {
        // Check gallery's cooldown and gallery index
        let (_creator, start) = self
            .get_gal_info(gallery_id)
            .map_err(|_| { NftError::InvalidParameter(InvalidParameter { point: 7 }) })?;
        self.cd_ck(gallery_id, start)?;
        if msg::sender() != self.nft_submit.get() {
            return Err(NftError::DeniedAccess(DeniedAccess { point: 1 }));
        }
        // add the nft to libary and create event to show that it was succesful
        let mut gallery_con = self.gallary_data.setter(gallery_id);
        let available_index = gallery_con.available_index.get();
        let mut g_con_data = gallery_con.data_x.setter(available_index);
        g_con_data.owner.set(user);
        g_con_data.data.set(nft_data);

        // this is to alart the gallery that a new nft has been submited for review
        evm::log(SubmitedNft {
            gallery_id,
            creator: user,
            nft_index: available_index,
        });

        Ok(())
    }

    // Accept an NFT
    pub fn set_nft_state(
        &mut self,
        gallery_id: U256,
        nft_id: U256,
        state: u8
    ) -> Result<(), NftError> {
        // Retrieve gallery info
        let (creator, start) = self
            .get_gal_info(gallery_id)
            .map_err(|_| { NftError::InvalidParameter(InvalidParameter { point: 7 }) })?;

        // Check if the sender is the creator
        if creator != msg::sender() {
            return Err(NftError::DeniedAccess(DeniedAccess { point: 1 }));
        }

        // Validate the state value
        if !(1..=2).contains(&state) {
            return Err(NftError::InvalidParameter(InvalidParameter { point: 11 }));
        }

        // Check gallery's cooldown and gallery index
        self.cd_ck(gallery_id, start)?;

        // Access the gallery data
        let mut gallery_con = self.gallary_data.setter(gallery_id);
        let available_index = gallery_con.available_index.get();

        // Validate the NFT ID
        if nft_id >= available_index {
            return Err(NftError::InvalidParameter(InvalidParameter { point: 1 }));
        }

        // Attempt to update the gallery data
        let updated = {
            let mut g_con_data = gallery_con.data_x.setter(nft_id);
            if g_con_data.status.get() != U8::from(0) {
                false
            } else {
                g_con_data.status.set(U8::from(state)); // Assuming `status` should be `state`
                true
            }
        };

        if !updated {
            return Err(NftError::InvalidParameter(InvalidParameter { point: 1 }));
        }

        let creator_address = gallery_con.data_x.getter(nft_id).owner.get();
        let accepted_index = gallery_con.av_accepted_index.get();

        // this means that the nft has been rejected
        if state == 2 {
            evm::log(RegetedNft {
                creator: creator_address,
                gallery_id,
                nft_id,
            });

            return Ok(());
        }

        // Add the NFT to the accepted data
        let mut acc_x = gallery_con.accepted.setter(accepted_index);
        acc_x.set(nft_id);

        evm::log(AcceptedNft {
            creator: creator_address,
            gallery_id,
            approved_nft_id: accepted_index,
        });

        gallery_con.av_accepted_index.set(accepted_index + U256::from(1));
        // Update the total number of NFTs in the system
        let old_total = self.total_nft.get();
        self.total_nft.set(old_total + U256::from(1));

        Ok(())
    }

    // this is to return the lenght of the accepted list
    pub fn nft_list_len(&self, gallery_id: U256) -> Result<(U256, U256), NftError> {
        // add the nft to libary and create event to show that it was succesful
        let gallery_con = self.gallary_data.getter(gallery_id);

        let available_index = gallery_con.available_index.get();

        let len_t = gallery_con.av_accepted_index.get();

        Ok((available_index, len_t))
    }

    pub fn set_gallery_submit(
        &mut self,
        submit_address: Address,
        gallery_address: Address
    ) -> Result<(), NftError> {
        self.check_admin().map_err(|e| { e })?;
        self.nft_submit.set(submit_address);
        self.gallery_c.set(gallery_address);
        Ok(())
    }

    pub fn get_nft(
        &self,
        gallery_index: U256,
        nft_id: U256,
        raw: bool
    ) -> Result<(Address, u8, U256), NftError> {
        // this is to get the index of the raw nft data;
        // Retrieve gallery info
        // raw if true then get raw state
        let gallery_con = self.gallary_data.getter(gallery_index);
        if raw {
            let (creator, _start) = self
                .get_gal_info(gallery_index)
                .map_err(|_| { NftError::InvalidParameter(InvalidParameter { point: 7 }) })?;

            // Check if the sender is the creator
            if creator != msg::sender() {
                return Err(NftError::DeniedAccess(DeniedAccess { point: 1 }));
            }

            let g_c = gallery_con.data_x.getter(nft_id);

            return Ok((g_c.owner.get(), g_c.status.get().to::<u8>(), g_c.data.get()));
        }
        let nc = gallery_con.accepted.getter(nft_id);

        let g_c = gallery_con.data_x.getter(nc.get());
        Ok((g_c.owner.get(), g_c.status.get().to::<u8>(), g_c.data.get()))
    }
}

// helper functions
impl Mainx {
    // function to check if the user has a ticket
    pub fn c_tik(&self, gallery_index: U256) -> bool {
        let user = msg::sender();
        let address = self.gallery_c.get();
        let gallery_contract = ISubject::new(address);
        let config = Call::new();
        gallery_contract.get_user_status(config, gallery_index, user).expect("drat")
    }

    // this will get the important gallery information from the gallery smart contract
    // creator address and the time the gallery is to start;
    pub fn get_gal_info(&self, gallery_index: U256) -> Result<(Address, u64), ()> {
        let address = self.gallery_c.get();
        let gallery_contract = ISubject::new(address);
        let config = Call::new();
        let data = gallery_contract.get_gallery(config, gallery_index).expect("drat");
        Ok((data.0, data.7))
    }

    pub fn cd_ck(&self, gallery_id: U256, start: u64) -> Result<bool, NftError> {
        // check if the gallery_id is correct
        // if they have a ticket
        // and to make sure that the event have not yet started.
        if !self.c_tik(gallery_id) || start <= block::timestamp() {
            return Err(
                NftError::InvalidParameter(InvalidParameter {
                    point: 11,
                })
            );
        }

        Ok(true)
    }

    pub fn check_admin(&mut self) -> Result<bool, NftError> {
        let default_x = Address::from([0x00; 20]);
        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                NftError::InvalidParameter(InvalidParameter {
                    point: 17,
                })
            );
        } else if self.admin.get() == default_x {
            self.admin.set(msg::sender());
            return Ok(true);
        } else {
            return Ok(true);
        }
    }
}
