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
//! - e.g., call: `impl_storage!(DataType, DataKeyType, Persistent);`
//! 
//! Manage data using generics:
//! - e.g., load data: `load_data::<DataType, DataKeyType>(&env, &key);`
//! 
//! Or, directly call methods on the instance:
//! - e.g., save data: `data_type_instance.save(&env, &key);`
//!
//! A cutom conversion function can be provided with bump_data_conv.
//! - e.g., in seconds: `bump_data_conv::<SomeContractType, DataKeyType,
//!                         fn(u64) -> u64>(&env, &key, 3600, 7200, seconds_to_ledgers);`
//! Or, call bump_data:
//! - e.g., in ledgers: `bump_data::<SomeContractType, DataKeyType>(&env, &key, 600, 1200);`


#[macro_use]
mod r#impl;

pub use r#impl::*;


