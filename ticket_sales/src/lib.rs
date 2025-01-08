#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U256 };
use stylus_sdk::{ prelude::*, msg, evm };
use alloy_sol_types::sol;

use stylus_sdk::call::Call;

sol_storage! {
    #[entrypoint]
    pub struct Buy {
        address gallery_c;
        address erc20;
        address admin;
    }
}

sol_interface! {
    interface IErc20 {
        function transferFrom(address from, address to, uint256 value) external returns (bool);
    }

    interface ISubject {
        function getGallery(uint256 gallery_index) external view returns (address, string memory, string memory, uint32, uint64, uint256, uint64, uint64, uint256);
        function buyTicket(uint256 gallery_index, address user) external;
        function getLastIndex() external view returns (uint256);
        function getUserStatus(uint256 gallery_index, address user) external view returns (bool);
    }    
}

//  ======   Errors and Events  =========  //

sol! {
    // event to show that a ticket has been bought
    event BoughtTicket(address indexed buyer, uint256  gallery_index, uint256 price);

    event SoldTicket(address indexed seller, uint256 gallery_index, uint256 price);

    // my error
    // error to show invalid parameter
    error InvalidParameter(uint8 point);

    error NoData();

    error ExistingTicket(uint256 gallery_index);
    error InSufficientAllowance(uint256 gallery_index);
}

#[derive(SolidityError)]
pub enum TicketError {
    InvalidParameter(InvalidParameter),
    ExistingTicket(ExistingTicket),
    InSufficientAllowance(InSufficientAllowance),
    NoData(NoData),
}

#[public]
impl Buy {
    pub fn buy_ticket(&mut self, gallery_index: U256) -> Result<(), TicketError> {
        // check if the gallery index is valid
        if !self.i_chk(gallery_index) {
            return Err(
                TicketError::InvalidParameter(InvalidParameter {
                    point: 1,
                })
            );
        }

        // check if user has a ticket
        if self.c_tik(gallery_index) {
            return Err(TicketError::ExistingTicket(ExistingTicket { gallery_index }));
        }

        // getting gallary info
        let (creator, price) = match self.get_gal_info(gallery_index) {
            Ok((creator, price)) => (creator, price),
            Err(_) => {
                return Err(TicketError::NoData(NoData {}));
            }
        };
        if price != U256::from(0){
        // Pay for the ticket and propagate errors
        self
            .fund_tf(creator, price)
            .map_err(|_| {
                TicketError::InSufficientAllowance(InSufficientAllowance { gallery_index })
            })?;
        }
        // set data in the gallery
        self.up_tik(gallery_index);

        // this will send an event that the user has bought the ticket '
        evm::log(BoughtTicket {
            buyer: msg::sender();
            gallery_index,
            price,
        });

        // this event to to show the user that a ticket has been sold 
        evm::log(SoldTicket {
            seller: creator;
            gallery_index,
            price,
        });


        Ok(())
    }

    pub fn set_erc20_gallery(
        &mut self,
        er20_address: Address,
        gallery_address: Address
    ) -> Result<(), TicketError> {
        self.check_admin().map_err(|e| { e })?;
        self.erc20.set(er20_address);
        self.gallery_c.set(gallery_address);
        Ok(())
    }
}

// helper functions

impl Buy {
    // function to check if the user has a ticket
    pub fn c_tik(&self, gallery_index: U256) -> bool {
        let user = msg::sender();
        let address = self.gallery_c.get();
        let gallery_contract = ISubject::new(address);
        let config = Call::new();
        gallery_contract.get_user_status(config, gallery_index, user).expect("drat")
    }

    // function to check if the index given is correct
    pub fn i_chk(&self, gallery_index: U256) -> bool {
        let address = self.gallery_c.get();
        let gallery_contract = ISubject::new(address);
        let config = Call::new();
        let lst = gallery_contract.get_last_index(config).expect("drat");

        if gallery_index > lst {
            return false;
        }

        true
    }

    // This function handles the transfer of funds
    pub fn fund_tf(&mut self, gallery_creator: Address, price: U256) -> Result<bool, u8> {
        // Create a new instance of the ERC-20 interface
        let meta_date_contract = IErc20::new(*self.erc20);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        meta_date_contract
            .transfer_from(config, msg::sender(), gallery_creator, price)
            .map_err(|_e| 0 as u8)
    }

    // this function gets the needed gallery info for the buying of ticket to be succesful
    // here we are returning the (creator_address and price of the ticket)
    pub fn get_gal_info(&self, gallery_index: U256) -> Result<(Address, U256), ()> {
        let address = self.gallery_c.get();
        let gallery_contract = ISubject::new(address);
        let config = Call::new();
        let data = gallery_contract.get_gallery(config, gallery_index).expect("drat");
        Ok((data.0, data.5))
    }

    // this function will be incharge of updating the status of the buying of ticket

    pub fn up_tik(&mut self, gallery_index: U256) {
        // update gallary information
        let meta_date_contract = ISubject::new(*self.gallery_c);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        let _ = meta_date_contract
            .buy_ticket(config, gallery_index, msg::sender())
            .map_err(|_e| 0 as u8);
    }

    pub fn check_admin(&mut self) -> Result<bool, TicketError> {
        let default_x = Address::from([0x00; 20]);
        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                TicketError::InvalidParameter(InvalidParameter {
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
}
