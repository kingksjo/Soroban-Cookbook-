# Intermediate Examples

This category contains examples that demonstrate common, real-world design patterns and use cases for Soroban smart contracts. These examples often combine multiple basic concepts to solve practical problems.

## What's Inside?

- **Access Control**: Implement patterns like multi-sig, Role-Based Access Control (RBAC), and timelocks.
- **Cross-Contract Communication**: See how to build systems with factory, proxy, and registry patterns.
- **Token Interactions**: Learn how to create contracts that interact with or wrap standard tokens.
- **Advanced Data Structures**: Examples of iterable maps, queues, and other complex data structures.
- **Emergency Controls**: Pause/unpause pattern for halting sensitive operations.

## Implemented Examples

- [`multi-sig-patterns`](./multi-sig-patterns/) — Threshold signatures and multi-party auth
- [`ajo-factory`](./ajo-factory/) — Contract deployment from within a contract
- [`03-pause-unpause`](./03-pause-unpause/) — Emergency pause/unpause mechanism

## Planned Examples

- `02-role-based-access-control`: An RBAC implementation for managing permissions.
- `04-token-wrapper`: A contract that wraps a standard token to add new functionality.
- `05-upgradable-proxy`: A basic proxy pattern for contract upgradability.
