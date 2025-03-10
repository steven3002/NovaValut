# Gallery Contract: NovaValult 🏛️

This smart contract is the backbone of the gallery management system for the **NovaValult** platform. It allows users to create galleries, manage ticketing, and track user participation, all while ensuring that key conditions—such as voting periods and staking requirements—are met.

---

## Overview 🚀

The Gallery Contract enables the following functionalities:

- **Gallery Creation:**  
  Users can create new galleries by providing essential details such as the gallery name, metadata, ticket price, and voting conditions (voting start/end times and minimum staking amount).

- **Ticket Sales:**  
  A dedicated mechanism allows users to buy tickets for galleries. This is controlled via an allowed contract to ensure secure and authorized transactions.

- **User Data Management:**  
  Tracks the galleries a user has created and joined, providing a comprehensive view of their engagement on NovaValult.

- **Admin Controls:**  
  Functions to set the allowed contract address and perform administrative tasks, ensuring only authorized actions can modify key settings.

---

## Key Components & Data Structures 🔧

### Storage Variables

- **Gallery Mapping:**  
  `mapping(uint256 => Gallery) gallery`  
  Stores all galleries, each identified by a unique index.

- **Ticket Index Mapping:**  
  `mapping(address => mapping(uint256 => bool)) ticket_index`  
  Associates users with the galleries for which they have valid tickets.

- **User Data:**  
  `mapping(address => UserData) state`  
  Keeps track of galleries created and joined by each user.

- **Allowed Contract & Admin:**  
  `address allowed_contract` and `address admin`  
  Control access for ticket purchases and other administrative functions.

- **Available Index:**  
  `uint256 available_index`  
  Used to generate unique indices for newly created galleries.

### Structs

- **Gallery:**  
  Contains information such as:
  - `name` and `meta_data`
  - `price` (ticket price)
  - `owner` (gallery creator)
  - `attendes` (number of attendees)
  - `created_at` timestamp
  - `VotingCondition` (voting start time, end time, and minimum staking amount)

- **VotingCondition:**  
  Sets the voting parameters:
  - `voting_start` and `voting_end`
  - `minimum_staking_amount`

- **UserData:**  
  Tracks:
  - `created_gallery` (list of galleries created by the user)
  - `joined_gallery` (list of galleries the user has joined)

---

## Events & Errors 📣

### Events

- **NewGallery:**  
  Emitted when a new gallery is created, logging the creator's address, gallery name, index, price, and timestamp.

- **JoinedGallery:**  
  Emitted when a user successfully joins a gallery via ticket purchase.

### Errors

- **InvalidParameter:**  
  Raised when parameters (e.g., empty name, invalid voting times) do not meet the required conditions.

- **DeniedAccess:**  
  Triggered if an unauthorized address attempts to perform restricted operations.

- **InSufficientAllowance:**  
  For cases where the user's allowance is not enough (though not elaborated here, it’s defined for potential future use).

- **NoData:**  
  Emitted when there is no available data for a requested user or gallery query.

---

## Core Functions 📚

### Gallery Creation

- **`create_gallery(...)`**  
  Creates a new gallery with the following parameters:
  - `name` and `meta_data`: Must not be empty.
  - `price`: Ticket price for joining the gallery.
  - `voting_start` & `voting_end`: Define the voting period; must be set in the future and `voting_start` must be before `voting_end`.
  - `minimum_staking_amount`: Minimum amount required for staking.
  
  **Flow:**  
  1. Validate input parameters.
  2. Increment the `available_index` (starting from 1 to minimize potential errors).
  3. Save the new gallery details.
  4. Update the creator's state (tracking the created gallery).
  5. Automatically give the creator a ticket.
  6. Log the `NewGallery` event.

### Ticket Purchase

- **`buy_ticket(gallery_index, user)`**  
  Allows a user to buy a ticket for a specified gallery.  
  **Flow:**  
  1. Verify that the caller is the authorized (allowed) contract.
  2. Update the gallery’s attendee count.
  3. Record the gallery in the user’s joined galleries.
  4. Mark the user as having a valid ticket in `ticket_index`.
  5. Emit the `JoinedGallery` event.

### Admin Function

- **`set_a_c(cn_address)`**  
  Sets the allowed contract address that is permitted to execute ticket purchases.
  **Flow:**  
  1. If the admin is not yet set, assign the caller as admin.
  2. If already set, ensure that only the admin can change the allowed contract.
  3. Update the allowed contract address.

### View Functions

- **`get_last_index()`**  
  Retrieves the last used gallery index.

- **`get_gallery(gallery_index)`**  
  Returns detailed information about a specific gallery, including owner, name, metadata, attendance, creation time, price, and voting conditions.

- **`get_user_status(gallery_index, user)`**  
  Checks if a user has a valid ticket for a specified gallery.

- **`in_session(gallery_index)`**  
  Checks if the current time falls within the voting period of the gallery.

- **`get_mim_s_a(gallery_index)`**  
  Retrieves the minimum staking amount required for a gallery.

- **`get_uc(index, user, state)`**  
  Gets a specific gallery from the user's created or joined list based on the `state` parameter (0 for created, 1 for joined).

- **`get_len_uc(user, state)`**  
  Returns the number of galleries a user has created or joined.

---

## Usage Examples 💡

### Creating a Gallery

```rust
subject_instance.create_gallery(
    "My Art Gallery".into(),
    "A collection of exclusive art pieces".into(),
    U256::from(1000),        // Ticket price
    1680000000,              // Voting start timestamp
    1680003600,              // Voting end timestamp
    U256::from(500)          // Minimum staking amount
)?;
```

### Buying a Ticket

This function must be called by the allowed contract:
```rust
subject_instance.buy_ticket(gallery_index, user_address)?;
```

### Setting the Allowed Contract (Admin Only)

```rust
subject_instance.set_a_c(allowed_contract_address)?;
```

### Viewing Gallery Details

```rust
let gallery_info = subject_instance.get_gallery(gallery_index)?;
```

---

## Conclusion 🎉

The Gallery Contract is a central piece of the **NovaValult** ecosystem, streamlining gallery creation, ticket sales, and user management. With robust error handling and detailed event logging, it ensures a transparent and secure environment for all users to engage in the artistic and community-driven experiences that NovaValult offers.

Embrace the future of decentralized gallery management with NovaValult! 🏛️🚀
