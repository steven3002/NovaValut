#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;
use alloy_primitives::{ Address, U256 };
use alloy_sol_types::sol;
use stylus_sdk::{ evm, msg, prelude::* };

// ERC-1155 Events
sol! {
    event TransferSingle(address indexed operator, address indexed from, address indexed to, uint256 id, uint256 value);
    event TransferBatch(address indexed operator, address indexed from, address indexed to, uint256[] ids, uint256[] values);
    event ApprovalForAll(address indexed account, address indexed operator, bool approved);
    event URI(string value, uint256 indexed id);

    error InvalidParameter(uint8 point);
}

sol_storage! {
    /// ERC-1155 storage
    pub struct Erc1155 {
        /// Token balances: token ID -> owner -> balance
        mapping(uint256 => mapping(address => uint256)) balances;
        /// Approval map: owner -> operator -> approval status
        mapping(address => mapping(address => bool)) operator_approvals;
        /// URI for metadata
        mapping(uint256 => string) uris;
    }
}

#[derive(SolidityError)]
pub enum Erc1155Error {
    InvalidParameter(InvalidParameter),
}

#[public]
impl Erc1155 {
    /// Returns the balance of `account` for token `id`.
    pub fn balance_of(&self, account: Address, id: U256) -> U256 {
        self.balances.get(id).get(account)
    }

    /// Batch balance query
    pub fn balance_of_batch(&self, accounts: Vec<Address>, ids: Vec<U256>) -> Vec<U256> {
        accounts
            .iter()
            .zip(ids.iter())
            .map(|(&account, &id)| self.balance_of(account, id))
            .collect()
    }

    /// Grants or revokes permission for `operator` to manage all tokens of `msg.sender`.
    pub fn set_approval_for_all(&mut self, operator: Address, approved: bool) {
        self.operator_approvals.setter(msg::sender()).insert(operator, approved);
        evm::log(ApprovalForAll {
            account: msg::sender(),
            operator,
            approved,
        });
    }

    /// Checks if `operator` is approved to manage tokens of `account`.
    pub fn is_approved_for_all(&self, account: Address, operator: Address) -> bool {
        self.operator_approvals.getter(account).get(operator)
    }

    /// Transfers tokens.
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
        });
        // self._do_safe_transfer_check(msg::sender(), from, to, id, amount, data);
        Ok(())
    }

    /// Batch transfer of tokens.
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
        });
        // self._do_safe_transfer_check(msg::sender(), from, to, ids, amounts, data);
        Ok(())
    }

    /// Internal single token transfer.
    fn _transfer_single(
        &mut self,
        operator: Address,
        from: Address,
        to: Address,
        id: U256,
        amount: U256
    ) -> Result<(), Erc1155Error> {
        let mut balance_from = self.balances.setter(id).setter(from);
        let mut balance_to = self.balances.setter(id).setter(to);

        let from_balance = balance_from.get();
        if from_balance < amount {
            return Err(
                Erc1155Error::InvalidParameter(InvalidParameter {
                    point: 17,
                })
            );
        }

        balance_from.set(from_balance - amount);
        balance_to.set(balance_to.get() + amount);
        Ok(())
    }

    /// Mints a token.
    pub fn _mint(&mut self, to: Address, id: U256, amount: U256, data: Vec<u8>) {
        let mut balance = self.balances.setter(id).setter(to);
        balance.set(balance.get() + amount);

        evm::log(TransferSingle {
            operator: Address::ZERO,
            from: Address::ZERO,
            to,
            id,
            value: amount,
        });

        // self._do_safe_transfer_check(Address::ZERO, Address::ZERO, to, id, amount, data);
    }

    /// Batch minting of tokens.
    pub fn _mint_batch(&mut self, to: Address, ids: Vec<U256>, amounts: Vec<U256>, data: Vec<u8>) {
        if ids.len() != amounts.len() {
            return;
        }
        for (id, amount) in ids.iter().zip(amounts.iter()) {
            let mut balance = self.balances.setter(*id).setter(to);
            balance.set(balance.get() + *amount);
        }

        evm::log(TransferBatch {
            operator: Address::ZERO,
            from: Address::ZERO,
            to,
            ids: ids.clone(),
            values: amounts.clone(),
        });

        // self._do_safe_transfer_check(Address::ZERO, Address::ZERO, to, ids, amounts, data);
    }

    /// Internal function for transfer checks.
    // fn _do_safe_transfer_check(
    //     &self,
    //     operator: Address,
    //     from: Address,
    //     to: Address,
    //     ids: impl Into<Vec<U256>>,
    //     amounts: impl Into<Vec<U256>>,
    //     data: Vec<u8>
    // ) {

    // }

    /// Sets the URI for metadata.
    pub fn _set_uri(&mut self, id: U256, new_uri: String) {
        self.uris.setter(id).set_str(new_uri.clone());
        evm::log(URI {
            value: new_uri,
            id,
        });
    }

    /// Gets the URI for a specific token ID.
    pub fn uri(&self, id: U256) -> String {
        self.uris.get_string(id)
    }
}
