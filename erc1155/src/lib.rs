#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U256, U32 };
use alloy_sol_types::sol;
use stylus_sdk::{ evm, msg, block, prelude::* };
use core::marker::PhantomData;

pub trait Erc1155Params {
    /// Immutable Collection name
    const COLLECTIONNAME: &'static str;
}

struct NovaParams;
impl Erc1155Params for NovaParams {
    const COLLECTIONNAME: &'static str = "Nova Vault NFTs and SFT";
}

// ERC-1155 Events
sol! {
    event TransferSingle(address indexed operator, address indexed from, address indexed to, uint256 id, uint256 value, uint32 time );
    event TransferBatch(address indexed operator, address indexed from, address indexed to, uint256[] ids, uint256[] values, uint32 time);
    event ApprovalForAll(address indexed account, address indexed operator, bool approved, uint32 time);
    event URI(string value, uint256 indexed id);

    error InvalidParameter(uint8 point);
}

sol_storage! {
    // ERC-1155 storage
    #[entrypoint]
    pub struct Erc1155 {
        // Token balances: token ID -> owner -> balance
        mapping(uint256 => mapping(address => uint256)) balances;
        // Approval map: owner -> operator -> approval status
        mapping(address => mapping(address => bool)) operator_approvals;

        // total_supply mapping: token ID -> totalsupply
        mapping(uint256 => uint256) total_supply;
        // mapping( uint256 => string) uris;

        mapping(uint256 => MetaData) meta_data;

        // allowed minting contract address
        address minter;
        address admin;

        PhantomData<NovaParams> phantom;
    }

    pub struct MetaData{
        uint256 gallery_id;
        uint256 system_nft_id;
        uint256 meta_data;
    }

}

#[derive(SolidityError)]
pub enum Erc1155Error {
    InvalidParameter(InvalidParameter),
}

#[public]
impl Erc1155 {
    // Returns the balance of `account` for token `id`.
    pub fn balance_of(&self, account: Address, id: U256) -> U256 {
        self.balances.getter(id).getter(account).get()
    }

    // Batch balance query
    pub fn balance_of_batch(&self, accounts: Vec<Address>, ids: Vec<U256>) -> Vec<U256> {
        accounts
            .iter()
            .zip(ids.iter())
            .map(|(&account, &id)| self.balance_of(account, id))
            .collect()
    }

    // Grants or revokes permission for `operator` to manage all tokens of `msg.sender`.
    pub fn set_approval_for_all(&mut self, operator: Address, approved: bool) {
        self.operator_approvals.setter(msg::sender()).insert(operator, approved);
        evm::log(ApprovalForAll {
            account: msg::sender(),
            operator,
            approved,
            time: block::timestamp() as u32,
        });
    }

    // Checks if `operator` is approved to manage tokens of `account`.
    pub fn is_approved_for_all(&self, account: Address, operator: Address) -> bool {
        self.operator_approvals.getter(account).get(operator)
    }

    // Transfers tokens.
    pub fn safe_transfer_from(
        &mut self,
        from: Address,
        to: Address,
        id: U256,
        amount: U256,
        data: Vec<u8>
    ) -> Result<(), Erc1155Error> {
        self._transfer_single(msg::sender(), from, to, id, amount)?;
        evm::log(TransferSingle {
            operator: msg::sender(),
            from,
            to,
            id,
            value: amount,
            time: block::timestamp() as u32,
        });

        Ok(())
    }

    // Batch transfer of tokens.
    pub fn safe_batch_transfer_from(
        &mut self,
        from: Address,
        to: Address,
        ids: Vec<U256>,
        amounts: Vec<U256>,
        data: Vec<u8>
    ) -> Result<(), Erc1155Error> {
        if ids.len() != amounts.len() {
            return Err(
                Erc1155Error::InvalidParameter(InvalidParameter {
                    point: 17,
                })
            );
        }
        for (id, amount) in ids.iter().zip(amounts.iter()) {
            self._transfer_single(msg::sender(), from, to, *id, *amount)?;
        }
        evm::log(TransferBatch {
            operator: msg::sender(),
            from,
            to,
            ids,
            values: amounts,
            time: block::timestamp() as u32,
        });

        Ok(())
    }

    // Mints a token.
    pub fn _mint(
        &mut self,
        to: Address,
        id: U256,
        amount: U256,
        data: Vec<u8>
    ) -> Result<(), Erc1155Error> {
        if msg::sender() != self.minter.get() {
            return Err(Erc1155Error::InvalidParameter(InvalidParameter { point: 2 }));
        }
        let balance_to = self.balances.getter(id).getter(to).get();

        let mut b_e = self.balances.setter(id);
        let mut balance = b_e.setter(to);
        balance.set(balance_to + amount);

        let old_supply = self.total_supply.getter(id).get();
        let mut new_supply = self.total_supply.setter(id);
        new_supply.set(old_supply + amount);

        evm::log(TransferSingle {
            operator: msg::sender(),
            from: Address::ZERO,
            to,
            id,
            value: amount,
            time: block::timestamp() as u32,
        });

        Ok(())
    }

    pub fn _mint_batch(
        &mut self,
        to: Address,
        ids: Vec<U256>,
        amounts: Vec<U256>,
        data: Vec<u8>
    ) -> Result<(), Erc1155Error> {
        if ids.len() != amounts.len() {
            return Err(Erc1155Error::InvalidParameter(InvalidParameter { point: 101 }));
        }

        if msg::sender() != self.minter.get() {
            return Err(Erc1155Error::InvalidParameter(InvalidParameter { point: 1 }));
        }

        for (id, amount) in ids.iter().zip(amounts.iter()) {
            let old_balance = self.balances.getter(*id).getter(to).get();
            let mut b_p = self.balances.setter(*id);
            let mut balance = b_p.setter(to);
            balance.set(old_balance + *amount);

            let old_supply = self.total_supply.getter(*id).get();
            let mut new_supply = self.total_supply.setter(*id);
            new_supply.set(old_supply + amount);
        }

        evm::log(TransferBatch {
            operator: msg::sender(),
            from: Address::ZERO,
            to,
            ids: ids.clone(),
            values: amounts.clone(),
            time: block::timestamp() as u32,
        });

        Ok(())
    }

    /// Total supply
    pub fn total_supply(&self, id: U256) -> Result<U256, Erc1155Error> {
        Ok(self.total_supply.getter(id).get())
    }

    pub fn set_minter(&mut self, minter: Address) -> Result<(), Erc1155Error> {
        self.check_admin().map_err(|e| { e })?;
        self.minter.set(minter);
        Ok(())
    }

    // pub fn uri(&self, id: U256) -> String {
    //     self.uris.getter(id).get_string()
    // }

    // pub fn _setURI(&mut self, id: U256, uri: String) {
    //     self.uris.setter(id).set_str(uri.clone());

    //     evm::log(URI {
    //         value: uri,
    //         id,
    //     })
    // }

    /// Immutable token name
    pub fn name() -> String {
        NovaParams::COLLECTIONNAME.into()
    }

    pub fn setData(
        &mut self,
        id: U256,
        g_id: U256,
        s_n_id: U256,
        m_d_id: U256
    ) -> Result<(), Erc1155Error> {
        if msg::sender() != self.minter.get() {
            return Err(Erc1155Error::InvalidParameter(InvalidParameter { point: 1 }));
        }
        let mut state = self.meta_data.setter(id);
        state.gallery_id.set(g_id);
        state.system_nft_id.set(s_n_id);
        state.meta_data.set(m_d_id);
        Ok(())
    }

    pub fn getData(&self, id: U256) -> Result<[U256; 3], Erc1155Error> {
        let state = self.meta_data.getter(id);
        Ok([state.gallery_id.get(), state.system_nft_id.get(), state.meta_data.get()])
    }
}

impl Erc1155 {
    // Internal single token transfer.
    pub fn _transfer_single(
        &mut self,
        operator: Address,
        from: Address,
        to: Address,
        id: U256,
        amount: U256
    ) -> Result<(), Erc1155Error> {
        let balance_from = self.balances.getter(id).getter(from).get();
        let balance_to = self.balances.getter(id).getter(to).get();

        if operator != from && !self.is_approved_for_all(from, operator) {
            return Err(Erc1155Error::InvalidParameter(InvalidParameter { point: 17 }));
        }

        if balance_from < amount {
            return Err(Erc1155Error::InvalidParameter(InvalidParameter { point: 111 }));
        }

        // Update `from` balance
        {
            let mut b_x = self.balances.setter(id);
            let mut balance_from_x = b_x.setter(from);
            balance_from_x.set(balance_from - amount);
        }

        // Update `to` balance
        {
            let mut b_y = self.balances.setter(id);
            let mut balance_to_y = b_y.setter(to);
            balance_to_y.set(balance_to + amount);
        }

        Ok(())
    }

    pub fn check_admin(&mut self) -> Result<(), Erc1155Error> {
        let default_x = Address::from([0x00; 20]);
        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                Erc1155Error::InvalidParameter(InvalidParameter {
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
