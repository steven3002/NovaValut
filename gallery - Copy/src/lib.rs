#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U256 };
use stylus_sdk::{ prelude::*, msg, block };
use alloy_sol_types::sol;

use stylus_sdk::call::Call;

sol_storage! {
    #[entrypoint]
    pub struct Cast {
        address stake;
        address admin;
        address nft_libary;
        address erc20;
        address gallery_c;
    }
}

sol_interface! {
    interface IErc20 {
        function transferFrom(address from, address to, uint256 value) external returns (bool);
    }
    interface ISubject {
        function getGallery(uint256 gallery_index) external view returns (address, string memory, string memory, uint32, uint64, uint256, uint64, uint64, uint256);
        function getUserStatus(uint256 gallery_index, address user) external view returns (bool);
    }    

    interface IStake {
        function stake(address user, uint256 gallery_id, uint256 nft_id, uint256 bid) external;
        function updateBid(address user, uint256 gallery_id, uint256 nft_id, uint256 vote_id, uint256 bid) external;
        function hasVoted(uint256 gallery_id, address user) external view returns (bool);
        function getCast(uint256 gallery_id, uint256 nft_id, uint256 vote_id) external view returns (uint256, uint32, address);
    }

    interface IMainx {
        function getNft(uint256 gallery_index, uint256 nft_id, bool raw) external view returns (address, uint8, uint256);
    }
}

sol! {
    
        error InvalidParameter(uint8 point);
    
        
}

#[derive(SolidityError)]
pub enum CastError {
    InvalidParameter(InvalidParameter),
}

#[public]
impl Cast {
    pub fn cast_bid(&mut self, gallery_id: U256, nft_id: U256, bid: U256) -> Result<(), CastError> {
        // getting gallary info
        let (start, end, minimum_bid) = match self.get_gal_info(gallery_id) {
            Ok((start, end, minimum_bid)) => (start, end, minimum_bid),
            Err(_) => {
                return Err(
                    CastError::InvalidParameter(InvalidParameter {
                        point: 202,
                    })
                );
            }
        };

        if
            minimum_bid > bid ||
            !self.c_tik(gallery_id, msg::sender()) ||
            self.has_voted(gallery_id)
        {
            return Err(
                CastError::InvalidParameter(InvalidParameter {
                    point: 20,
                })
            );
        }

        self.check_time(start, end)?;
        let nft_creator = self.get_creator(gallery_id, nft_id)?;

        self.fund_tf(nft_creator, bid)?;
        self.stake(gallery_id, nft_id, bid)?;
        Ok(())
    }

    pub fn increase_cast(
        &mut self,
        gallery_id: U256,
        nft_id: U256,
        vote_id: U256,
        bid: U256
    ) -> Result<(), CastError> {
        let (start, end, _minimum_bid) = match self.get_gal_info(gallery_id) {
            Ok((start, end, minimum_bid)) => (start, end, minimum_bid),
            Err(_) => {
                return Err(
                    CastError::InvalidParameter(InvalidParameter {
                        point: 202,
                    })
                );
            }
        };

        self.check_time(start, end)?;
        let nft_creator = self.get_creator(gallery_id, nft_id)?;

        // get data of the vote made
        let old_bid = self.get_staking_data(gallery_id, nft_id, vote_id)?;

        if old_bid >= bid {
            return Err(
                CastError::InvalidParameter(InvalidParameter {
                    point: 2,
                })
            );
        }

        let balance_bid = bid - old_bid;

        self.fund_tf(nft_creator, balance_bid)?;
        self.update_bid(gallery_id, nft_id, bid, vote_id)?;

        Ok(())
    }

    pub fn set_control(
        &mut self,
        stake: Address,
        erc20: Address,
        gallery: Address
    ) -> Result<(), CastError> {
        self.check_admin().map_err(|e| { e })?;
        self.stake.set(stake);
        self.gallery_c.set(gallery);
        self.erc20.set(erc20);
        Ok(())
    }
}

impl Cast {
    // info returns (start, end, minimum_bid)
    pub fn get_gal_info(&self, gallery_index: U256) -> Result<(u64, u64, U256), ()> {
        let address = self.gallery_c.get();
        let gallery_contract = ISubject::new(address);
        let config = Call::new();
        let data = gallery_contract.get_gallery(config, gallery_index).expect("drat");
        Ok((data.7, data.6, data.8))
    }

    // function to check if the user has a ticket
    pub fn c_tik(&self, gallery_index: U256, user: Address) -> bool {
        let address = self.gallery_c.get();
        let gallery_contract = ISubject::new(address);
        let config = Call::new();
        gallery_contract.get_user_status(config, gallery_index, user).expect("drat")
    }

    pub fn check_time(&self, start: u64, end: u64) -> Result<(), CastError> {
        let current_time: u64 = block::timestamp() as u64;
        if start < current_time && end > current_time {
            return Ok(());
        }
        Err(
            CastError::InvalidParameter(InvalidParameter {
                point: 19,
            })
        )
    }

    pub fn stake(&mut self, gallery_id: U256, nft_id: U256, bid: U256) -> Result<(), CastError> {
        // Create a new instance of the ERC-20 interface
        let meta_date_contract = IStake::new(*self.stake);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        meta_date_contract.stake(config, msg::sender(), gallery_id, nft_id, bid).map_err(|_e| {
            CastError::InvalidParameter(InvalidParameter {
                point: 11,
            })
        })
    }

    pub fn update_bid(
        &mut self,
        gallery_id: U256,
        nft_id: U256,
        bid: U256,
        vote_id: U256
    ) -> Result<(), CastError> {
        // Create a new instance of the stake interface
        let meta_date_contract = IStake::new(*self.stake);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        meta_date_contract
            .update_bid(config, msg::sender(), gallery_id, nft_id, vote_id, bid)
            .map_err(|_e| {
                CastError::InvalidParameter(InvalidParameter {
                    point: 201,
                })
            })
    }

    pub fn has_voted(&self, gallery_id: U256) -> bool {
        let address = self.gallery_c.get();
        let gallery_contract = IStake::new(address);
        let config = Call::new();
        gallery_contract.has_voted(config, gallery_id, msg::sender()).expect("drat")
    }

    // This function handles the transfer of funds
    pub fn fund_tf(&mut self, nft_creator: Address, price: U256) -> Result<bool, CastError> {
        // Create a new instance of the ERC-20 interface
        let meta_date_contract = IErc20::new(*self.erc20);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        meta_date_contract.transfer_from(config, msg::sender(), nft_creator, price).map_err(|_e| {
            CastError::InvalidParameter(InvalidParameter {
                point: 201,
            })
        })
    }

    pub fn check_admin(&mut self) -> Result<bool, CastError> {
        let default_x: Address = Address::from([0x00; 20]);

        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                CastError::InvalidParameter(InvalidParameter {
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

    pub fn get_creator(&self, gallery_id: U256, nft_id: U256) -> Result<Address, CastError> {
        let address = self.nft_libary.get();
        let gallery_contract = IMainx::new(address);
        let config = Call::new();
        let default_x = Address::from([0x00; 20]);

        // Use a match to handle the result of `get_nft`
        match gallery_contract.get_nft(config, gallery_id, nft_id, false) {
            Ok((creator, _, _)) if creator != default_x => Ok(creator),
            Ok(_) => Err(CastError::InvalidParameter(InvalidParameter { point: 191 })),
            Err(_) => Err(CastError::InvalidParameter(InvalidParameter { point: 181 })),
        }
    }

    pub fn get_staking_data(
        &self,
        gallery_id: U256,
        nft_id: U256,
        vote_id: U256
    ) -> Result<U256, CastError> {
        let address = self.stake.get();
        let stake_contract = IStake::new(address);
        let config = Call::new();
        let data = stake_contract.get_cast(config, gallery_id, nft_id, vote_id).expect("drat");
        if data.2 != msg::sender() {
            return Err(CastError::InvalidParameter(InvalidParameter { point: 81 }));
        }
        Ok(data.0)
    }
}
