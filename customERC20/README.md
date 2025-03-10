# Custom ERC20 Contract: Nova Vault Token 🚀

This smart contract is a custom ERC20 implementation built on Arbitrum Stylus Rust. It manages the **Nova Vault (NovaV)** tokens and extends the standard ERC20 functionalities with additional features, including token purchasing, admin-controlled minting, and market management. Below is an in-depth overview of the contract's components, functions, events, and error handling.

---

## Overview 📝

- **Token Details:**  
  - **Name:** Nova Vault  
  - **Symbol:** NovaV  
  - **Decimals:** 10

- **Purpose:**  
  This contract not only implements the basic ERC20 functions (like transferring tokens, approving allowances, and checking balances) but also includes:
  - A `buy()` function allowing users to purchase Nova tokens with ETH.
  - Admin functionalities for minting tokens and setting market parameters.
  - A built-in mechanism to manage token sales, including market limits and pricing adjustments.

- **Built Using:**  
  Arbitrum Stylus Rust, leveraging the robust and secure environment provided by the Stylus SDK.

---

## Key Features ✨

1. **Standard ERC20 Functions:**
   - **name(), symbol(), decimals(), total_supply(), balance_of()**  
     Retrieve immutable token properties and account balances.

2. **Token Transfers:**
   - **transfer() & transfer_from()**  
     Enable users to send tokens to one another, with proper allowance checks and balance updates.

3. **Approval Mechanism:**
   - **approve() & allowance()**  
     Allow users to grant spending rights on their tokens, with event logging for transparency.

4. **Admin Functions:**
   - **set_mint()**  
     Manage which addresses are allowed to mint new tokens.
   - **set_token_price() & set_market()**  
     Adjust token pricing and available market supply.
   - **mint(), mint_to() & burn()**  
     Mint new tokens or burn existing tokens under controlled conditions.

5. **Token Sale:**
   - **BUY()** (payable function)  
     Users can purchase Nova tokens by sending ETH. The function checks for sufficient funds, market supply, and logs the token sale event.

6. **Internal Safety Checks:**
   - **_transfer(), _mint(), _burn()**  
     Handle the core logic of token movement and supply changes.
   - **check_admin()**  
     Ensures that admin-only functions are accessible only by authorized addresses.

---

## Contract Structure & Components 🔧

### Storage Variables
- **balances:**  
  A mapping from addresses to their token balances.
- **allowances:**  
  Nested mappings to track approved spenders for each address.
- **total_supply:**  
  The current total supply of Nova tokens.
- **admin & allow_admin:**  
  Admin control for managing mint permissions and other privileged functions.
- **price & market:**  
  Variables to store the token's price in ETH and the available tokens in the market for sale.

### Events & Errors
- **Events:**
  - `Transfer` – Logs token transfers.
  - `Approval` – Logs approval actions.
  - `TokenSold` – Logs details of token purchases.
  - `SetMarket` & `SetPrice` – Log changes in market supply and pricing.
  
- **Errors:**
  - `InsufficientBalance`  
  - `InsufficientAllowance`  
  - `InvalidParameter`  
  - `InsufficientFunds`  
  - `MarketExceeded`  
  - `Unauthorized`

These events and errors ensure that every significant action and failure state is logged, helping with debugging and transparent on-chain activity monitoring.

---

## How to Use 📚

### Basic Token Operations

- **Query Token Details:**
  ```rust
  let token_name = Erc20::name();
  let token_symbol = Erc20::symbol();
  let decimals = Erc20::decimals();
  let totalSupply = instance.total_supply();
  ```
  
- **Check Balance:**
  ```rust
  let userBalance = instance.balance_of(user_address);
  ```

- **Token Transfer:**
  ```rust
  instance.transfer(recipient_address, amount)?;
  ```

- **Approve & Transfer From:**
  ```rust
  instance.approve(spender_address, amount);
  instance.transfer_from(owner_address, recipient_address, amount)?;
  ```

### Admin & Minting Operations

- **Setting Admin Permissions:**
  ```rust
  instance.set_mint(vec![user_address1, user_address2], vec![true, false])?;
  ```

- **Adjusting Token Price & Market:**
  ```rust
  instance.set_token_price(new_price)?;
  instance.set_market(new_market_supply)?;
  ```

- **Minting Tokens:**
  ```rust
  // Admin minting tokens to self or another address
  instance.mint(amount)?;
  instance.mint_to(recipient_address, amount)?;
  ```

- **Burning Tokens:**
  ```rust
  instance.burn(amount)?;
  ```

### Buying Tokens

- **Purchasing Tokens with ETH:**
  ```rust
  // This function is payable and requires sending ETH along with the call.
  instance.BUY()?;
  ```
  The `BUY()` function checks:
  - If the ETH sent is sufficient based on the current token price.
  - If the market has enough tokens available.
  - Then it deducts the purchased tokens from the market supply, mints new tokens, and logs the sale.

---

## Conclusion 🎉

This custom ERC20 contract for Nova Vault is designed to provide a robust and secure token system tailored to the unique needs of the Nova Value platform. Its extended functionalities—such as direct token purchases, admin-controlled minting, and market management—enhance the standard ERC20 features, ensuring a flexible and dynamic ecosystem for users and creators alike.

Whether you’re a developer looking to integrate Nova tokens into your dApp or an admin managing token economics, this contract offers the tools and transparency required for a next-generation digital asset platform.

---

Happy coding and welcome to the future of tokenized value! 🚀💎
