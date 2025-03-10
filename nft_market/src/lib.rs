// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{ U256, Address },
    prelude::*,
    msg,
    evm,
    block,
    call::Call,
    contract,
};

use alloy_sol_types::sol;

sol_storage! {
    #[entrypoint]
    pub struct Market {
        mapping(address => mapping(uint256 => Cost)) sales;  
        address admin;
        address erc1155;
        address erc20;

    }

    pub struct Cost{
        uint256 price;
        bool for_sale;
        uint256 amount;
    }
}

sol_interface! {
    interface IErc1155 {
        function balanceOf(address account, uint256 id) external view returns (uint256);
        function safeTransferFrom(address from, address to, uint256 id, uint256 amount, uint8[] memory data) external;
        function isApprovedForAll(address account, address operator) external view returns (bool);
    }

    
    interface IErc20 {
        function transferFrom(address from, address to, uint256 value) external returns (bool);
    }

    

}

//  ======   Errors and Events  =========  //

sol! {
    // event to show that a new gallary have been created
    event Sold(address indexed pre_owner, address indexed new_owner, uint256 indexed nft_id, uint64 time);

    
    // my error
    // error to show invalid parameter
    error InvalidParameter(uint8 point);

    error InSufficientBalance(uint256 nft_id);
    error InSufficientAllowance(uint256 cost);
    error NoAccess(uint256 nft_id);
}

#[derive(SolidityError)]
pub enum MarketError {
    InvalidParameter(InvalidParameter),
    InSufficientBalance(InSufficientBalance),
    InSufficientAllowance(InSufficientAllowance),
    NoAccess(NoAccess),
}

#[public]
impl Market {
    // nft_id => this is the id of the nft that the user wants to put for sale
    // amount => this is the amount of that nft that they want to offer
    // price => this is the price that each of the nft will go for
    // for_sale => this is to set the state of the offer <to put the offer on a sale or not>
    pub fn offer(
        &mut self,
        nft_id: U256,
        amount: U256,
        price: U256,
        for_sale: bool
    ) -> Result<(), MarketError> {
        // we check the balance to see if the user has enough balance
        // and to make sure that the amount is not more than what they have
        let balance = self.c_b(nft_id);
        if balance == U256::from(0) || amount > balance {
            return Err(
                MarketError::InSufficientBalance(InSufficientBalance {
                    nft_id,
                })
            );
        }

        let mut s_t = self.sales.setter(msg::sender());
        let mut cost = s_t.setter(nft_id);
        cost.price.set(price);
        cost.for_sale.set(for_sale);
        cost.amount.set(amount);
        Ok(())
    }

    // this is to get the information about a list of nft
    // check if the nft is for sales or not
    // returns Vec(price of nft, state of sales, amount avialable for the sales)

    pub fn get_cost_batch(
        &self,
        owners: Vec<Address>,
        nft_ids: Vec<U256>
    ) -> Vec<(U256, bool, U256)> {
        owners
            .iter()
            .zip(nft_ids.iter())
            .map(|(owner, nft_id)| {
                let l_t = self.sales.getter(*owner);
                let cost = l_t.getter(*nft_id);
                let price = cost.price.get();
                let state = cost.for_sale.get();
                let amount = cost.amount.get();
                (price, state, amount)
            })
            .collect()
    }

    pub fn buy(&mut self, owner: Address, nft_id: U256, amount: U256) -> Result<(), MarketError> {
        let results_ = self.get_cost_batch(vec![owner], vec![nft_id]);
        let (cost, state, amount_x) = results_[0];
        // confirming that the nft is open for sales
        // confirming that the user has enough balance
        if amount > amount_x || amount_x == U256::from(0) || !state {
            return Err(
                MarketError::InvalidParameter(InvalidParameter {
                    point: 1,
                })
            );
        }

        // checking if the seller has given this contract the permission to transfer the nft
        if !self.a_c(owner) {
            return Err(
                MarketError::NoAccess(NoAccess {
                    nft_id,
                })
            );
        }

        //transfer funds from the buyer to the seller

        self
            .fund_tf(owner, cost * amount)
            .map_err(|_| { MarketError::InSufficientAllowance(InSufficientAllowance { cost }) })?;

        // transfer nft from seller to buyer
        self
            .nft_tf(owner, nft_id, amount)
            .map_err(|_| { MarketError::InSufficientBalance(InSufficientBalance { nft_id }) })?;

        // setting the new state of the stored amount left for display
        let mut s_t = self.sales.setter(owner);
        let mut cost_x = s_t.setter(nft_id);
        cost_x.amount.set(amount_x - amount);

        // emmiting this transaction
        evm::log(Sold {
            pre_owner: owner,
            new_owner: msg::sender(),
            nft_id,
            time: block::timestamp() as u64,
        });

        Ok(())
    }

    // this function is used to check and set the erc1155 contract address by the admin
    pub fn set_erc1155(&mut self, erc1155: Address, erc20: Address) -> Result<(), MarketError> {
        self.check_admin().map_err(|e| { e })?;
        self.erc1155.set(erc1155);
        self.erc20.set(erc20);
        Ok(())
    }
}

impl Market {
    // this function is used to check if the user has the balance of nft that they want to offer for sale
    pub fn c_b(&self, nft_id: U256) -> U256 {
        let user = msg::sender();
        let address = self.erc1155.get();
        let erc1155_contract = IErc1155::new(address);
        let config = Call::new();
        erc1155_contract.balance_of(config, user, nft_id).expect("drat")
    }

    // this function is used to check if the is smart contract has been approved for a transaction
    pub fn a_c(&self, account: Address) -> bool {
        let address = self.erc1155.get();
        let erc1155_contract = IErc1155::new(address);
        let config = Call::new();
        // contract adderess
        let c_address = contract::address();
        erc1155_contract.is_approved_for_all(config, account, c_address).expect("drat")
    }

    // This function handles the transfer of funds
    pub fn fund_tf(&mut self, owner: Address, price: U256) -> Result<bool, u8> {
        // Create a new instance of the ERC-20 interface
        let meta_date_contract = IErc20::new(*self.erc20);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        meta_date_contract.transfer_from(config, msg::sender(), owner, price).map_err(|_e| 0 as u8)
    }

    // This function handles the transfer of nfts
    pub fn nft_tf(&mut self, from: Address, nft_id: U256, amount: U256) -> Result<(), u8> {
        // Create a new instance of the ERC-1155 interface
        let meta_date_contract = IErc1155::new(*self.erc1155);

        // Set up the call configuration
        let config = Call::new_in(self);

        // Attempt the transfer
        meta_date_contract
            .safe_transfer_from(config, from, msg::sender(), nft_id, amount, vec![0])
            .map_err(|_e| 0 as u8)
    }

    // this is the lock that lockes a user as the admin of this contract; there by making sure that it can only be called once
    pub fn check_admin(&mut self) -> Result<(), MarketError> {
        let default_x = Address::from([0x00; 20]);
        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                MarketError::InvalidParameter(InvalidParameter {
                    point: 17,
                })
            );
        }
        if self.admin.get() == default_x {
            self.admin.set(msg::sender());
        }
        Ok(())
    }
}
