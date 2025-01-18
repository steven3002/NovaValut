#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U32, U256 };
use stylus_sdk::{ prelude::*, block, contract, msg };
use stylus_sdk::call::Call;
use alloy_sol_types::sol;

const AIRDROP: u32 = 200_000;

// this will be the factor of reduction of the numbers of tokewns a user can get
const SUBX: u32 = 32;

sol_storage! {
    #[entrypoint]
    pub struct Users {
        mapping(address => User) users; // maps the user to their digital identity 
        address erc20; // holds the smart contract address of the erc20;
        address admin; // holds the one time admin wallet address; to make sure only the admin can change the contract address state
        uint256 registerd_users; //holds the numbers of registered users
    }

    // users can store their name and a set of profile data
    pub struct User{
        bool has_registered; // used to confirm that the user has or has not registered{mostly used for the airdrop}
        string name; // name of a user 
        Profile profile; // struct holding the user bio and meta_data
    }


    pub struct Profile{
        string bio;

        // string intrests;
        // this will be json that will hold the socials of the individual
        // string socials;
        // this will contain the banner and the profile image
        //one could store the index of an nft to save as their profile picture
        string meta_data;

        uint32 joined_at;  // stores the time that the user registerd for the platform
        uint32 last_updated; // stores teh last time the user updated their personal info
    }

}

sol_interface! {
    // interface that allows this contract to perfrom minting to users wallet
    interface IErc20 {
        function mintTo(address to, uint256 value) external;
    }
}
sol! {

    error AlreadyRegistered(uint8 point );
    error EmptyField(uint8 point);
    error NotRegistered(uint8 point);
    error InvalidParameter(uint8 point);
}

#[derive(SolidityError)]
pub enum RegError {
    AlreadyRegistered(AlreadyRegistered),
    EmptyField(EmptyField),
    NotRegistered(NotRegistered),
    InvalidParameter(InvalidParameter),
}

#[public]
impl Users {
    pub fn register_user(
        &mut self,
        name: String,
        bio: String,
        // socials: String,
        meta_data: String
    ) -> Result<(), RegError> {
        let user_state = self.has_registered(msg::sender());
        if
            name.is_empty() ||
            bio.is_empty() ||
            // interests.is_empty() ||
            // socials.is_empty() ||
            meta_data.is_empty()
        {
            return Err(
                RegError::EmptyField(EmptyField {
                    point: 0,
                })
            );
        }

        let mut user = self.users.setter(msg::sender());
        user.name.set_str(name);
        user.has_registered.set(true);

        user.profile.bio.set_str(bio);
        // user.profile.socials.set_str(socials);
        user.profile.meta_data.set_str(meta_data);
        user.profile.last_updated.set(U32::from(block::timestamp()));

        // so instade of creating another function that will change  the data
        // we will just call this function to register and change data
        if !user_state {
            user.profile.joined_at.set(U32::from(block::timestamp()));

            //this will decrease the numbers of tokens a user will get
            let registerd_users = self.registerd_users.get();
            let token = self.re_f();
            // Give new registrants some tokens
            if token >= U256::from(53) {
                self.mint_tkn(token, msg::sender());
            }
            self.registerd_users.set(registerd_users + U256::from(1));
        }

        Ok(())
    }

    pub fn set_erc20(&mut self, er20_address: Address) -> Result<(), RegError> {
        self.check_admin()?;
        self.erc20.set(er20_address);
        Ok(())
    }

    // ===== view functions ==== //
    // this function is to get the data of a particular user
    pub fn get_user_info(
        &self,
        user_address: Address
    ) -> Result<([String; 3], [u32; 2]), RegError> {
        if !self.has_registered(user_address) {
            return Err(
                RegError::NotRegistered(NotRegistered {
                    point: 0,
                })
            );
        }

        let user = self.users.getter(user_address);
        let data = (
            [
                user.name.get_string(),
                user.profile.bio.get_string(),
                // user.profile.socials.get_string(),
                user.profile.meta_data.get_string(),
            ],
            [user.profile.joined_at.get().to::<u32>(), user.profile.last_updated.get().to::<u32>()],
        );
        Ok(data)
    }

    pub fn has_registered(&self, user_address: Address) -> bool {
        self.users.get(user_address).has_registered.get()
    }
}

impl Users {
    pub fn mint_tkn(&mut self, tkn: U256, address: Address) {
        let meta_date_contract = IErc20::new(*self.erc20);
        let config = Call::new_in(self);
        let _ = meta_date_contract
            .mint_to(config, address, tkn)
            .expect("Failed to call on MetaDate_contract");
    }

    pub fn check_admin(&mut self) -> Result<bool, RegError> {
        let default_x = Address::from([0x00; 20]);
        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                RegError::InvalidParameter(InvalidParameter {
                    point: 9,
                })
            );
        } else if self.admin.get() == default_x {
            self.admin.set(msg::sender());
            return Ok(true);
        } else {
            return Ok(true);
        }
    }

    // this is the reduction factor
    pub fn re_f(&self) -> U256 {
        let x = U256::from(SUBX) * self.registerd_users.get();
        U256::from(AIRDROP) - x
    }
}
