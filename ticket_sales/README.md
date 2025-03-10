# Ticket Sale Contract: NovaValult Ticketing System 🎟️

This contract handles the sale of tickets for galleries on the NovaValult platform. It enables users to purchase entry tickets for galleries by transferring funds (via an ERC-20 token) and then updating their ticket status. The contract interacts with the Gallery contract to validate gallery details and update ticket records.

---

## Overview 🚀

- **Purpose:**  
  Facilitate secure and transparent ticket sales for galleries on NovaValult. Users can buy tickets after verifying the gallery’s validity and ensuring they haven’t already purchased one.

- **Key Interactions:**  
  - **ERC-20 Token Contract:** Handles the transfer of funds for ticket purchases.  
  - **Gallery Contract (ISubject):** Provides gallery details (creator, ticket price, last index, user ticket status) and updates ticket ownership when a purchase is made.

- **Admin Control:**  
  The contract allows an admin to set or update critical addresses (ERC-20 and Gallery contract addresses) to ensure proper operation.

---

## Core Functions & Workflow 📚

### 1. Buying a Ticket (`buy_ticket`)
- **What It Does:**  
  Lets a user purchase a ticket for a gallery.
  
- **Workflow:**
  - **Gallery Validation:**  
    Uses `i_chk` to check if the provided gallery index is valid.
  - **Duplicate Check:**  
    Calls `c_tik` to ensure the user hasn't already purchased a ticket for that gallery.
  - **Retrieve Gallery Info:**  
    Fetches the gallery creator and ticket price using `get_gal_info`.
  - **Fund Transfer:**  
    If the ticket price is greater than zero, calls `fund_tf` to transfer tokens from the buyer to the gallery creator.
  - **Ticket Update:**  
    Calls `up_tik` to update the ticket status in the Gallery contract.
  - **Event Logging:**  
    Emits a **BoughtTicket** event (for the buyer) and a **SoldTicket** event (for the seller/creator).

---

### 2. Admin Configuration (`set_erc20_gallery`)
- **What It Does:**  
  Allows the admin to set the ERC-20 token and Gallery contract addresses.
  
- **Workflow:**
  - **Admin Check:**  
    Uses `check_admin` to ensure that only the admin can perform this configuration.
  - **Address Update:**  
    Sets the new ERC-20 token contract address and the Gallery contract address.

---

## Helper Functions & Checks 🔧

- **Ticket Check (`c_tik`):**  
  Determines if the caller already holds a ticket for the specified gallery by querying the Gallery contract.

- **Index Check (`i_chk`):**  
  Verifies the gallery index by comparing it with the last index obtained from the Gallery contract.

- **Fund Transfer (`fund_tf`):**  
  Handles the ERC-20 token transfer from the ticket buyer to the gallery creator.  
  - If the buyer has insufficient allowance, it returns an error.

- **Gallery Info Retrieval (`get_gal_info`):**  
  Gets the gallery creator’s address and ticket price from the Gallery contract.

- **Ticket Update (`up_tik`):**  
  Calls the Gallery contract to update the ticket status after a successful purchase.

- **Admin Check (`check_admin`):**  
  Ensures that only the admin can perform administrative actions (like setting contract addresses).  
  - If the admin is not yet set, the first caller becomes the admin.

---

## Events & Error Handling 🎉🚨

- **Events:**  
  - **BoughtTicket:**  
    Logs details when a user successfully purchases a ticket (buyer, gallery index, ticket price, timestamp).
  - **SoldTicket:**  
    Logs details when a ticket is sold (seller/creator, gallery index, ticket price, timestamp).

- **Errors:**  
  - **InvalidParameter:**  
    Thrown when parameters (e.g., an invalid gallery index) are incorrect.
  - **ExistingTicket:**  
    Indicates that the user already has a ticket for the specified gallery.
  - **InSufficientAllowance:**  
    Raised when the buyer does not have enough ERC-20 tokens approved for transfer.
  - **NoData:**  
    Indicates missing data, such as gallery information.

---

## Usage Example 💡

### Buying a Ticket
```rust
// A user purchases a ticket for a gallery.
ticket_sale_instance.buy_ticket(gallery_index)?;
```
- The function will:
  - Validate the gallery index.
  - Check if the user already owns a ticket.
  - Retrieve the gallery creator and ticket price.
  - Transfer the necessary funds using ERC-20 tokens.
  - Update the gallery’s ticket records.
  - Emit events to log the transaction.

### Admin Configuration
```rust
// Admin sets the ERC-20 token and Gallery contract addresses.
ticket_sale_instance.set_erc20_gallery(erc20_contract_address, gallery_contract_address)?;
```
- Only the admin (or the first caller setting the admin) can perform this action.

---

## Conclusion 🎉

The Ticket Sale Contract is an integral part of the NovaValult platform, ensuring that ticket purchases are secure, transparent, and efficient. By leveraging ERC-20 token transfers and interfacing with the Gallery contract, it provides a robust system for managing gallery access. With its clear admin controls and comprehensive error handling, NovaValult users can confidently purchase tickets and participate in the platform's vibrant ecosystem.

Enjoy seamless gallery access with NovaValult Ticketing! 🚀🎟️
