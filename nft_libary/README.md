# NFT Library Contract: Mainx for NovaValult 📚

The **Mainx** contract is a central component of the NovaValult platform that manages NFT submissions and tracks their review status. It organizes NFT data per gallery, supports both raw and accepted NFTs, and maintains an overall count of NFTs in the system. This contract works closely with the gallery and NFT submission contracts to ensure that every NFT's status is transparently recorded and managed.

---

## Overview 🚀

- **Purpose:**  
  Mainx serves as the NFT library for NovaValult, holding the core data for each NFT submitted to a gallery. It maintains records for NFTs under review and those that have been accepted, allowing for efficient tracking and retrieval.

- **Key Responsibilities:**
  - **Storing NFT Data:**  
    Each gallery has a dedicated data structure (a **Concept**) that holds raw NFT submissions and accepted NFTs.
  - **Review Management:**  
    The contract allows gallery admins to mark NFTs as accepted or rejected.
  - **Tracking Totals:**  
    It keeps a global count of accepted NFTs across galleries.
  - **Interacting with External Contracts:**  
    It retrieves gallery information and validates NFT submissions via calls to the gallery contract.

---

## Data Structures & Storage 🔧

- **Concept:**  
  Each gallery (indexed by a unique ID) has an associated **Concept** that contains:
  - A mapping (`data_x`) of NFT submissions (each with an owner, status, and metadata index).
  - A mapping (`accepted`) for accepted NFTs to facilitate easy lookup by their new index.
  - Counters (`available_index` for raw submissions and `av_accepted_index` for accepted NFTs).

- **Nft Struct:**  
  Represents an individual NFT with:
  - **Status:**  
    - `0` for "undergoing review"  
    - `1` for "accepted"  
    - `2` for "rejected"
  - **Owner:**  
    The address of the NFT creator.
  - **Data:**  
    A reference to the metadata index stored in the NFT submission contract.

- **Global Variables:**
  - **gallery_data:**  
    Maps each gallery ID to its corresponding Concept.
  - **total_nft:**  
    Tracks the total number of accepted NFTs in the system.
  - **Contract Addresses:**  
    - `gallery_c`: The gallery contract address.
    - `nft_submit`: The allowed contract for NFT metadata submissions.
    - `admin`: The admin address controlling the library.

---

## Core Functions & Workflow 📚

### 1. Submitting an NFT: `submit_nft`

- **What It Does:**  
  Accepts NFT submissions from the authorized NFT submission contract.
- **Workflow:**
  - Retrieves gallery information (like start time) to ensure the gallery hasn't begun.
  - Checks that the submitting user has a valid ticket for the gallery.
  - Records the NFT data (creator and metadata reference) under a new index.
  - Logs a **SubmitedNft** event to indicate successful submission.

---

### 2. Setting NFT Status: `set_nft_state`

- **What It Does:**  
  Allows a gallery admin (the gallery creator) to update the status of an NFT.
- **Workflow:**
  - Retrieves gallery info and validates that the caller is the gallery creator.
  - Checks the provided NFT ID exists and that the NFT hasn't been updated already.
  - Updates the NFT's status:
    - **Accepted (1):**  
      - Logs an **AcceptedNft** event.
      - Adds the NFT to the accepted list and increments the accepted index.
      - Updates the global NFT count.
    - **Rejected (2):**  
      - Logs a **RejectedNft** event.
  - Returns an error if any validations fail.

---

### 3. Listing and Retrieving NFTs

- **`nft_list_len`:**  
  Returns the length of the raw NFT submissions and the accepted NFT list for a given gallery.
  
- **`get_nft`:**  
  Retrieves details about a specific NFT from a gallery.  
  - When `raw` is set to **true**, only the gallery admin can fetch raw submission data.
  - When `raw` is **false**, it returns the accepted NFT data (creator, status, metadata index).

- **`get_system_total_nft`:**  
  Returns the total number of accepted NFTs across all galleries.

---

## Helper & Utility Functions 🛠️

- **Ticket Verification (`c_tik`):**  
  Checks if a user has a valid ticket for a given gallery by calling the gallery contract.

- **Gallery Information Retrieval (`get_gal_info`):**  
  Gets key details (like the gallery creator and start time) from the gallery contract.

- **Cooldown Check (`cd_ck`):**  
  Verifies that a gallery event has not started and that the user holds a valid ticket before processing an NFT submission.

- **Admin Check (`check_admin`):**  
  Ensures that only an authorized admin can perform certain administrative actions, such as setting contract addresses.

---

## Usage Examples 💡

### Submitting an NFT

A submission typically comes from the NFT submission contract. When a user submits an NFT:
- The contract checks that the gallery hasn't started and that the user has a ticket.
- The NFT is stored under a new index and a **SubmitedNft** event is logged.

### Accepting or Rejecting an NFT

Gallery admins can update an NFT's status by:
- Calling `set_nft_state` with the appropriate state value:
  - **1** to accept the NFT.
  - **2** to reject it.
- Accepted NFTs are assigned a new index for easier tracking, and events are logged accordingly.

### Retrieving NFT Data

Users and admins can retrieve:
- **Raw NFT data:** (for internal review, accessible only to the admin)
- **Accepted NFT data:** (available publicly for accepted NFTs)

---

## Conclusion 🎉

The **Mainx** NFT Library contract is vital for organizing and managing NFT submissions within NovaValult. By providing structured storage, rigorous status updates, and robust validation via external contract calls, Mainx ensures that every NFT's journey—from submission to acceptance—is transparent and secure.

Embrace a smarter way to manage NFT data with Mainx on NovaValult! 🚀📚
