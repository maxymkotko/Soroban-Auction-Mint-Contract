/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

pub use super::types::*;
use soroban_sdk::{
    storage::{Instance, Persistent, Temporary},
    Env,
};

// Usage Example:
// 
// Use the impl_storage! macro to specify the contract storage for the type.
// impl_storage!(SomeContractType, Persistent);
// 
// Manage data using generics:
// - e.g. load data: load_data::<SomeContractType>(&env, &key);
// 
// Or, directly call methods on the instance:
// - e.g. save data: some_contract_type_instance.save(&env, &key);
// 

fn with_instance_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Instance) -> T,
{
    let storage = env.storage().instance();
    f(&storage)
}

fn with_persistent_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Persistent) -> T,
{
    let storage = env.storage().persistent();
    f(&storage)
}

fn with_temporary_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Temporary) -> T,
{
    let storage = env.storage().temporary();
    f(&storage)
}

macro_rules! impl_storage_data {
    ($type:ty, $storage_func:expr) => {
        impl StorageData for $type {
            fn save(&self, env: &Env, key: &DataKey) {
                $storage_func(env, |storage| storage.set(key, self));
            }

            fn load(env: &Env, key: &DataKey) -> Self {
                $storage_func(env, |storage| storage.get(key).unwrap())
            }

            fn delete(env: &Env, key: &DataKey) {
                $storage_func(env, |storage| storage.remove(key));
            }

            fn has(env: &Env, key: &DataKey) -> bool {
                $storage_func(env, |storage| storage.has(key))
            }
        }
    };
}

macro_rules! impl_storage {
    ($type:ty, Instance) => {
        impl StorageTypeInfo for $type {
            fn get_storage_type() -> StorageType {
                StorageType::Instance
            }
        }
        impl_storage_data!($type, with_instance_storage);
    };
    ($type:ty, Persistent) => {
        impl StorageTypeInfo for $type {
            fn get_storage_type() -> StorageType {
                StorageType::Persistent
            }
        }
        impl_storage_data!($type, with_persistent_storage);
    };
    ($type:ty, Temporary) => {
        impl StorageTypeInfo for $type {
            fn get_storage_type() -> StorageType {
                StorageType::Temporary
            }
        }
        impl_storage_data!($type, with_temporary_storage);
    };
}

pub fn load_data<T: StorageData>(env: &Env, key: &DataKey) -> T {
    T::load(env, key)
}

pub fn has_data<T: StorageData>(env: &Env, key: &DataKey) -> bool {
    T::has(env, key)
}

pub fn delete_data<T: StorageData>(env: &Env, key: &DataKey) {
    T::delete(env, key)
}

pub fn save_data<T: StorageData>(env: &Env, key: &DataKey, data: &T) {
    data.save(env, key)
}

pub fn bump_data<T>(env: &Env, key: &DataKey, low_exp_watermark: u32, hi_exp_watermark: u32)
where
    T: StorageTypeInfo + StorageData,
{
    match T::get_storage_type() {
        StorageType::Instance => {
            with_instance_storage(env, |storage| {
                storage.bump(low_exp_watermark, hi_exp_watermark);
            });
        }
        StorageType::Persistent => {
            with_persistent_storage(env, |storage| {
                storage.bump(key, low_exp_watermark, hi_exp_watermark);
            });
        }
        StorageType::Temporary => {
            with_temporary_storage(env, |storage| {
                storage.bump(key, low_exp_watermark, hi_exp_watermark);
            });
        }
    }
}

// Implement AdminData with Instance storage.
impl_storage!(AdminData, Instance);

// Implement AuctionData with Persistent storage.
impl_storage!(AuctionData, Persistent);
