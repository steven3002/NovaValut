# Safe Vote Contract: NovaValult Voting System 🗳️

The Safe Vote Contract is an essential component of the **NovaValult** platform that manages secure, token-based voting for NFTs. It ensures that users can safely cast and increase their votes during the gallery’s voting period while enforcing eligibility, timing, and fund transfer checks. This contract interacts with several external contracts—such as the gallery, stake, ERC-20, and NFT Library contracts—to verify conditions and record votes securely.

---

## Overview 🚀

- **Purpose:**  
  Enables users to cast votes on NFTs in a gallery by transferring tokens as bids. Votes are recorded safely, and users can also increase their previously cast bids. The contract guarantees that:
  - Users can only vote if they have a valid ticket for the gallery.
  - Votes can only be cast during the active voting period.
  - Bids meet the minimum requirements and are correctly transferred to the NFT creator.
  - Only authorized voters (and the vote owner) can update their bids.

- **Key Integrations:**  
  - **ERC-20 Contract:** Handles token transfers for bidding.
  - **Gallery Contract:** Provides gallery details (voting times, user ticket status).
  - **Stake Contract:** Manages vote recording and bid updates.
  - **NFT Library Contract:** Used to verify NFT creator details and vote ownership.

---

## Key Features ✨

1. **Casting Votes Securely:**  
   - **Function:** `cast_vote`  
     Allows a user to cast a vote on an NFT by submitting a bid. The function:
     - Retrieves gallery voting information (start, end, and minimum bid).
     - Validates that the bid meets the minimum requirement.
     - Confirms that the user holds a valid ticket and has not already voted.
     - Checks that the vote is cast during the active voting period.
     - Transfers funds from the voter to the NFT creator.
     - Records the vote in the stake contract.

2. **Increasing a Vote:**  
   - **Function:** `increase_cast`  
     Enables a user to increase their bid on an already cast vote. The process:
     - Ensures the vote is being increased during the valid voting period.
     - Retrieves the current bid value and confirms the new bid is higher.
     - Transfers only the difference in bid value from the voter to the NFT creator.
     - Updates the bid amount in the stake contract.

3. **Admin Control & Configuration:**  
   - **Function:** `set_control`  
     Allows the admin to configure the essential contract addresses (stake, ERC-20, gallery, and NFT Library) to ensure all interactions are routed correctly.
   - **Admin Verification:**  
     Uses a check to lock the admin role and restricts sensitive functions to authorized users.

4. **Supporting Utility Functions:**  
   - **Gallery Information Retrieval:**  
     `get_gal_info` fetches voting start/end times and the minimum bid required.
   - **Ticket Verification:**  
     `c_tik` checks if a user holds a valid ticket for a gallery.
   - **Vote Ownership & Data:**  
     Functions like `get_creator` and `get_staking_data` ensure that the NFT exists and that only the owner of a vote can update their bid.
   - **Fund Transfer:**  
     `fund_tf` facilitates the secure transfer of tokens (ERC-20) from the voter to the NFT creator.

---

## Core Functions & Workflow 📚

### 1. Casting a Vote (`cast_vote`)
- **Functionality:**  
  Enables users to cast their vote (bid) on a specific NFT within a gallery.
- **Workflow:**
  - **Gallery Info & Eligibility:**  
    Retrieves voting parameters (start time, end time, minimum bid) from the gallery contract. Checks if the bid is sufficient and that the user holds a valid ticket.
  - **Timing Verification:**  
    Validates that the current time is within the active voting period.
  - **NFT Existence & Ownership:**  
    Retrieves the NFT creator from the NFT Library to ensure the NFT exists.
  - **Fund Transfer:**  
    Transfers the bid amount from the voter to the NFT creator via the ERC-20 contract.
  - **Vote Recording:**  
    Calls the stake contract to record the vote with the provided bid.

---

### 2. Increasing a Vote (`increase_cast`)
- **Functionality:**  
  Allows users to increase the value of an already cast vote.
- **Workflow:**
  - **Retrieve Current Vote Details:**  
    Fetches current voting parameters and the existing bid from the stake contract.
  - **Validation:**  
    Ensures that the new bid is higher than the current bid.
  - **Fund Transfer & Update:**  
    Transfers the difference in bid amount from the voter to the NFT creator and updates the bid value in the stake contract accordingly.

---

### 3. Admin Configuration (`set_control`)
- **Functionality:**  
  Enables the admin to set up or update the addresses of integrated contracts.
- **Workflow:**
  - **Admin Verification:**  
    Checks that the caller is the admin (or sets the admin if not already set).
  - **Address Update:**  
    Configures addresses for the stake, ERC-20, gallery, and NFT Library contracts, ensuring proper routing of calls.

---

### 4. Utility Functions
- **Gallery Info (`get_gal_info`):**  
  Retrieves the gallery's voting period and minimum bid.
- **Ticket Check (`c_tik`):**  
  Verifies if the caller has a valid ticket for the gallery.
- **Time Check (`check_time`):**  
  Ensures that actions are performed within the designated voting period.
- **Fund Transfer (`fund_tf`):**  
  Manages ERC-20 token transfers securely.
- **Vote Data Retrieval (`get_staking_data` & `get_creator`):**  
  Confirms vote ownership and verifies NFT creator information.

---

## Usage Examples 💡

### Casting a Vote
```rust
// A user casts a vote on an NFT with a specified bid amount
cast_instance.cast_vote(
    gallery_id,  // ID of the gallery
    nft_id,      // ID of the NFT being voted on
    bid_amount   // Bid amount (must meet or exceed minimum bid)
)?;
```

### Increasing an Existing Vote
```rust
// A user increases their vote (bid) on an NFT
cast_instance.increase_cast(
    gallery_id,  // ID of the gallery
    nft_id,      // ID of the NFT
    vote_id,     // Existing vote identifier
    new_bid      // New bid value (must be higher than the current bid)
)?;
```

### Admin Setting Up the Contract
```rust
// Admin configures the safe vote contract by setting the necessary external addresses
cast_instance.set_control(
    stake_contract_address,  // Address of the stake contract
    erc20_contract_address,    // Address of the ERC-20 token contract
    gallery_contract_address,  // Address of the gallery contract
    nft_library_address        // Address of the NFT Library contract
)?;
```

---

## Conclusion 🎉

The Safe Vote Contract on **NovaValult** is designed to provide a secure and transparent voting mechanism for NFT validation. By integrating rigorous checks for eligibility, timing, and fund transfers, it ensures that all votes (bids) are cast safely and recorded accurately. With its robust functionality and admin-controlled configuration, the contract plays a pivotal role in maintaining the integrity and fairness of the NovaValult voting ecosystem.

Cast your vote confidently and help shape the future of NFT validation with NovaValult! 🗳️🚀
