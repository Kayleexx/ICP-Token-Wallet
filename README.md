
# ICP Token Wallet

A decentralized token wallet built on the Internet Computer Protocol (ICP) blockchain. This project allows users to manage tokens with features like minting, transferring, and querying balances. Written in Rust and integrated with the DFINITY SDK, it demonstrates the creation of secure and scalable token management on the ICP network.



## Prerequisites

- [Node.js](https://nodejs.org/) (v12.22.9)
- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove) (v0.24.2)
- Internet Computer Local Network

Ensure you have an identity set up with `dfx`:
```bash
dfx identity use <identity_name>
```

---

## Installation

1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd icp_token_wallet
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

---

## Project Structure

```plaintext
icp_token_wallet/
│
├── src/
│   └── icp_token_wallet_backend/
│       ├── src/
│       │   └── lib.rs          # Rust backend logic
│       └── icp_token_wallet_backend.did  # Candid interface
│
├── dfx.json                    # DFX configuration
└── package.json                # Project metadata
```

---

## Deploying the Canister

1. **Start the Local Network:**
   ```bash
   dfx start --background
   ```

2. **Create and Deploy the Canister:**
   ```bash
   dfx canister create icp_token_wallet_backend
   dfx build
   dfx canister install icp_token_wallet_backend
   ```

---

## Using the Canister

### Check Principal ID
To interact with the wallet, get your principal ID:
```bash
dfx identity get-principal
```

### Example Commands
1. **Query Balance:**
   ```bash
   dfx canister call icp_token_wallet_backend balance_of "(principal \"<your_principal_id>\")"
   ```
   Replace `<your_principal_id>` with your principal ID.

2. **Transfer Tokens:**
   ```bash
   dfx canister call icp_token_wallet_backend transfer "(principal \"<recipient_principal_id>\", <amount>)"
   ```

3. **Mint Tokens:**
   Only the owner can mint tokens:
   ```bash
   dfx canister call icp_token_wallet_backend mint "(principal \"<recipient_principal_id>\", <amount>)"
   ```

4. **Query Token Info:**
   ```bash
   dfx canister call icp_token_wallet_backend token_info
   ```

---

## Security Measures

Security is a key focus in the development of this token wallet to ensure the integrity of transactions and user data. Below are the key security measures implemented:

### 1. **Owner Control**
   - The wallet is initialized with a designated owner. Only the owner can mint new tokens.
   - This ensures that the supply of tokens is controlled and prevents unauthorized users from inflating the total supply.

### 2. **Access Control for Transactions**
   - For each transaction, the sender's balance is checked before transferring tokens to the recipient.
   - If the sender does not have enough tokens, the transaction is rejected.
   - This ensures that users cannot transfer more tokens than they possess, preventing issues such as negative balances.

### 3. **Principal-based Authorization**
   - Transactions are verified based on the `Principal` of the caller (sender). 
   - This ensures that only the intended user can interact with their own wallet, adding a layer of identity validation.

### 4. **Immutable Wallet State**
   - The wallet state is stored securely using the `RefCell` mechanism, making it thread-safe and ensuring that the data is not tampered with during updates.
   - This ensures that no unauthorized changes can be made to balances, token symbols, or other key properties without proper checks.

### 5. **Secure Token Minting**
   - The minting function ensures that only the designated owner of the wallet can create new tokens. 
   - This prevents unauthorized minting, ensuring that the token supply is under control and not subject to malicious manipulation.

### 6. **Data Privacy**
   - All wallet data, such as balances and token information, is stored in a decentralized manner on the ICP blockchain, ensuring that users' private information remains secure and immutable.
   
### 7. **Secure Encryption for Wallet Identity**
   - Wallet identities are encrypted to prevent unauthorized access and to ensure user data privacy.

### 8. **Smart Contract Safety**
   - The code leverages the DFINITY SDK's secure development environment, ensuring that no malicious code can be injected into the wallet contract.
   - Each contract call (such as `transfer`, `mint`, `balance_of`) is carefully implemented to avoid vulnerabilities such as reentrancy attacks, ensuring safe interactions with the blockchain.

---


## Available Methods

### `init`
Initializes the canister with the deployer's principal as the owner.

### `balance_of(owner: Principal) -> u64`
Returns the balance of the specified principal.

### `transfer(to: Principal, amount: u64) -> bool`
Transfers tokens from the caller to the specified recipient.

### `mint(to: Principal, amount: u64) -> bool`
Mints tokens to a specific principal. Only callable by the owner.

### `token_info() -> (String, String, u8, u64)`
Returns the token's metadata:
- Token name
- Symbol
- Decimal places
- Total supply

---

