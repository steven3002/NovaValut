#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U256 };
use stylus_sdk::{ prelude::*, msg, block };
use alloy_sol_types::sol;

use stylus_sdk::call::Call;

// this contract is the safe contract allowed to take the user votes
sol_storage! {
    #[entrypoint]
    pub struct Minter {
        mapping(address => mapping(uint256 => bool)) has_minted;
        address stake; // stores the contract address of the unsafe stake contract
        address admin; //stores the admin contract address
        address nft_libary; // stores the nft_libary contract address
        address nft_storage; // stores the nft_submit contract address
        address gallery_c; // stores the gallery contract address
        address erc1155; // contract address of the token contract
    }
}

sol_interface! {
    // interface of the token contract
    interface IErc20 {
        function transferFrom(address from, address to, uint256 value) external returns (bool);
    }

    // interface of the gallery contract
    interface ISubject {
        function getGallery(uint256 gallery_index) external view returns (address, string memory, string memory, uint32, uint64, uint256, uint64, uint64, uint256);
  
    }    

   

    //  interface of the nft_libary contract
    interface IMainx {
        function getNft(uint256 gallery_index, uint256 nft_id, bool raw) external view returns (address, uint8, uint256);
    }

    //interface of the nft_submit contract

    interface INftStorage {
        function systemMint(uint256 nft_id) external;
    }

    //interface of the erc1155 contract 
    interface IErc1155 {
        function mint(address to, uint256 id, uint256 amount, uint8[] memory data) external;
        function setData(uint256 id, uint256 g_id, uint256 s_n_id, uint256 m_d_id) external;
    }

     // interface of the unsafe stake contract
    interface IStake {
        function getPosition(uint256 gallery_id, uint256 nft_id, address user) external view returns (uint8);
    }
}

sol! {

        error InvalidParameter(uint8 point);
           
}

#[derive(SolidityError)]
pub enum MinterError {
    InvalidParameter(InvalidParameter),
}

#[public]
impl Minter {
    // allows users to cast a safe vote
    pub fn claim_SFT(&mut self, gallery_id: U256, nft_id: U256) -> Result<(), MinterError> {
        // get the gallery info
        let end = match self.get_gal_info(gallery_id) {
            Ok(end) => end,
            Err(_) => {
                return Err(
                    MinterError::InvalidParameter(InvalidParameter {
                        point: 202,
                    })
                );
            }
        };

        self.check_time(end)?; //makes sure voting period has ended

        if self.has_minted.getter(msg::sender()).getter(gallery_id).get() {
            return Err(
                MinterError::InvalidParameter(InvalidParameter {
                    point: 202,
                })
            );
        }

        // check their position in the leaderboard
        let position = self.get_position(gallery_id, nft_id)?;
        let amount = match position {
            0 => 3,
            1 => 2,
            2 => 1,
            _ => {
                return Err(
                    MinterError::InvalidParameter(InvalidParameter {
                        point: 202,
                    })
                );
            }
        };
        let nft_storage_id = self.get_nft(gallery_id, nft_id)?;
        self.set_libary(nft_storage_id)?;

        self.mint(nft_storage_id, U256::from(amount))?;
        self.set_data(nft_storage_id, gallery_id, nft_id)?;

        let mut minting_state = self.has_minted.setter(msg::sender());
        let mut m_s_h = minting_state.setter(gallery_id);
        m_s_h.set(true);
        Ok(())
    }

    // safe function to set the admin and the needed contract address
    pub fn set_control(
        &mut self,
        stake: Address, // unsafe stake contract address
        gallery: Address, // gallery address
        libary: Address, //nft_ libary contract address
        erc1155: Address,
        nft_storage: Address
    ) -> Result<(), MinterError> {
        self.check_admin().map_err(|e| { e })?; // makes sure only the admin can call this function
        self.stake.set(stake);
        self.gallery_c.set(gallery);
        self.nft_libary.set(libary);
        self.erc1155.set(erc1155);
        self.nft_storage.set(nft_storage);
        Ok(())
    }

    pub fn has_claimed(&self, gallery_id: U256) -> bool {
        self.has_minted.getter(msg::sender()).getter(gallery_id).get()
    }
}

impl Minter {
    // info returns end
    // get the gallery infromation
    pub fn get_gal_info(&self, gallery_index: U256) -> Result<u64, ()> {
        let address = self.gallery_c.get();
        let gallery_contract = ISubject::new(address);
        let config = Call::new();
        let data = gallery_contract.get_gallery(config, gallery_index).expect("drat");
        Ok(data.6)
    }

    // function to make sure voting period has ended
    pub fn check_time(&self, end: u64) -> Result<(), MinterError> {
        let current_time: u64 = block::timestamp() as u64;
        if end < current_time {
            return Ok(());
        }
        Err(
            MinterError::InvalidParameter(InvalidParameter {
                point: 19,
            })
        )
    }

    // control the admin state
    pub fn check_admin(&mut self) -> Result<bool, MinterError> {
        let default_x: Address = Address::from([0x00; 20]);

        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                MinterError::InvalidParameter(InvalidParameter {
                    point: 0,
                })
            );
        } else if self.admin.get() == default_x {
            self.admin.set(msg::sender());
            return Ok(true);
        } else {
            return Ok(true);
        }
    }

    // get the nft index of the nft_submit
    pub fn get_nft(&self, gallery_id: U256, nft_id: U256) -> Result<U256, MinterError> {
        let address = self.nft_libary.get();
        let gallery_contract = IMainx::new(address);
        let config = Call::new();

        // Use a match to handle the result of `get_nft`
        match gallery_contract.get_nft(config, gallery_id, nft_id, false) {
            Ok((_, _, data_id)) => Ok(data_id),
            // Ok(_) => Err(CastError::InvalidParameter(InvalidParameter { point: 191 })),
            Err(_) => Err(MinterError::InvalidParameter(InvalidParameter { point: 181 })),
        }
    }

    // check position on the leaderboard
    pub fn get_position(
        &self,
        gallery_id: U256, // gallery identity
        nft_id: U256 // nft identity
    ) -> Result<u8, MinterError> {
        let address = self.stake.get();
        let stake_contract = IStake::new(address);
        let config = Call::new();
        match stake_contract.get_position(config, gallery_id, nft_id, msg::sender()) {
            Ok(position) => Ok(position),
            Err(_) => Err(MinterError::InvalidParameter(InvalidParameter { point: 81 })),
        }
    }
    // this function will call the nft_submit contract and will set the metada to open
    pub fn set_libary(&mut self, s_nft_id: U256) -> Result<(), MinterError> {
        // this is to update the main nft libary and create the identification of the nft

        let meta_date_contract = INftStorage::new(*self.nft_storage);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        meta_date_contract.system_mint(config, s_nft_id).map_err(|_e| {
            MinterError::InvalidParameter(InvalidParameter {
                point: 11,
            })
        })
    }

    pub fn mint(&mut self, new_nft_id: U256, amount: U256) -> Result<(), MinterError> {
        // this is to update the main nft libary and create the identification of the nft

        let meta_date_contract = IErc1155::new(*self.erc1155);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        meta_date_contract.mint(config, msg::sender(), new_nft_id, amount, vec![0]).map_err(|_e| {
            MinterError::InvalidParameter(InvalidParameter {
                point: 11,
            })
        })
    }

    pub fn set_data(
        &mut self,
        token_id: U256,
        gallery_id: U256,
        system_nft_id: U256
    ) -> Result<(), MinterError> {
        let meta_date_contract = IErc1155::new(*self.erc1155);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        meta_date_contract
            .set_data(config, token_id, gallery_id, system_nft_id, token_id)
            .map_err(|_e| {
                MinterError::InvalidParameter(InvalidParameter {
                    point: 11,
                })
            })
    }
}
