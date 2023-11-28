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

pub fn with_instance_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Instance) -> T,
{
    let storage = env.storage().instance();
    f(&storage)
}

pub fn with_persistent_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Persistent) -> T,
{
    let storage = env.storage().persistent();
    f(&storage)
}

pub fn with_temporary_storage<F, T>(env: &Env, f: F) -> T
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

mod ledger_times {
    // Assuming 6 seconds average time per ledger.
    pub const LEDGERS_PER_MINUTE: u64 = 10;
    pub const LEDGERS_PER_HOUR: u64 = LEDGERS_PER_MINUTE * 60;
    pub const LEDGERS_PER_DAY: u64 = LEDGERS_PER_HOUR * 24;
    pub const LEDGERS_PER_YEAR: u64 = LEDGERS_PER_DAY * 365;
}

pub fn bump_data<T>(
    env: &Env,
    key: &DataKey,
    low_expiration_watermark: u64,
    hi_expiration_watermark: u64,
    in_seconds: bool,
) where
    T: StorageTypeInfo + StorageData,
{
    fn seconds_to_ledgers(watermark: u64) -> u64 {
        watermark
            .checked_add(ledger_times::LEDGERS_PER_MINUTE - 1)
            .and_then(|sum| sum.checked_div(ledger_times::LEDGERS_PER_MINUTE))
            .expect("Invalid duration.")
            .min(ledger_times::LEDGERS_PER_YEAR)
    }

    let (lo_exp, hi_exp) = if !in_seconds {
        (low_expiration_watermark, hi_expiration_watermark)
    } else {
        (
            seconds_to_ledgers(low_expiration_watermark),
            seconds_to_ledgers(hi_expiration_watermark),
        )
    };

    match T::get_storage_type() {
        StorageType::Instance => {
            with_instance_storage(env, |storage| {
                storage.bump(lo_exp as u32, hi_exp as u32);
            });
        }
        StorageType::Persistent => {
            with_persistent_storage(env, |storage| {
                storage.bump(key, lo_exp as u32, hi_exp as u32);
            });
        }
        StorageType::Temporary => {
            with_temporary_storage(env, |storage| {
                storage.bump(key, lo_exp as u32, hi_exp as u32);
            });
        }
    }
}
