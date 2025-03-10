#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U256 };
use stylus_sdk::{ prelude::*, msg, evm };
use alloy_sol_types::sol;
// use alloy_sol_types::*;

use stylus_sdk::call::Call;

// this contract holds the meta_data of nfts;
sol_storage! {
    #[entrypoint]
    // this is the nft storage point
    pub struct NftStorage{
        mapping(uint256 => Nft) data; // identity of nft meta_data
        uint256 available_index; // new identity of nft
        address admin; // admin address, the user that can set the controls of the system itself
        address libary; // address of the nft_libary; where the actual system index of the gallery to the nft
        address gallery_c; // address of the gallery contract
        address minter; // contract that addresses the minting of the nft
    }

    // Nft meta_data
    pub struct Nft{
        uint256 gallery; //this is the gallery the nft is registered to
        // uint256 system_nft_id; //during minting; this will be the nft id it was registered to in the platform; given users the ability to check the staking state of the nft after minting
        string data; //this is the meta_data of the nft stringified json
        address owner; //creator of the nft
        bool open; // when minting we will use this to check if the data has been concluded and the nft is open for all to view 

    }
}

sol_interface! {
    interface IMainx {
        function submitNft(uint256 gallery_id, address user, uint256 nft_data) external;
    }
    interface ISubject {
        function getUserStatus(uint256 gallery_index, address user) external view returns (bool);
    }   
}

//  ======   Errors and Events  =========  //

sol! {
    // event to show that a new gallary have been created
    event SubmitNft(address indexed owner, uint256  gallery_index, uint256 nft_id);


    // my error
    // error to show invalid parameter
    error InvalidParameter(uint8 point);
}

#[derive(SolidityError)]
pub enum SubmitError {
    InvalidParameter(InvalidParameter),
}

#[public]
impl NftStorage {
    // called to submit an nft to a gallery in the platform
    pub fn submit_nft(&mut self, gallery_id: U256, data: String) -> Result<(), SubmitError> {
        // starting it from 1; because the none value is <0> and that can cause permission parallax error
        let available_index = self.available_index.get() + U256::from(1); // getting an identity for the new nft

        // passing the data, is also used to check the parameters of the conditions like if the user has a ticket and to make sure that the event has not started
        self.pass_data(gallery_id, available_index)?;

        // setting the meta_data
        let mut data_state = self.data.setter(available_index);
        data_state.data.set_str(data);
        data_state.owner.set(msg::sender());
        data_state.gallery.set(gallery_id);

        self.available_index.set(available_index); // creating a new identity

        evm::log(SubmitNft {
            owner: msg::sender(),
            gallery_index: gallery_id,
            nft_id: available_index,
            // time: block::timestamp() as u64,
        });

        Ok(())
    }

    // countroled method of getting the nft meta_data
    pub fn get_nft_data(&self, nft_id: U256) -> Result<(Address, String, U256), SubmitError> {
        let data_x = self.data.getter(nft_id);
        // making sure that only people that have a ticket to the gallery can see the nft
        if
            !data_x.open.get() && //first check if the nft is open to view
            msg::sender() != self.minter.get() && // check if the user is the minter
            !self.c_tik(data_x.gallery.get()) // check if the user has a tikect
        {
            return Err(
                SubmitError::InvalidParameter(InvalidParameter {
                    point: 17,
                })
            );
        }
        // returning
        // (the creator of the nft, the meta_data of the nft, (gallery id , system nft id ))
        Ok((data_x.owner.get(), data_x.data.get_string(), data_x.gallery.get()))
    }

    // this function is called by the minter to mint the nft
    pub fn system_mint(&mut self, nft_id: U256) -> Result<(), SubmitError> {
        let mut data_x = self.data.setter(nft_id);

        // this is to make sure that only the minter can mint
        if msg::sender() != self.minter.get() {
            return Err(
                SubmitError::InvalidParameter(InvalidParameter {
                    point: 101,
                })
            );
        }

        data_x.open.set(true);
        Ok(())
    }
    // admin method of setting the contract address
    // can only be done by the admin of the contract
    pub fn set_libary(
        &mut self,
        libary_address: Address,
        gallery: Address,
        minter: Address
    ) -> Result<(), SubmitError> {
        self.check_admin().map_err(|e| { e })?;
        self.libary.set(libary_address);
        self.gallery_c.set(gallery);
        self.minter.set(minter);
        Ok(())
    }
}

// helper functions

// helper functions
impl NftStorage {
    // this checks if the user has a ticket to submit a nft
    pub fn c_tik(&self, gallery_index: U256) -> bool {
        let user = msg::sender();
        let address = self.gallery_c.get();
        let gallery_contract = ISubject::new(address);
        let config = Call::new();
        gallery_contract.get_user_status(config, gallery_index, user).expect("drat")
    }

    // this is the lock that lockes a user as the admin of this contract; there by making sure that it can only be called once
    pub fn check_admin(&mut self) -> Result<(), SubmitError> {
        let default_x = Address::from([0x00; 20]);
        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                SubmitError::InvalidParameter(InvalidParameter {
                    point: 17,
                })
            );
        }
        if self.admin.get() == default_x {
            self.admin.set(msg::sender());
        }
        Ok(())
    }

    // this is to update the main nft libary and create the identification of the nft
    pub fn pass_data(&mut self, gallery_id: U256, nft_data: U256) -> Result<(), SubmitError> {
        // Create a new instance of the  interface
        let meta_date_contract = IMainx::new(*self.libary);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        meta_date_contract.submit_nft(config, gallery_id, msg::sender(), nft_data).map_err(|_e| {
            SubmitError::InvalidParameter(InvalidParameter {
                point: 11,
            })
        })
    }
}
