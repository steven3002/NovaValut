#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U256 };
use stylus_sdk::{ prelude::*, msg, evm };
use alloy_sol_types::sol;
// use alloy_sol_types::*;

use stylus_sdk::call::Call;

sol_storage! {
    #[entrypoint]
    pub struct NftStorage{
        mapping(uint256 => Nft) data;
        uint256 available_index;
        address admin;
        address libary;
        address gallery_c;
    }
    pub struct Nft{
        uint256 gallery;
        string data;
        address owner;
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
    pub fn submit_nft(&mut self, gallery_id: U256, data: String) -> Result<(), SubmitError> {
        let available_index = self.available_index.get();
        self.pass_data(gallery_id, available_index).map_err(|e| { e })?;
        let mut data_state = self.data.setter(available_index);
        data_state.data.set_str(data);
        data_state.owner.set(msg::sender());
        data_state.gallery.set(gallery_id);

        self.available_index.set(available_index + U256::from(1));

        evm::log(SubmitNft {
            owner: msg::sender(),
            gallery_index: gallery_id,
            nft_id: available_index,
        });

        Ok(())
    }

    pub fn get_nft_data(&self, nft_id: U256) -> Result<(Address, String), SubmitError> {
        let data_x = self.data.getter(nft_id);
        if !self.c_tik(data_x.gallery.get()) {
            return Err(
                SubmitError::InvalidParameter(InvalidParameter {
                    point: 17,
                })
            );
        }
        Ok((data_x.owner.get(), data_x.data.get_string()))
    }

    pub fn set_libary(
        &mut self,
        libary_address: Address,
        gallery: Address
    ) -> Result<(), SubmitError> {
        self.check_admin().map_err(|e| { e })?;
        self.libary.set(libary_address);
        self.gallery_c.set(gallery);
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
    pub fn check_admin(&mut self) -> Result<bool, SubmitError> {
        let default_x = Address::from([0x00; 20]);
        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                SubmitError::InvalidParameter(InvalidParameter {
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

    // this is to update the main nft libary and create the identification of the nft
    pub fn pass_data(&mut self, gallery_id: U256, nft_data: U256) -> Result<(), SubmitError> {
        // Create a new instance of the ERC-20 interface
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
