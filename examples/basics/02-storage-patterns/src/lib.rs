//! Storage Patterns Contract
//!
//! Demonstrates the three types of storage available in Soroban:
//! - Persistent: Data that lives long-term and requires TTL management
//! - Temporary: Data that only exists for the current ledger
//! - Instance: Data tied to the contract instance lifetime (shared TTL)

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Persistent(Symbol),
    Temporary(Symbol),
    Instance(Symbol),
}

/// Storage contract demonstrating all three storage types
#[contract]
pub struct StorageContract;

#[contractimpl]
impl StorageContract {
    // ==================== PERSISTENT STORAGE ====================

    /// Stores a value in persistent storage and extends its TTL.
    pub fn set_persistent(env: Env, key: Symbol, value: u64) {
        let storage_key = DataKey::Persistent(key.clone());
        env.storage().persistent().set(&storage_key, &value);
        // Extend TTL so data survives ledger advances in tests
        env.storage().persistent().extend_ttl(&storage_key, 100, 1000);
        env.events()
            .publish((symbol_short!("persist"), symbol_short!("set")), (key, value));
    }

    /// Retrieves a value from persistent storage.
    /// Returns `Some(value)` if present, or `None`.
    pub fn get_persistent(env: Env, key: Symbol) -> Option<u64> {
        env.storage().persistent().get(&DataKey::Persistent(key))
    }

    /// Checks if a key exists in persistent storage.
    pub fn has_persistent(env: Env, key: Symbol) -> bool {
        env.storage().persistent().has(&DataKey::Persistent(key))
    }

    /// Removes a value from persistent storage and emits an event.
    pub fn remove_persistent(env: Env, key: Symbol) {
        env.storage().persistent().remove(&DataKey::Persistent(key.clone()));
        env.events()
            .publish((symbol_short!("persist"), symbol_short!("remove")), key);
    }

    // ==================== TEMPORARY STORAGE ====================

    /// Stores a value in temporary storage.
    pub fn set_temporary(env: Env, key: Symbol, value: u64) {
        env.storage().temporary().set(&DataKey::Temporary(key.clone()), &value);
        env.events()
            .publish((symbol_short!("temp"), symbol_short!("set")), (key, value));
    }

    /// Retrieves a value from temporary storage.
    /// Returns `Some(value)` if present, or `None`.
    pub fn get_temporary(env: Env, key: Symbol) -> Option<u64> {
        env.storage().temporary().get(&DataKey::Temporary(key))
    }

    /// Checks if a key exists in temporary storage.
    pub fn has_temporary(env: Env, key: Symbol) -> bool {
        env.storage().temporary().has(&DataKey::Temporary(key))
    }

    // ==================== INSTANCE STORAGE ====================

    /// Stores a value in instance storage and extends the instance TTL.
    pub fn set_instance(env: Env, key: Symbol, value: u64) {
        env.storage().instance().set(&DataKey::Instance(key.clone()), &value);
        env.storage().instance().extend_ttl(1000, 10000);
        env.events()
            .publish((symbol_short!("instance"), symbol_short!("set")), (key, value));
    }

    /// Retrieves a value from instance storage.
    /// Returns `Some(value)` if present, or `None`.
    pub fn get_instance(env: Env, key: Symbol) -> Option<u64> {
        env.storage().instance().get(&DataKey::Instance(key))
    }

    /// Checks if a key exists in instance storage.
    pub fn has_instance(env: Env, key: Symbol) -> bool {
        env.storage().instance().has(&DataKey::Instance(key))
    }

    /// Removes a value from instance storage and emits an event.
    pub fn remove_instance(env: Env, key: Symbol) {
        env.storage().instance().remove(&DataKey::Instance(key.clone()));
        env.events()
            .publish((symbol_short!("instance"), symbol_short!("remove")), key);
    }
}

#[cfg(test)]
mod test;
