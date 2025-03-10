# NFT Market Contract: NovaValult Marketplace 🛒

The NFT Market Contract is a decentralized marketplace component of **NovaValult**. It enables users to list, offer, and purchase NFTs using a token-based payment system. Built with the Stylus SDK and targeting the Arbitrum network, this contract integrates with ERC-1155 and ERC-20 standards to ensure secure, transparent, and seamless NFT transactions.

---

## Overview 🚀

- **Purpose:**  
  This contract allows users to:
  - Offer their NFTs for sale by setting a price, sale status, and available quantity.
  - Retrieve sale details for multiple NFTs.
  - Purchase NFTs from other users while transferring funds securely.
  - Update and enforce marketplace rules through admin-controlled functions.

- **Key Standards:**  
  - **ERC-1155:** Used for NFT balance checks and safe transfers.
  - **ERC-20:** Utilized for transferring funds between buyers and sellers.

- **Core Components:**
  - **Sales Mapping:**  
    A double mapping that tracks sale details (price, sale state, available amount) for each NFT offered by a seller.
  - **Admin Control:**  
    An admin address is maintained to set up critical addresses (ERC-1155 and ERC-20 contracts) and enforce marketplace rules.

---

## Key Features ✨

1. **Offering NFTs for Sale:**  
   Sellers can list their NFTs by specifying:
   - **NFT ID:** The unique identifier of the NFT.
   - **Amount:** The number of NFTs they want to sell.
   - **Price:** The per-unit sale price.
   - **Sale State:** Whether the NFT is actively for sale.

2. **Batch Retrieval of Sale Information:**  
   Get sale details for multiple NFTs across different owners in a single call, returning the price, sale state, and available amount.

3. **Secure Purchase Workflow:**  
   - **Validation:** Checks that the seller has sufficient NFT balance and that the offer is valid.
   - **Approval Check:** Confirms that the seller has granted permission for the contract to transfer NFTs on their behalf.
   - **Fund Transfer:** Uses ERC-20 token transfers to securely move funds from the buyer to the seller.
   - **NFT Transfer:** Executes safe NFT transfers from the seller to the buyer.

4. **Admin Functions:**  
   - **Contract Setup:** The admin can set or update the addresses of the ERC-1155 and ERC-20 contracts.
   - **Access Control:** Ensures only authorized users (and the admin) perform sensitive operations.

---

## Core Functions & Workflow 📚

### 1. Offering an NFT (`offer`)
- **What It Does:**  
  Lists an NFT for sale by recording its sale details.
- **Key Steps:**
  - Verifies that the seller’s NFT balance meets the offer requirements.
  - Sets the sale price, available amount, and sale state in the contract’s storage.
  
### 2. Retrieving Sale Information (`get_cost_batch`)
- **What It Does:**  
  Retrieves a list of sale details for a batch of NFTs.
- **Key Steps:**
  - Accepts a list of seller addresses and NFT IDs.
  - Returns corresponding sale details (price, sale status, amount available).

### 3. Purchasing an NFT (`buy`)
- **What It Does:**  
  Allows buyers to purchase a specified quantity of NFTs from a seller.
- **Key Steps:**
  - Validates that the NFT is available for sale and that the desired amount does not exceed what is offered.
  - Checks that the seller has authorized the contract to transfer NFTs.
  - Transfers funds from the buyer to the seller using ERC-20 tokens.
  - Transfers the NFT(s) from the seller to the buyer using a safe ERC-1155 transfer.
  - Updates the sale record to reflect the reduced available amount.
  - Emits a **Sold** event to record the transaction.

### 4. Admin Setup (`set_erc1155`)
- **What It Does:**  
  Allows the admin to set the ERC-1155 (NFT) and ERC-20 (token) contract addresses.
- **Key Steps:**
  - Verifies the caller is the admin.
  - Updates the contract storage with new addresses for interacting with NFT and token contracts.

### 5. Supporting Internal Functions
- **Balance Check (`c_b`):**  
  Retrieves the seller’s NFT balance using the ERC-1155 standard.
- **Approval Check (`a_c`):**  
  Confirms that the seller has granted the contract permission to transfer their NFTs.
- **Fund Transfer (`fund_tf`):**  
  Facilitates the ERC-20 token transfer from the buyer to the seller.
- **NFT Transfer (`nft_tf`):**  
  Executes the safe transfer of NFTs from the seller to the buyer.
- **Admin Verification (`check_admin`):**  
  Ensures only the admin can execute sensitive functions like setting contract addresses.

---

## Usage Examples 💡

### Offering an NFT for Sale
```rust
market_instance.offer(
    nft_id,             // Unique identifier of the NFT.
    amount,             // Number of NFTs to sell.
    price,              // Price per NFT.
    true                // Set the NFT as available for sale.
)?;
```

### Retrieving Sale Information
```rust
let sale_details = market_instance.get_cost_batch(
    vec![seller_address],  // List of seller addresses.
    vec![nft_id]           // Corresponding NFT IDs.
);
// Returns a vector of tuples: (price, sale state, amount available)
```

### Purchasing an NFT
```rust
market_instance.buy(
    seller_address,  // Seller’s address.
    nft_id,          // NFT ID to purchase.
    purchase_amount  // Amount to purchase.
)?;
```

### Admin Setting Contract Addresses
```rust
market_instance.set_erc1155(
    erc1155_contract_address,  // Address of the ERC-1155 contract.
    erc20_contract_address     // Address of the ERC-20 contract.
)?;
```

---

## Conclusion 🎉

The NFT Market Contract on **NovaValult** is engineered to provide a secure, transparent, and efficient marketplace for NFT transactions. With robust checks for balance, approval, and fund transfers, it ensures that both sellers and buyers can engage in NFT trading with confidence. The admin-controlled setup further enforces a trusted and well-maintained marketplace ecosystem.

Embrace a new era of NFT trading with NovaValult Marketplace! 🚀🛒
