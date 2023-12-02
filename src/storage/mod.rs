/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

//! The `storage` module provides generic functions and traits to
//! facilitate contract data interaction with `soroban_sdk` storage types.
//! 
//! Usage Example:
//! 
//! Use the `impl_storage!` macro to implement the desired storage for the data type:
//!   ```rust
//!   // Example: Implementing Persistent storage for DataType with DataKeyType.
//!   impl_storage!(DataType, DataKeyType, Persistent);
//!   ```
//! 
//! You can now manage your data with generics:
//!   ```rust
//!     // Example: Loading data from storage.
//!     let data = load_data::<DataType, DataKeyType>(&env, &key);
//!     // Example: Saving data to storage.
//!     save_data::<DataType, DataKeyType>(&env, &key, &data);
//!   ```


#[macro_use]
mod r#impl;

pub use r#impl::*;


