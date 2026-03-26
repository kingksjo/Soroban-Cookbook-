# Security Fundamentals Guide for Soroban

This guide provides a practical foundation for building secure smart contracts on Soroban. It covers threat modeling, common vulnerability classes, mitigation strategies, and a secure development workflow.

## 1. Threat Model Basics

Understanding the security landscape of your contract is the first step toward securing it.

### What’s at Stake?
*   **Tokens & Assets:** User funds, protocol liquidity, and escrowed assets.
*   **Contract State:** Administrative configurations, access control lists, and user balances.
*   **Reputation:** Trust in your protocol and the broader Stellar ecosystem.

### Potential Attackers
*   **Malicious Users:** Individuals looking to exploit bugs for financial gain.
*   **Malicious Contracts:** External contracts that may interact with yours in unexpected ways.
*   **Front-runners:** Actors who monitor the mempool to exploit transaction ordering (though Soroban's fee model and transaction ordering mitigate some of this).

### Attack Surfaces
*   **Public Functions:** Every exported function in your `[contractimpl]` is a potential entry point.
*   **External Calls:** Interacting with other contracts (e.g., tokens, oracles) introduces external dependencies.
*   **State Transitions:** Any logic that modifies storage is a critical point for maintaining invariants.

---

## 2. High-Risk Vulnerability Classes

### 1. Unauthorized Access / Missing Authorization
**What it is:** Sensitive functions that can be called by anyone, or incorrect verification of the caller.
**How it happens:** Forgetting to call `address.require_auth()` or checking the wrong address.
**Mitigation:**
*   Always call `require_auth()` for any action that shouldn't be public.
*   Use the "Admin" pattern for protocol-level configurations.

### 2. Reentrancy / Unexpected Call Flows
**What it is:** An external contract call that re-enters your contract before the first execution finished, potentially exploiting inconsistent state.
**How it happens:** Making an external call (like a token transfer) before updating your own internal state.
**Mitigation:**
*   Follow the **Checks-Effects-Interactions** pattern.
*   Update all internal state *before* calling external contracts.

### 3. Integer Overflow / Underflow
**What it is:** Arithmetic operations that exceed the maximum or minimum value of a type.
**How it happens:** Using standard operators (`+`, `-`, `*`) on user-provided amounts without checks.
**Mitigation:**
*   Use `checked_*` methods (e.g., `checked_add`, `checked_sub`).
*   Handle the `None` case explicitly or use `.expect()` with a clear error message.

### 4. State Inconsistency
**What it is:** A contract reaching a state that violates its core business logic (invariants).
**How it happens:** Partial updates where one part of the state is updated but another fails or is skipped.
**Mitigation:**
*   Ensure atomic updates.
*   Use custom error types to roll back state changes (Soroban transactions are atomic).

### 5. Input Validation Issues
**What it is:** Accepting parameters that are out of bounds or logically invalid.
**How it happens:** Assuming `amount > 0` or that a `Vec` isn't empty without checking.
**Mitigation:**
*   Explicitly validate all arguments at the start of the function.
*   Check for zero values, maximum limits, and empty collections.

### 6. Denial of Service (DoS)
**What it is:** Making a contract unusable by consuming all resources or locking it.
**How it happens:** Unbounded loops, excessive storage writes, or allowing users to "squat" on critical resources.
**Mitigation:**
*   Avoid loops over user-controlled data.
*   Use appropriate storage types (Temporary storage for non-critical, high-churn data).

### 7. Precision / Rounding Errors
**What it is:** Losing value during calculations, especially in DeFi.
**How it happens:** Dividing before multiplying or using low-precision types.
**Mitigation:**
*   **Multiply before dividing** to maintain precision.
*   Use large integer types (e.g., `i128`) for intermediate calculations.

---

## 3. Mitigation Checklist

Use this checklist before every deployment:

- [ ] **Authorization:** Do all sensitive functions have `require_auth()`?
- [ ] **Arithmetic:** Are all mathematical operations using `checked_*` methods?
- [ ] **Input Validation:** Are all arguments checked for validity (range, non-zero, etc.)?
- [ ] **External Calls:** Are all external calls made *after* internal state updates?
- [ ] **Storage:** Are we using the correct storage type (Persistent vs. Instance vs. Temporary)?
- [ ] **Resources:** Are there any unbounded loops or expensive operations?
- [ ] **Invariants:** Does the contract maintain its core logic even if a call fails?
- [ ] **Events:** Are all state-changing operations emitting events for transparency?

---

## 4. Secure Development Workflow

1.  **Threat Model:** Identify what you are protecting and who might attack it.
2.  **Safe Design:** Use established patterns (e.g., the [Token interface](https://soroban.stellar.org/docs/reference/interfaces/token-interface)).
3.  **Implementation:** Write idiomatic Rust using the Soroban SDK.
4.  **Self-Review:** Go through the [Mitigation Checklist](#3-mitigation-checklist).
5.  **Testing:**
    *   **Unit Tests:** Test individual logic pieces.
    *   **Edge Cases:** Test zero amounts, maximum values, and unauthorized calls.
    *   **Integration Tests:** Test how your contract interacts with others.
6.  **Audit:** For high-value contracts, seek a professional third-party audit.

---

## 5. Soroban-Specific Examples

### Authorization
❌ **Bad:**
```rust
pub fn withdraw(env: Env, from: Address, amount: i128) {
    let balance = read_balance(&env, &from);
    write_balance(&env, &from, balance - amount);
}
```

✅ **Good:**
```rust
pub fn withdraw(env: Env, from: Address, amount: i128) {
    from.require_auth(); // Critical check!
    let balance = read_balance(&env, &from);
    let new_balance = balance.checked_sub(amount).expect("Insufficient balance");
    write_balance(&env, &from, new_balance);
}
```

### Arithmetic
❌ **Bad:**
```rust
let total = price * balance; // Potential overflow
```

✅ **Good:**
```rust
let total = price.checked_mul(balance).expect("Price overflow");
```

### State Updates (Checks-Effects-Interactions)
❌ **Bad:**
```rust
pub fn claim_reward(env: Env, user: Address) {
    user.require_auth();
    let reward = calculate_reward(&env, &user);
    token_client.transfer(&env.current_contract_address(), &user, &reward);
    // DANGER: State updated AFTER external call
    set_reward_paid(&env, &user);
}
```

✅ **Good:**
```rust
pub fn claim_reward(env: Env, user: Address) {
    user.require_auth();
    let reward = calculate_reward(&env, &user);
    
    // Update state FIRST
    set_reward_paid(&env, &user);
    
    // Interact LAST
    token_client.transfer(&env.current_contract_address(), &user, &reward);
}
```
