# Basic Examples

Core Soroban fundamentals, one concept per example. Perfect for beginners starting their journey with Soroban smart contracts.

## 🚀 Suggested Learning Path

Follow these examples in order to build a solid foundation in Soroban development:

1.  **The Basics**: Start with [01-hello-world](./01-hello-world/) and [09-primitive-types](./09-primitive-types/) to understand contract structure and basic Rust types.
2.  **Data Modeling**: Explore [06-soroban-types](./06-soroban-types/), [07-enum-types](./07-enum-types/), and [08-custom-structs](./08-custom-structs/) to learn how to represent data.
3.  **Storage Layers**: Dive into [instance-storage](./instance-storage/), [persistent-storage](./persistent-storage/), and [temporary_storage](./temporary_storage/) to understand how data persists on-chain.
4.  **Interactivity**: Learn about [basic-event-emission](./basic-event-emission/) and [03-custom-errors](./03-custom-errors/) to communicate with the outside world and handle failures.
5.  **Advanced Fundamentals**: Master [03-authentication](./03-authentication/) and [06-validation-patterns](./06-validation-patterns/) to build secure and robust contracts.

---

## 📋 Example Index

| # | Example | Difficulty | Description | Key Concepts |
|:---:|:---|:---:|:---|:---|
| 1 | [01-hello-world](./01-hello-world/) | ⭐ | The "Hello World" of Soroban. | `#[contract]`, `Symbol`, Tests |
| 2 | [09-primitive-types](./09-primitive-types/) | ⭐ | Integer types and overflow safety. | `u32`, `i128`, Arithmetic safety |
| 3 | [06-soroban-types](./06-soroban-types/) | ⭐ | Built-in Soroban types. | `Address`, `Symbol`, `Bytes` |
| 4 | [10-data-types](./10-data-types/) | ⭐ | Comprehensive type exploration. | Data representation |
| 5 | [06-type-conversions](./06-type-conversions/) | ⭐⭐ | Secure type casting and conversion. | `Into`, `From`, `TryInto` |
| 6 | [07-enum-types](./07-enum-types/) | ⭐ | Using enums in contract logic. | Enums, Pattern matching |
| 7 | [08-custom-structs](./08-custom-structs/) | ⭐ | Complex data structures. | Structs, `#[contracttype]` |
| 8 | [11-collection-types](./11-collection-types/) | ⭐⭐ | Working with `Vec` and `Map`. | Collections, Iteration |
| 9 | [instance-storage](./instance-storage/) | ⭐⭐ | Shared contract-instance storage. | Instance storage, TTL |
| 10 | [persistent-storage](./persistent-storage/) | ⭐⭐ | Long-term data persistence. | Persistent storage, Keys |
| 11 | [temporary_storage](./temporary_storage/) | ⭐⭐ | Cost-optimized transient data. | Temporary storage, TTL mgmt |
| 12 | [02-storage-patterns](./02-storage-patterns/) | ⭐⭐⭐ | Advanced storage management. | Combined storage layers |
| 13 | [basic-event-emission](./basic-event-emission/) | ⭐ | Simple event publishing. | `env.events().publish()` |
| 14 | [events](./events/) | ⭐ | General event counter example. | State changes, Events |
| 15 | [04-events](./04-events/) | ⭐⭐ | Structured event topics and design. | Topic indexing, Layouts |
| 16 | [11-event-filtering](./11-event-filtering/) | ⭐⭐⭐ | Complex multi-topic filters. | Advanced event queries |
| 17 | [03-custom-errors](./03-custom-errors/) | ⭐⭐ | Custom contract error enums. | `#[contracterror]` |
| 18 | [05-error-handling](./05-error-handling/) | ⭐⭐⭐ | Propagation and validation patterns. | Result, Panic vs Return |
| 19 | [03-authentication](./03-authentication/) | ⭐⭐ | Authorization with `require_auth()`. | Auth, Addresses, Roles |
| 20 | [05-auth-context](./05-auth-context/) | ⭐⭐⭐ | Cross-contract execution context. | Invoker, Contract address |
| 21 | [06-validation-patterns](./06-validation-patterns/) | ⭐⭐⭐ | Security and validation best practices. | Preconditions, State gating |


## 📋 Planned Examples

- **Iterative Mappings** - Efficient iteration over large data sets.
- **Batch Processing** - Handling multiple operations in a single call.
- **State Machine Patterns** - Structured state transitions for complex logic.

## 🎯 Prerequisites

Before diving into these examples, ensure you have:
- [Set up your development environment](../../guides/getting-started.md)
- [Read the Testing Guide](../../guides/testing.md)
- A basic understanding of Rust programming.

## 🧪 Running Tests

```bash
# From the root directory
cargo test -p [package-name]

# Example:
cargo test -p hello-world
```
