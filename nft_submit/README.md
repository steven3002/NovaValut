# NFT Submit Contract: NovaValult NFT Metadata Storage 📤

The NFT Submit contract is a vital component of the **NovaValult** platform. It manages the submission and storage of NFT metadata before the minting process. This contract holds the metadata for NFTs in a structured format and ensures that only authorized users with a valid ticket can access or submit NFT data.

---

## Overview 🚀

- **Purpose:**  
  The NFT Submit contract (named **NftStorage**) stores NFT metadata submitted by users for a specific gallery. It acts as a staging area for NFT data that is later referenced by the NFT Library and used during the minting process.  
- **Core Responsibilities:**  
  - Collecting and storing stringified JSON metadata for NFTs.
  - Ensuring that NFT submissions are only accepted from authorized sources.
  - Enforcing access control so that only users with valid gallery tickets (or the minter) can view NFT metadata.
  - Interacting with the NFT Library contract to register the NFT submission.
  - Enabling the minter to mark NFT data as "open" for public viewing upon successful minting.

---

## Key Features ✨

1. **NFT Submission:**  
   - Users can submit NFT metadata (in string format) along with the target gallery ID.
   - A unique NFT identity is generated for each submission.

2. **Access Control:**  
   - Only users with a valid ticket for the respective gallery can retrieve NFT metadata.
   - The contract distinguishes between raw (non-public) and open (public) NFT data.
   - The minter has exclusive permission to mark NFTs as open for public viewing.

3. **Integration with NFT Library:**  
   - Upon submission, the contract calls the NFT Library (via the `pass_data` function) to register the NFT.
   - This helps maintain a consistent system index across the NovaValult ecosystem.

4. **Admin Functions:**  
   - The contract includes an admin mechanism to set the NFT Library, gallery contract, and minter addresses.
   - Only the admin (or the first caller) can configure these critical parameters, ensuring secure management.

---

## Core Functions & Workflow 📚

### 1. Submitting NFT Metadata (`submit_nft`)
- **What It Does:**  
  Allows a user to submit their NFT's metadata for a specific gallery.
- **Workflow:**  
  - **Generate Identity:** Increments the available index to assign a unique ID for the NFT.
  - **Pass Data to NFT Library:** Calls the external NFT Library contract to register the submission via `pass_data`.
  - **Store Metadata:** Saves the metadata (stringified JSON), the submitting user's address, and the associated gallery ID.
  - **Event Logging:** Emits a **SubmitNft** event to record the submission details.

---

### 2. Retrieving NFT Metadata (`get_nft_data`)
- **What It Does:**  
  Retrieves the metadata for a submitted NFT.
- **Workflow:**  
  - Checks if the NFT is marked as open or if the caller is the minter.
  - Validates that the caller holds a valid ticket for the gallery if the NFT is not open.
  - Returns the NFT creator's address, the metadata string, and the associated gallery ID.

---

### 3. Minting Preparation (`system_mint`)
- **What It Does:**  
  Marks an NFT as open for public viewing, making it eligible for minting.
- **Workflow:**  
  - Only callable by the designated minter.
  - Updates the NFT's open state to `true` once the NFT is ready for minting.

---

### 4. Admin Setup (`set_libary`)
- **What It Does:**  
  Sets the critical contract addresses for the NFT Submit contract.
- **Workflow:**  
  - Can only be called by the admin.
  - Configures the addresses for:
    - **NFT Library:** Where the NFT data is cross-referenced.
    - **Gallery Contract:** For verifying gallery-specific conditions (e.g., ticket status).
    - **Minter Contract:** Authorized to trigger the minting process.

---

### 5. Helper Functions

- **Ticket Verification (`c_tik`):**  
  Checks if the caller holds a valid ticket for a given gallery by interacting with the Gallery Contract.

- **Admin Verification (`check_admin`):**  
  Ensures that only the admin can perform critical operations, locking the admin role upon first configuration.

- **Registering Submission (`pass_data`):**  
  Calls the NFT Library contract to register the NFT submission, ensuring that the system maintains a consistent index.

---

## Usage Examples 💡

### Submitting NFT Metadata

A user submits an NFT by providing the gallery ID and the metadata as a JSON string:
```rust
nft_storage_instance.submit_nft(
    gallery_id,       // Gallery ID to which the NFT belongs
    json_metadata     // Stringified JSON containing NFT metadata
)?;
```

### Retrieving NFT Metadata

Users (with a valid ticket or if the NFT is open) can retrieve the NFT data:
```rust
let (creator, metadata, gallery_id) = nft_storage_instance.get_nft_data(nft_id)?;
```

### Marking NFT as Open for Minting

The minter can mark the NFT as open (ready for minting):
```rust
nft_storage_instance.system_mint(nft_id)?;
```

### Setting Up Admin Parameters

The admin configures the NFT Submit contract with necessary external addresses:
```rust
nft_storage_instance.set_libary(
    nft_library_address,  // Address of the NFT Library contract
    gallery_contract_address,  // Address of the Gallery contract
    minter_contract_address    // Address of the Minter contract
)?;
```

---

## Conclusion 🎉

The NFT Submit contract in NovaValult is designed to securely handle the submission and storage of NFT metadata. By enforcing strict access controls and integrating closely with the NFT Library and Gallery contracts, it ensures that NFT data is accurately recorded and remains tamper-proof until the minting process. This setup not only protects the integrity of the NFT metadata but also streamlines the minting workflow for a seamless user experience.

Welcome to a new era of NFT creation and management with NovaValult! 🚀📤
