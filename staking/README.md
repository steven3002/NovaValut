# Staking Contract: NovaValult Voting & Leaderboard System 📊

The Staking Contract is a vital component of the **NovaValult** platform. It securely manages user votes (staked bids) for NFTs within each gallery and builds dynamic leaderboards to determine the top stakers. This contract ensures that votes are cast only once per gallery per user (with the option to update the bid later) and organizes the ranking of votes to identify the most influential validators.

---

## Overview 🚀

- **Purpose:**  
  The contract tracks and records votes on NFTs in a gallery. Each vote is associated with a bid value that contributes to a dynamic leaderboard for that NFT. These leaderboards help determine NFT validation outcomes and reward distribution.

- **Key Responsibilities:**  
  - Recording a user’s vote (bid) for an NFT.
  - Allowing vote updates (increasing a bid) during the voting period.
  - Maintaining a leaderboard of top stakers for each NFT.
  - Enforcing that each user votes only once per gallery while enabling bid upgrades.
  - Integrating with external systems (via a safe vote contract) to ensure only authorized calls update staking data.

---

## Data Structures & Storage 🔧

- **Stake Structure:**  
  - **room:** Maps each gallery ID to its corresponding **Gallery** structure.
  - **voted:** Tracks if a user has already voted in a given gallery.
  - **stake_control:** The address of the safe contract allowed to call staking functions.
  - **admin:** Admin address controlling critical functions.

- **Gallery Structure:**  
  - **total_votes:** Total votes cast in the gallery.
  - **nft:** Mapping from NFT IDs to their respective **Nft** data.

- **Nft Structure:**  
  - **leaderboard:** A mapping that ranks top stakers (by vote ID) for the NFT.
  - **total_votes:** Total votes cast for the NFT.
  - **casted:** A mapping of individual vote (or cast) records, where each vote is identified by a unique index.

- **Cast Structure:**  
  - **bid:** The bid amount (vote value) cast by a user.
  - **updated:** The timestamp of the last update to the vote.
  - **voter:** The address of the user who cast the vote.

---

## Core Functions & Workflow 📚

### 1. Casting a Vote (`stake`)
- **Purpose:**  
  Allows an authorized (safe) contract to record a user’s vote on an NFT.
- **Workflow:**
  - **Authorization:**  
    Checks that the call comes from the designated safe contract.
  - **One Vote Per Gallery:**  
    Verifies that the user hasn’t already voted in the gallery.
  - **Vote Registration:**  
    Increments the NFT’s total votes and records the new vote with the bid amount, current timestamp, and voter's address.
  - **Leaderboard Update:**  
    Calls `update_le_nft` to adjust the NFT’s leaderboard based on the new bid.
  - **Mark Vote State:**  
    Flags that the user has voted in the given gallery.
  - **Event Emission:**  
    Emits a **Stakes** event to log the vote details.

---

### 2. Updating a Vote (`update_bid`)
- **Purpose:**  
  Allows users to increase their bid on an NFT if they wish to improve their ranking.
- **Workflow:**
  - **Safe Contract Check:**  
    Ensures that only the safe contract can call this function.
  - **Vote Ownership:**  
    Validates that the user attempting the update is the original voter.
  - **Bid Comparison:**  
    Checks that the new bid is higher than the existing bid.
  - **Update Vote Data:**  
    Updates the bid value and timestamp in the corresponding vote record.
  - **Leaderboard Refresh:**  
    Calls `update_le_nft` to reorder the leaderboard based on the updated bid.
  - **Event Emission:**  
    Emits an **UpdatedCast** event with the old and new bid values.

---

### 3. Leaderboard Management (`update_le_nft`)
- **Purpose:**  
  Organizes the NFT’s leaderboard by ranking votes from highest to lowest.
- **Workflow:**
  - **Fetch Current Leaderboard:**  
    Retrieves the top entries from the NFT’s stored leaderboard.
  - **Incorporate New/Updated Vote:**  
    Updates the existing entry if the vote exists or adds a new entry if not.
  - **Sorting & Truncation:**  
    Sorts the leaderboard by bid amounts in descending order and keeps only the top entries (up to a fixed size, e.g., 30).
  - **Storage Update:**  
    Saves the updated leaderboard back to the NFT’s storage.

---

### 4. Utility Functions & Checks
- **Vote Status Check (`has_voted`):**  
  Returns whether a user has already cast a vote in a specific gallery.
- **Total Votes Retrieval (`get_total_votes` & `get_gallery_total_votes`):**  
  Retrieves the total votes for an NFT or an entire gallery.
- **Vote Data Retrieval (`get_cast`):**  
  Fetches details (bid, timestamp, voter) of a specific vote by its ID.
- **Leaderboard Retrieval (`get_leaderboard`):**  
  Provides a slice of the leaderboard for an NFT, sorted by bid value.
- **Position Calculation (`get_position`):**  
  Determines a user’s rank on the NFT’s leaderboard based on their vote.
- **Admin & Control (`set_control`, `check_admin`):**  
  Allows the admin to set the safe stake contract and ensures that only authorized calls can perform sensitive operations.

---

## Usage Examples 💡

### Casting a Vote
- **Scenario:** A user casts a vote with a specific bid value on an NFT within a gallery.
- **Process:**  
  - The safe contract calls `stake` with the user’s address, gallery ID, NFT ID, and bid amount.
  - The vote is recorded, the NFT’s total votes are incremented, and the leaderboard is updated.
  - An event is logged to record the vote.

### Updating a Vote
- **Scenario:** A user decides to increase their bid to improve their leaderboard ranking.
- **Process:**  
  - The safe contract calls `update_bid` with the user’s address, gallery ID, NFT ID, vote ID, and the new bid value.
  - The contract verifies ownership and updates the vote data.
  - The leaderboard is refreshed to reflect the updated bid.
  - An event logs the change.

### Viewing Leaderboard
- **Scenario:** Retrieve the top votes for an NFT to see the ranking.
- **Process:**  
  - Call `get_leaderboard` with the gallery ID, NFT ID, and desired range.
  - The function returns a list of vote IDs, bid amounts, voter addresses, and last update timestamps.

---

## Conclusion 🎉

The Staking Contract is at the heart of NovaValult’s voting system, ensuring that every vote is securely recorded and dynamically ranked. By managing both initial vote casting and subsequent bid updates, it fosters a competitive and transparent environment for NFT validation. This robust mechanism not only safeguards the integrity of the voting process but also directly influences reward distribution and NFT evaluation on NovaValult.

Empower your voice and stake your claim in the future of NFT validation with NovaValult’s innovative staking system! 🚀📊
