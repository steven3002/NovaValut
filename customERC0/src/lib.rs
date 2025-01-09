#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

// Modules and imports
mod erc20;

use alloy_primitives::{ Address, U256 };
use stylus_sdk::{ msg, prelude::* };
use crate::erc20::{ Erc20, Erc20Params, Erc20Error };

/// Immutable definitions
struct GallyParams;
impl Erc20Params for GallyParams {
    const NAME: &'static str = "Nova Vault";
    const SYMBOL: &'static str = "NovaV";
    const DECIMALS: u8 = 10;
}

// Define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    struct StylusToken {
        // Allows erc20 to access StylusToken's storage and make calls
        #[borrow]
        Erc20<GallyParams> erc20;
    }
}

#[public]
#[inherit(Erc20<GallyParams>)]
impl StylusToken {
    /// Mints tokens
    pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(msg::sender(), value)?;
        Ok(())
    }

    /// Mints tokens to another address
    pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(to, value)?;
        Ok(())
    }

    /// Burns tokens
    pub fn burn(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.burn(msg::sender(), value)?;
        Ok(())
    }
}
