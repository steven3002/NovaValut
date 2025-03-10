# User Registration Contract: NovaValult User Identity & Airdrop 🚀

The User Registration Contract is a core component of the NovaValult platform. It allows users to register their digital identities, store profile information, and receive a welcome token airdrop based on a diminishing reward scheme as more users join.

---

## Overview 📖

- **Purpose:**  
  Enables users to register on NovaValult by providing their name, bio, and metadata. New registrants receive a token airdrop, and all profile data is stored on-chain for future reference.

- **Key Features:**
  - **User Registration:**  
    Users provide a name, bio, and metadata (in JSON format) to create their digital identity.
  - **Profile Management:**  
    Stores and retrieves user profiles, including registration and update timestamps.
  - **Airdrop Rewards:**  
    Awards tokens to new users. The token reward decreases as the number of registered users increases.
  - **Admin Control:**  
    Only the admin can update critical parameters like the ERC-20 contract address. The first caller becomes the admin if none is set.

---

## Constants & Reward Mechanism 🔢

- **AIRDROP:** 200,000  
  The initial token amount available for airdrop to new users.

- **SUBX:** 32  
  The reduction factor that decreases the airdrop amount as more users register.  
  - The actual token reward for a new user is computed as:  
    **Reward = AIRDROP - (SUBX × total_registered_users)**

---

## Core Functions & Workflow 📚

### 1. Registering a User (`register_user`)
- **What It Does:**  
  Registers a new user by storing their name, bio, and metadata.
- **Workflow:**
  - **Validation:**  
    Ensures that none of the input fields (name, bio, metadata) are empty.
  - **Profile Setup:**  
    Saves the user's name and profile details, including bio and metadata.
  - **Timestamping:**  
    Sets the registration time and last updated time.
  - **Airdrop Reward:**  
    If the user is not already registered, calculates the token reward using the reduction formula and mints tokens to the user's wallet.
  - **User Count Update:**  
    Increments the total registered users count.

---

### 2. Retrieving User Information (`get_user_info`)
- **What It Does:**  
  Retrieves the stored profile information for a registered user.
- **Workflow:**
  - **Check Registration:**  
    Ensures that the queried address is registered.
  - **Return Data:**  
    Provides the user's name, bio, metadata, registration timestamp, and last update timestamp.

---

### 3. Admin Configuration (`set_erc20`)
- **What It Does:**  
  Allows the admin to update the ERC-20 token contract address used for minting tokens.
- **Workflow:**
  - **Admin Verification:**  
    Ensures that only the admin (or the first caller) can make this change.
  - **Address Update:**  
    Sets the new ERC-20 contract address.

---

### 4. Internal Functions
- **`mint_tkn(tkn, address)`:**  
  Mints a specified amount of tokens to a user's address using the ERC-20 interface.
- **`check_admin()`:**  
  Validates that the caller is the admin; if not set, the first caller becomes the admin.
- **`re_f()`:**  
  Computes the current airdrop token reward based on the number of registered users.

---

## Usage Examples 💡

### Registering a User
```rust
users_instance.register_user(
    "Alice", 
    "Artist & Collector", 
    "{\"twitter\": \"@alice\", \"website\": \"alice.art\"}"
)?;
```
- This function validates inputs, stores Alice's profile, sets timestamps, and mints a welcome token airdrop based on the current reward calculation.

---

### Retrieving User Information
```rust
let (profile, timestamps) = users_instance.get_user_info(alice_address)?;
```
- Returns Alice's name, bio, metadata, and the timestamps for when she registered and last updated her profile.

---

### Admin Setting ERC-20 Contract Address
```rust
users_instance.set_erc20(new_erc20_contract_address)?;
```
- Only the admin (or the first caller setting the admin) can update the ERC-20 address.

---

## Conclusion 🎉

The User Registration Contract is essential for onboarding new users onto NovaValult. It ensures that every user has a verifiable digital identity while incentivizing early adoption through a token airdrop. With robust admin controls and an innovative reward reduction mechanism, this contract lays a solid foundation for a secure and engaging user experience on NovaValult.

Welcome to the future of digital identity and decentralized rewards—welcome to NovaValult! 🚀
