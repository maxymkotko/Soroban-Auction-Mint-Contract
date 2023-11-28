/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

//! The `storage` module provides interfaces with generic storage functions
//! to manage contract data with Soroban Instance, Persistent, and Temporary storage.
//! 
//! Usage Example:
//! 
//! Use the `impl_storage!` macro to specify the contract storage for the type.
//! - e.g., call: `impl_storage!(SomeContractType, Persistent);`
//! 
//! Manage data using generics:
//! - e.g., load data: `load_data::<SomeContractType>(&env, &key);`
//! 
//! Or, directly call methods on the instance:
//! - e.g., save data: `some_contract_type_instance.save(&env, &key);`
//!
//! Extending data lifetime can be done in seconds or in ledgers.
//! - e.g., in seconds: `bump_data::<SomeContractType>(&env, &key, 3600, 7200, true);`
//! - e.g., in ledgers: `bump_data::<SomeContractType>(&env, &key, 600, 1200, false);`


#[macro_use]
pub mod r#impl;
pub mod types;

pub use r#impl::*;
use soroban_sdk::Env;

// Implement AdminData with Instance storage.
impl_storage!(AdminData, Instance);

// Implement AuctionData with Persistent storage.
impl_storage!(AuctionData, Persistent);