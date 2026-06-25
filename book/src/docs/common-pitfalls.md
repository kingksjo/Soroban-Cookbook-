# Common Pitfalls in Soroban Smart Contracts

This guide outlines the most common mistakes developers make when writing smart contracts for Soroban on Stellar. It covers both beginner-level issues (basic API usage, Rust quirks, and entry point rules) and intermediate-level design flaws (storage architecture, state archival, and security vulnerabilities).

---

## 🟢 Beginner Pitfalls

### 1. Missing Authorization Checks
* **Mistake:** Passing an `Address` to a function and performing state changes or asset transfers on its behalf without verifying its authorization.
* **Consequence:** Anyone can call the contract function and input another user's address, allowing them to unauthorizedly spend funds or manipulate state.
* **How to avoid/fix:** Always invoke `.require_auth()` on the `Address` parameter representing the signer.
* **Code Example:**
  ```rust
  // ❌ BAD: Anyone can call this function and pass another user's address
  pub fn withdraw(env: Env, user: Address, amount: i128) {
      let mut balance = get_balance(&env, &user);
      balance -= amount;
      set_balance(&env, &user, balance);
  }

  // ✅ GOOD: Verifies that the transaction was signed by the user
  pub fn withdraw(env: Env, user: Address, amount: i128) {
      user.require_auth();
      let mut balance = get_balance(&env, &user);
      balance -= amount;
      set_balance(&env, &user, balance);
  }
  ```

### 2. Unchecked Arithmetic (Integer Overflow/Underflow)
* **Mistake:** Using standard mathematical operators (`+`, `-`, `*`, `/`) for calculating token balances or values.
* **Consequence:** In release builds, overflow checks might wrap around silently (if they are disabled in the profile), leading to exploit vectors such as minting massive numbers of tokens. Even if checks are enabled, standard operators panic without providing a clear error code, making error handling impossible.
* **How to avoid/fix:** Use checked arithmetic methods (`checked_add`, `checked_sub`, `checked_mul`, `checked_div`) and return custom, descriptive contract errors.
* **Code Example:**
  ```rust
  // ❌ BAD: Standard operator can cause panics or wrap-around exploits
  pub fn add_interest(balance: u64, rate: u64) -> u64 {
      balance + (balance * rate) / 100
  }

  // ✅ GOOD: Checked math returning custom error types
  pub fn add_interest(balance: u64, rate: u64) -> Result<u64, Error> {
      let interest = balance
          .checked_mul(rate)
          .ok_or(Error::ArithmeticOverflow)?
          .checked_div(100)
          .ok_or(Error::ArithmeticOverflow)?;
      
      balance.checked_add(interest).ok_or(Error::ArithmeticOverflow)
  }
  ```

### 3. Exceeding the 32-Character Limit on Custom Symbols
* **Mistake:** Creating a `Symbol` (often used as storage keys) with a string length greater than 32 characters.
* **Consequence:** The Soroban VM enforces a strict 32-character limit on symbols. Calling `Symbol::new(&env, "...")` with a string longer than 32 characters will result in a runtime panic.
* **How to avoid/fix:** Keep symbols short and descriptive. For symbols of 9 characters or less, use `symbol_short!` which is stored more efficiently.
* **Code Example:**
  ```rust
  // ❌ BAD: Panics at runtime (44 characters)
  let key = Symbol::new(&env, "user_registration_timestamp_for_active_pool");

  // ✅ GOOD: Short symbol or type-safe enum key (see intermediate section)
  let key = Symbol::new(&env, "user_reg_time");
  // Or:
  let key = symbol_short!("reg_time");
  ```

### 4. Ignoring Result/Return Values on Safe Token Operations
* **Mistake:** Calling transfer functions or contract client actions that return `Result` values and ignoring them via `let _ = ...` or not asserting success.
* **Consequence:** The contract execution proceeds assuming the operation succeeded, even if it failed silently or returned an error payload, causing state inconsistency.
* **How to avoid/fix:** Always handle results using Rust's `?` operator or explicitly match on them. Note that SEP-41 token operations panic on failure, but custom token logic or cross-contract invocations might return `Result`.
* **Code Example:**
  ```rust
  // ❌ BAD: Ignored return value could cause transaction to proceed unexpectedly
  let result = token_client.try_transfer(&from, &to, &amount);
  // execution continues anyway...

  // ✅ GOOD: Explicitly check result and bubble up custom error
  token_client.try_transfer(&from, &to, &amount)
      .map_err(|_| Error::TokenTransferFailed)?;
  ```

---

## 🟡 Intermediate Pitfalls

### 1. Unbounded Storage Growth (Gas Exhaustion)
* **Mistake:** Appending items to a single shared `Vec` or `Map` under one storage key to keep track of all users or transactions.
* **Consequence:** Every time the collection is read or written, the entire collection must be deserialized and serialized. As the number of users grows, the transaction CPU and memory gas usage increases linearly. Eventually, transactions will exceed the ledger limits, permanently bricking the contract and locking user funds.
* **How to avoid/fix:** Store user-specific data under separate, unique keys. Use a type-safe `#[contracttype]` enum to represent storage key variants.
* **Code Example:**
  ```rust
  // ❌ BAD: Storing a growing list of users in a single vector under one key
  pub fn register_user(env: Env, user: Address) {
      let mut users: Vec<Address> = env.storage().instance()
          .get(&symbol_short!("users"))
          .unwrap_or_else(|| Vec::new(&env));
      users.push_back(user);
      env.storage().instance().set(&symbol_short!("users"), &users);
  }

  // ✅ GOOD: Storing individual user flags under separate, scoped keys
  #[contracttype]
  pub enum DataKey {
      UserRegistered(Address),
  }

  pub fn register_user(env: Env, user: Address) {
      let key = DataKey::UserRegistered(user);
      env.storage().persistent().set(&key, &true);
  }
  ```

### 2. State Archival Lockout (Storage Type & TTL Selection)
* **Mistake:** Storing user balances in `instance` storage or failing to extend the Time-To-Live (TTL) of `persistent` storage keys.
* **Consequence:**
  1. `instance` storage has limited size constraints. Storing user data here can cause the storage to fill up. Additionally, if the `instance` key expires, the contract is locked for *all* users and requires a costly restoration process.
  2. If `persistent` storage keys are not periodically extended, they will expire and be archived. Users will lose access to their data/funds until a manual, costly on-chain restoration transaction is submitted.
* **How to avoid/fix:**
  1. Use `instance` storage only for configuration and admin addresses.
  2. Use `persistent` storage for user balances and user-specific structs.
  3. Regularly extend the TTL of keys when they are accessed using `extend_ttl`.
* **Code Example:**
  ```rust
  #[contracttype]
  pub enum DataKey {
      Balance(Address),
  }

  // ✅ GOOD: Access balance from persistent storage and extend its TTL
  pub fn get_balance_and_bump(env: Env, user: Address) -> i128 {
      let key = DataKey::Balance(user);
      let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
      
      // Bump TTL: if the key has less than 10,000 ledgers remaining,
      // extend its lifespan to 50,000 ledgers from the current ledger.
      env.storage().persistent().extend_ttl(&key, 10000, 50000);
      
      balance
  }
  ```

### 3. Reentrancy and the Checks-Effects-Interactions Pattern
* **Mistake:** Changing local contract state *after* invoking an external contract (such as a token transfer to a user).
* **Consequence:** The external contract could intercept control and call back into this contract (reenter) before the local state is updated, leading to double-withdrawals.
* **How to avoid/fix:** Follow the **Checks-Effects-Interactions** pattern: perform validation checks first, write and persist local state changes next (effects), and invoke external contract calls last (interactions).
* **Code Example:**
  ```rust
  // ❌ BAD: Draining external tokens before updating internal state
  pub fn claim_rewards(env: Env, user: Address) {
      user.require_auth();
      let rewards = get_pending_rewards(&env, &user);
      
      let token_client = token::Client::new(&env, &get_token_id(&env));
      token_client.transfer(&env.current_contract_address(), &user, &rewards); // Interaction
      
      // DANGER: Reentrant call could hijack control before this line executes!
      set_pending_rewards(&env, &user, 0); // Effect
  }

  // ✅ GOOD: State update occurs before the external transfer is initiated
  pub fn claim_rewards(env: Env, user: Address) {
      user.require_auth();
      
      let rewards = get_pending_rewards(&env, &user);
      if rewards <= 0 {
          panic!("No rewards to claim"); // Check
      }
      
      set_pending_rewards(&env, &user, 0); // Effect
      
      let token_client = token::Client::new(&env, &get_token_id(&env));
      token_client.transfer(&env.current_contract_address(), &user, &rewards); // Interaction
  }
  ```

### 4. Custom Signature Replay Attacks
* **Mistake:** Verifying custom cryptographic signatures (e.g. Ed25519 or secp256k1) directly without binding the payload to the specific contract instance, network, and sequential transaction nonces.
* **Consequence:** The signature can be intercepted by an attacker and replayed on other contracts, different networks (e.g., testnet vs. mainnet), or multiple times on the same contract to execute unauthorized actions.
* **How to avoid/fix:** Use native Soroban authorization (`require_auth`) whenever possible, as it manages nonces and replay protection automatically. If custom verification is absolutely necessary, construct a signed payload containing:
  1. The contract's unique address (`env.current_contract_address()`).
  2. The network's passphrase ID (`env.ledger().network_id()`).
  3. A unique, incrementing user nonce.
* **Code Example:**
  ```rust
  // ❌ BAD: Signature payload contains only action data, vulnerable to replays
  pub fn execute_custom_auth(env: Env, public_key: BytesN<32>, signature: BytesN<64>, action_data: Bytes) {
      env.crypto().ed25519_verify(&public_key, &action_data, &signature);
  }

  // ✅ GOOD: Signed payload binds contract address, network, and nonce
  pub fn execute_custom_auth(
      env: Env, 
      user: Address, 
      public_key: BytesN<32>, 
      signature: BytesN<64>, 
      action_data: Bytes
  ) {
      let nonce = get_and_increment_nonce(&env, &user);
      
      let mut signed_payload = Bytes::new(&env);
      signed_payload.append(&env.current_contract_address().to_xdr_bytes());
      signed_payload.append(&env.ledger().network_id().to_xdr_bytes());
      signed_payload.append(&nonce.to_xdr_bytes());
      signed_payload.append(&action_data);
      
      env.crypto().ed25519_verify(&public_key, &signed_payload, &signature);
  }
  ```
