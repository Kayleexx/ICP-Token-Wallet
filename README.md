
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

