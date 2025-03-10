# ERC1155 Contract: novaValult NFTs and SFT 🎨

This contract is a robust ERC1155 implementation designed for the novaValult platform. It manages both NFTs and Semi-Fungible Tokens (SFTs) with extended metadata capabilities and admin-controlled minting—all built on Arbitrum Stylus Rust.

---

## Overview 🚀

- **Collection Name:**  
  Defined by the `NovaParams` trait as `"Nova Vault NFTs and SFT"`.  
  *(Note: While the immutable name here is defined as "Nova Vault NFTs and SFT", the platform is branded as **novaValult**.)*

- **Purpose:**  
  This contract extends standard ERC1155 functionality to:
  - Handle multiple token types within a single contract.
  - Maintain detailed metadata linking each token to its gallery, system NFT ID, and additional metadata ID.
  - Securely mint tokens via designated admin and minter roles.
  - Facilitate safe transfers and batch operations with full event logging.

---

## Key Features ✨

1. **Multi-Token Management:**
   - **Balance Queries:**  
     - `balance_of(account, id)` returns the token balance for a specific ID.
     - `balance_of_batch(accounts, ids)` enables batch queries for multiple tokens.
   - **Operator Approvals:**  
     - `set_approval_for_all(operator, approved)` lets users delegate token management.
     - `is_approved_for_all(account, operator)` verifies delegated permissions.

2. **Secure Transfers:**
   - **Single Transfers:**  
     - `safe_transfer_from(from, to, id, amount, data)` securely moves tokens between addresses.
   - **Batch Transfers:**  
     - `safe_batch_transfer_from(from, to, ids, amounts, data)` handles multiple token transfers in one go.

3. **Minting & Supply Control:**
   - **Minting Functions:**  
     - `_mint(to, id, amount, data)` and `_mint_batch(to, ids, amounts, data)` allow the designated minter to create new tokens.
   - **Total Supply Tracking:**  
     - Maintains a mapping to track the total number of tokens for each token ID.

4. **Metadata Management:**
   - **Custom Data Storage:**  
     - Uses a `MetaData` struct to store the gallery ID, system NFT ID, and metadata ID for each token.
   - **Data Operations:**  
     - `setData(id, g_id, s_n_id, m_d_id)` assigns metadata to a token.
     - `getData(id)` retrieves metadata for verification and display.

5. **Admin Controls & Security:**
   - **Minter & Admin Roles:**  
     - Only the designated minter can mint tokens.
     - `set_minter(minter)` allows admin-controlled assignment of the minter address.
   - **Error Handling:**  
     - Custom errors (e.g., `InvalidParameter`) ensure robust failure management and transparency.

6. **Event Logging:**
   - **Transparency:**  
     - Events such as `TransferSingle`, `TransferBatch`, and `ApprovalForAll` log critical actions to the blockchain for auditing and debugging.
   - **URI Event (Commented):**  
     - An event for URI updates is in place for future metadata handling enhancements.

---

## Architecture & Components 🔧

- **Storage Variables:**
  - **Balances:**  
    Maps token IDs to owner addresses and their balances.
  - **Operator Approvals:**  
    Maps account addresses to their approved operators.
  - **Total Supply:**  
    Keeps track of the total number of tokens for each token ID.
  - **Metadata Mapping:**  
    Associates each token ID with its metadata (gallery ID, system NFT ID, metadata ID).
  - **Minter & Admin:**  
    Addresses that control minting and admin functions, ensuring secure and controlled operations.

- **NovaParams Trait:**
  - Provides immutable parameters such as the collection name, ensuring consistency across the platform.

- **Error Management:**
  - Uses a defined `Erc1155Error` enum with custom errors to manage invalid operations or unauthorized access.

---

## Usage Guide 📚

### Querying Balances
```rust
let balance = erc1155_instance.balance_of(user_address, token_id);
let balances = erc1155_instance.balance_of_batch(vec![addr1, addr2], vec![token_id1, token_id2]);
```

### Managing Approvals
```rust
erc1155_instance.set_approval_for_all(operator_address, true);
let isApproved = erc1155_instance.is_approved_for_all(user_address, operator_address);
```

### Transferring Tokens
- **Single Transfer:**
  ```rust
  erc1155_instance.safe_transfer_from(from_address, to_address, token_id, amount, vec![])?;
  ```
- **Batch Transfer:**
  ```rust
  erc1155_instance.safe_batch_transfer_from(from_address, to_address, vec![token_id1, token_id2], vec![amount1, amount2], vec![])?;
  ```

### Minting Tokens (Admin Only)
- **Single Mint:**
  ```rust
  erc1155_instance._mint(recipient_address, token_id, amount, vec![])?;
  ```
- **Batch Mint:**
  ```rust
  erc1155_instance._mint_batch(recipient_address, vec![token_id1, token_id2], vec![amount1, amount2], vec![])?;
  ```

### Metadata Management
- **Setting Metadata:**
  ```rust
  erc1155_instance.setData(token_id, gallery_id, system_nft_id, metadata_id)?;
  ```
- **Getting Metadata:**
  ```rust
  let data = erc1155_instance.getData(token_id)?;
  ```

### Admin Functions
- **Setting the Minter:**
  ```rust
  erc1155_instance.set_minter(new_minter_address)?;
  ```

---

## Conclusion 🎉

The ERC1155 contract is a core component of the novaValult ecosystem, enabling secure and flexible management of digital assets. Its robust design supports multiple token types, detailed metadata, and secure minting—making it ideal for modern NFT applications. Whether you're transferring, minting, or verifying token data, this contract provides the necessary tools to ensure a seamless and transparent experience.

Embrace the future of digital asset management with **novaValult NFTs and SFT**! 🚀🎨
