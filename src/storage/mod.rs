/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

//! Provides generic functions and traits to facilitate
//! contract data interaction with `soroban_sdk` storage types.
//! 
//! Usage Example:
//! 
//! Use the `impl_soroban_storage!` macro to implement the desired storage for the soroban-sdk contracttype:
//!   ```rust
//!     #[contracttype]
//!     pub enum Key {
//!         User(Address),
//!     }
//!
//!     #[contracttype]
//!      pub struct Data {
//!          pub token: Address,
//!      }
//! 
//!      // Example: Implementing Soroban Instance storage for Data
//!      impl_soroban_storage!(Data, Instance);
//!      // Example: Implementing Soroban Persistent storage for Data
//!      impl_soroban_storage!(Data, Persistent);
//!      // Example: Implementing Soroban Temporary storage for Data
//!      impl_soroban_storage!(Data, Temporary);
//!   ```
//! 
//! You can now manage Data instance storage with generics:
//!   ```rust
//!      let key = Key::User(Address::random(&env)); 
//!      // Example: Saving some data to Soroban storage.
//!      save_data::<Key, Data>(&env, &key, &Data { token: Address::random(&env) });
//!      // Example: Loading data from Soroban storage.
//!      load_data::<Key, Data>(&env, &key);
//!      // Example: Loading with error tolerance.
//!      load_data_or_else::<Key, Data, _, _>(&env, &key, |opt| opt.unwrap_or_else(|| default_value()));
//!      // Example: Checking the Soroban storage has data.
//!      has_data::<Key, Data>(&env, &key);
//!      // Example: Bumping data lifetime.
//!      bump_data::<Key, Data>(&env, &key, 1, 1);
//!      // Example: Deleting the data from Soroban storage.
//!      delete_data::<Key, Data>(&env, &key);
//!   ```

#[macro_use]
mod r#impl;

pub use r#impl::*;


