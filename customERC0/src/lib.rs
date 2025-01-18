#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloc::string::String;
use alloy_primitives::{ Address, U256 };
use alloy_sol_types::sol;
use core::marker::PhantomData;
use stylus_sdk::{ evm, msg, prelude::*, call::{ call, Call } };

pub trait Erc20Params {
    /// Immutable token name
    const NAME: &'static str;

    /// Immutable token symbol
    const SYMBOL: &'static str;

    /// Immutable token decimals
    const DECIMALS: u8;
}

/// Immutable definitions
struct NovaParams;
impl Erc20Params for NovaParams {
    const NAME: &'static str = "Nova Vault";
    const SYMBOL: &'static str = "NovaV";
    const DECIMALS: u8 = 10;
}

sol_storage! {
    /// Erc20 implements all ERC-20 methods.
    #[entrypoint]
    pub struct Erc20 {
        /// Maps users to balances
        mapping(address => uint256) balances;
        /// Maps users to a mapping of each spender's allowance
        mapping(address => mapping(address => uint256)) allowances;
        /// The total supply of the token
        uint256 total_supply;
        /// Used to allow [`Erc20Params`]
        PhantomData<NovaParams> phantom;

        address admin;
        mapping(address => bool) allow_admin;
        uint256 price; // price of the tokens to ethers
        uint256 market // tokens avialable for sales

    }
}

// Declare events and Solidity error types
sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event TokenSold(address indexed buyer, uint256 indexed price, uint256 indexed value, uint256 market);
    event SetMarket(uint256 indexed old_market, uint256 indexed new_market);
    event SetPrice(uint256 indexed old_price, uint256 indexed new_price);

    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);
    error InvalidParameter(uint8 point);
    error InsufficientFunds(address from, uint256 value, uint256 price);
    error MarketExceeded(address from, uint256 market, uint256 amount);
    error Unauthorized(uint8 point);

}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum Erc20Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
    InvalidParameter(InvalidParameter),
    InsufficientFunds(InsufficientFunds),
    MarketExceeded(MarketExceeded),
    Unauthorized(Unauthorized),
}

// These methods are external to other contracts
// Note: modifying storage will become much prettier soon
#[public]
impl Erc20 {
    /// Immutable token name
    pub fn name() -> String {
        NovaParams::NAME.into()
    }

    /// Immutable token symbol
    pub fn symbol() -> String {
        NovaParams::SYMBOL.into()
    }

    /// Immutable token decimals
    pub fn decimals() -> u8 {
        NovaParams::DECIMALS
    }

    /// Total supply of tokens
    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    /// Balance of `address`
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(owner)
    }

    /// Transfers `value` tokens from msg::sender() to `to`
    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
        self._transfer(msg::sender(), to, value)?;
        Ok(true)
    }

    /// Transfers `value` tokens from `from` to `to`
    /// (msg::sender() must be able to spend at least `value` tokens from `from`)
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256
    ) -> Result<bool, Erc20Error> {
        // Check msg::sender() allowance
        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(msg::sender());
        let old_allowance = allowance.get();
        if old_allowance < value {
            return Err(
                Erc20Error::InsufficientAllowance(InsufficientAllowance {
                    owner: from,
                    spender: msg::sender(),
                    have: old_allowance,
                    want: value,
                })
            );
        }

        // Decreases allowance
        allowance.set(old_allowance - value);

        // Calls the internal transfer function
        self._transfer(from, to, value)?;

        Ok(true)
    }

    /// Approves the spenditure of `value` tokens of msg::sender() to `spender`
    pub fn approve(&mut self, spender: Address, value: U256) -> bool {
        self.allowances.setter(msg::sender()).insert(spender, value);
        evm::log(Approval {
            owner: msg::sender(),
            spender,
            value,
        });
        true
    }

    /// Returns the allowance of `spender` on `owner`'s tokens
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.getter(owner).get(spender)
    }

    pub fn set_mint(&mut self, users: Vec<Address>, state: Vec<bool>) -> Result<(), Erc20Error> {
        self.check_admin()?;

        // Update the state for each user
        for (ad, bo) in users.into_iter().zip(state.into_iter()) {
            let mut position = self.allow_admin.setter(ad);
            position.set(bo);
        }

        Ok(())
    }

    pub fn set_token_price(&mut self, price: U256) -> Result<(), Erc20Error> {
        self.check_admin()?;
        let old_price = self.price.get();
        self.price.set(price);
        evm::log(SetPrice {
            old_price,
            new_price: price,
        });
        Ok(())
    }

    pub fn set_market(&mut self, market: U256) -> Result<(), Erc20Error> {
        self.check_admin()?;
        let old_market = self.market.get();
        self.market.set(market);
        evm::log(SetMarket {
            old_market,
            new_market: market,
        });
        Ok(())
    }

    pub fn get_market(&self) -> U256 {
        self.market.get()
    }

    pub fn get_price(&self) -> U256 {
        self.price.get()
    }

    #[payable]
    pub fn BUY(&mut self) -> Result<(), Erc20Error> {
        let value = msg::value();
        let price = self.price.get();
        let admin = self.admin.get();
        if price > value {
            return Err(
                Erc20Error::InsufficientFunds(InsufficientFunds {
                    from: msg::sender(),
                    value,
                    price,
                })
            );
        }

        let amount = value / price;
        let market = self.market.get();
        if amount > market {
            return Err(
                Erc20Error::MarketExceeded(MarketExceeded {
                    from: msg::sender(),
                    market,
                    amount,
                })
            );
        }

        self.market.set(market - amount);

        let _ = self._mint(msg::sender(), amount)?;

        let _ = call(Call::new_in(self).value(value), admin, &[]).map_err(|_e| {
            Erc20Error::InvalidParameter(InvalidParameter {
                point: 11,
            })
        });

        evm::log(TokenSold {
            buyer: msg::sender(),
            price,
            value,
            market,
        });

        Ok(())
    }

    pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
        let user_state = self.allow_admin.getter(msg::sender()).get();
        if !user_state {
            return Err(
                Erc20Error::Unauthorized(Unauthorized {
                    point: 0,
                })
            );
        }
        self._mint(msg::sender(), value)?;
        Ok(())
    }

    /// Mints tokens to another address
    pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), Erc20Error> {
        let user_state = self.allow_admin.getter(msg::sender()).get();
        if !user_state {
            return Err(
                Erc20Error::Unauthorized(Unauthorized {
                    point: 1,
                })
            );
        }
        self._mint(to, value)?;
        Ok(())
    }

    /// Burns tokens
    pub fn burn(&mut self, value: U256) -> Result<(), Erc20Error> {
        self._burn(msg::sender(), value)?;
        Ok(())
    }
}

// These methods aren't exposed to other contracts
// Methods marked as "pub" here are usable outside of the erc20 module (i.e. they're callable from lib.rs)
// Note: modifying storage will become much prettier soon
impl Erc20 {
    /// Movement of funds between 2 accounts
    /// (invoked by the external transfer() and transfer_from() functions )
    pub fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), Erc20Error> {
        // Decreasing sender balance
        let mut sender_balance = self.balances.setter(from);
        let old_sender_balance = sender_balance.get();
        if old_sender_balance < value {
            return Err(
                Erc20Error::InsufficientBalance(InsufficientBalance {
                    from,
                    have: old_sender_balance,
                    want: value,
                })
            );
        }
        sender_balance.set(old_sender_balance - value);

        // Increasing receiver balance
        let mut to_balance = self.balances.setter(to);
        let new_to_balance = to_balance.get() + value;
        to_balance.set(new_to_balance);

        // Emitting the transfer event
        evm::log(Transfer { from, to, value });
        Ok(())
    }

    /// Mints `value` tokens to `address`
    pub fn _mint(&mut self, address: Address, value: U256) -> Result<(), Erc20Error> {
        // Increasing balance
        let mut balance = self.balances.setter(address);
        let new_balance = balance.get() + value;
        balance.set(new_balance);

        // Increasing total supply
        self.total_supply.set(self.total_supply.get() + value);

        // Emitting the transfer event
        evm::log(Transfer {
            from: Address::ZERO,
            to: address,
            value,
        });

        Ok(())
    }

    /// Burns `value` tokens from `address`
    pub fn _burn(&mut self, address: Address, value: U256) -> Result<(), Erc20Error> {
        // Decreasing balance
        let mut balance = self.balances.setter(address);
        let old_balance = balance.get();
        if old_balance < value {
            return Err(
                Erc20Error::InsufficientBalance(InsufficientBalance {
                    from: address,
                    have: old_balance,
                    want: value,
                })
            );
        }
        balance.set(old_balance - value);

        // Decreasing the total supply
        self.total_supply.set(self.total_supply.get() - value);

        // Emitting the transfer event
        evm::log(Transfer {
            from: address,
            to: Address::ZERO,
            value,
        });

        Ok(())
    }

    // this is the lock that lockes a user as the admin of this contract; there by making sure that it can only be called once
    pub fn check_admin(&mut self) -> Result<(), Erc20Error> {
        let default_x = Address::from([0x00; 20]);
        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                Erc20Error::InvalidParameter(InvalidParameter {
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
