# Minter Contract: NovaValult Safe Minting 🚀

The Minter contract is a critical component of the **NovaValult** platform. It ensures that NFTs are minted safely after the voting period, based on the users' validated positions in the leaderboard. This contract interacts with several other contracts to retrieve gallery data, validate vote positions, and mint NFTs accordingly.

---

## Overview 📖

- **Purpose:**  
  The Minter contract allows users to claim their NFT rewards (SFTs) after the voting period ends. It verifies that the voting period is over, checks that the user has not already minted for a specific gallery, and mints NFT copies based on the user's leaderboard position:
  - **Position 0:** 3 copies  
  - **Position 1:** 2 copies  
  - **Position 2:** 1 copy

- **Key Interactions:**  
  - **Stake Contract:** Retrieves the user’s voting position.  
  - **Gallery Contract:** Fetches gallery information (voting end time, etc.).  
  - **NFT Library & NFT Storage:** Manages NFT identification and metadata.  
  - **ERC1155 Contract:** Handles the actual minting and data setting of NFTs.

- **Admin & Control:**  
  The contract includes admin functions to set critical addresses (stake, gallery, NFT library, ERC1155 token contract, and NFT storage) ensuring secure configuration and management.

---

## Key Features ✨

1. **Claiming NFT Rewards:**
   - **Function:** `claim_SFT(gallery_id, nft_id)`
   - **Process:**
     - Retrieves gallery information (specifically the voting end time).
     - Validates that the voting period has ended.
     - Checks if the user has already claimed/minted for that gallery.
     - Verifies the user’s position on the leaderboard via the stake contract.
     - Determines the number of NFT copies to mint based on the user's rank.
     - Calls external contracts to update the NFT library, mint NFTs, and set NFT metadata.
     - Marks the user as having minted for the specific gallery to prevent duplicate claims.

2. **Control & Configuration:**
   - **Function:** `set_control(...)`
   - **Purpose:**  
     Allows the admin to set or update the critical contract addresses that the Minter interacts with:
     - **Stake Contract Address:** For verifying vote positions.
     - **Gallery Contract Address:** For fetching gallery data.
     - **NFT Library Contract Address:** For NFT data retrieval.
     - **ERC1155 Token Contract Address:** For minting tokens.
     - **NFT Storage Contract Address:** For handling NFT submission and metadata.

3. **Utility Functions:**
   - **Time Check:**  
     `check_time(end)` ensures that NFT claiming occurs only after the voting period.
   - **Admin Verification:**  
     `check_admin()` enforces that only an authorized admin can update control addresses.
   - **Claim Check:**  
     `has_claimed(gallery_id)` lets users verify if they have already claimed their NFT for a gallery.

4. **Inter-Contract Communication:**  
   The contract uses defined interfaces to interact with external contracts (ERC20, Gallery, NFT Library, NFT Storage, and Stake contracts) ensuring modular and secure operations.

---

## Contract Architecture 🔧

### Storage Variables

- **`has_minted` Mapping:**  
  Tracks whether a user has already minted their NFT for a specific gallery.
  
- **Critical Contract Addresses:**
  - `stake` – Unsafe stake contract address.
  - `admin` – Admin contract address.
  - `nft_libary` – Address of the NFT library contract.
  - `nft_storage` – Address of the NFT submission/storage contract.
  - `gallery_c` – Address of the gallery contract.
  - `erc1155` – Address of the ERC1155 token contract.

### Interfaces

- **IErc20:**  
  For transferring tokens (if needed).
  
- **ISubject:**  
  To fetch gallery details like the voting end time.
  
- **IMainx:**  
  Retrieves NFT data from the NFT library.
  
- **INftStorage:**  
  Used to perform system minting operations on the NFT storage contract.
  
- **IErc1155:**  
  For minting NFTs and setting NFT metadata.
  
- **IStake:**  
  To determine the user’s position on the leaderboard for a specific gallery and NFT.

### Error Handling

- **InvalidParameter:**  
  Custom error that handles various failure states such as invalid timing, duplicate claims, and unauthorized actions.

---

## Core Functions & Workflow 📚

### 1. Claiming NFT Rewards

```rust
pub fn claim_SFT(&mut self, gallery_id: U256, nft_id: U256) -> Result<(), MinterError> {
    // Retrieve gallery information (voting end time)
    let end = self.get_gal_info(gallery_id).map_err(|_| {
        MinterError::InvalidParameter(InvalidParameter { point: 202 })
    })?;

    // Ensure the voting period has ended
    self.check_time(end)?;

    // Prevent duplicate claims
    if self.has_minted.getter(msg::sender()).getter(gallery_id).get() {
        return Err(MinterError::InvalidParameter(InvalidParameter { point: 202 }));
    }

    // Check user's position on the leaderboard
    let position = self.get_position(gallery_id, nft_id)?;
    let amount = match position {
        0 => 3,
        1 => 2,
        2 => 1,
        _ => return Err(MinterError::InvalidParameter(InvalidParameter { point: 202 })),
    };

    // Retrieve NFT identifier from the NFT library
    let nft_storage_id = self.get_nft(gallery_id, nft_id)?;
    self.set_libary(nft_storage_id)?;

    // Mint the NFT copies based on the user's position
    self.mint(nft_storage_id, U256::from(amount))?;
    self.set_data(nft_storage_id, gallery_id, nft_id)?;

    // Mark that the user has claimed for this gallery
    let mut minting_state = self.has_minted.setter(msg::sender());
    let mut m_s_h = minting_state.setter(gallery_id);
    m_s_h.set(true);

    Ok(())
}
```

### 2. Setting Control Addresses (Admin Only)

```rust
pub fn set_control(
    &mut self,
    stake: Address,   // Unsafe stake contract address
    gallery: Address, // Gallery contract address
    libary: Address,  // NFT library contract address
    erc1155: Address, // ERC1155 token contract address
    nft_storage: Address // NFT storage contract address
) -> Result<(), MinterError> {
    self.check_admin()?;  // Ensure the caller is the admin
    self.stake.set(stake);
    self.gallery_c.set(gallery);
    self.nft_libary.set(libary);
    self.erc1155.set(erc1155);
    self.nft_storage.set(nft_storage);
    Ok(())
}
```

### 3. Utility Functions

- **Time Check:**  
  `check_time(end)` compares the current block timestamp with the voting end time.

- **Admin Check:**  
  `check_admin()` ensures that only the admin (or the first caller who sets the admin) can change control parameters.

- **Position & NFT Data Retrieval:**  
  - `get_position(gallery_id, nft_id)` fetches the user's leaderboard position from the stake contract.
  - `get_nft(gallery_id, nft_id)` retrieves the NFT identifier from the NFT library.

- **Minting & Metadata Setting:**  
  - `mint(new_nft_id, amount)` interacts with the ERC1155 contract to mint the NFT copies.
  - `set_data(token_id, gallery_id, system_nft_id)` sets the NFT metadata on the ERC1155 contract.
  - `set_libary(s_nft_id)` calls the NFT storage contract to update the NFT library.

---

## Usage Examples 💡

### Claiming Your NFT Reward

After the voting period ends, a user can claim their NFT reward by calling:

```rust
minter_instance.claim_SFT(gallery_id, nft_id)?;
```

This function will:
- Validate the voting period has ended.
- Check the user's leaderboard position.
- Mint NFT copies based on that position (3, 2, or 1 copy).
- Update the NFT library and metadata accordingly.

### Configuring Control Addresses (Admin Only)

Set up the necessary contract addresses with:

```rust
minter_instance.set_control(
    stake_contract_address,
    gallery_contract_address,
    nft_libary_contract_address,
    erc1155_contract_address,
    nft_storage_contract_address
)?;
```

---

## Conclusion 🎉

The Minter contract for **NovaValult** plays a vital role in securely minting NFTs as rewards based on community voting results. It integrates seamlessly with multiple contracts—ensuring that NFT rewards are only issued after proper validation and within a secure, controlled environment. This contract not only enhances the user experience by rewarding active participation but also ensures the integrity and transparency of the minting process.

Embrace the secure future of NFT minting with NovaValult! 🚀💎
