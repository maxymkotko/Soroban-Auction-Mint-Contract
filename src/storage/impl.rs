/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_sdk::{
    storage::{Instance, Persistent, Temporary},
    Env, Val, IntoVal, TryFromVal,
};

pub trait StorageData<K>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn save(&self, env: &Env, key: &K);
    fn load(env: &Env, key: &K) -> Self
    where
        Self: Sized;
    fn delete(env: &Env, key: &K);
    fn has(env: &Env, key: &K) -> bool;
}

pub trait StorageTypeInfo {
    fn get_storage_type() -> StorageType;
}

#[allow(dead_code)]
pub enum StorageType {
    Instance,
    Persistent,
    Temporary,
}

pub fn with_instance_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Instance) -> T,
{
    f(&env.storage().instance())
}

pub fn with_persistent_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Persistent) -> T,
{
    f(&env.storage().persistent())
}

pub fn with_temporary_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Temporary) -> T,
{
    f(&env.storage().temporary())
}

#[macro_export]
macro_rules! impl_storage_data {
    ($type:ty, $storage_func:expr) => {
        impl<K> $crate::StorageData<K> for $type 
        where
            K: soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
            + soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>,
        {
            fn save(&self, env: &soroban_sdk::Env, key: &K) {
                $storage_func(env, |storage| storage.set(key, self));
            }

            fn load(env: &soroban_sdk::Env, key: &K) -> Self {
                $storage_func(env, |storage| storage.get(key).unwrap())
            }

            fn delete(env: &soroban_sdk::Env, key: &K) {
                $storage_func(env, |storage| storage.remove(key));
            }

            fn has(env: &soroban_sdk::Env, key: &K) -> bool {
                $storage_func(env, |storage| storage.has(key))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_storage {
    ($type:ty, Instance) => {
        impl $crate::StorageTypeInfo for $type {
            fn get_storage_type() -> $crate::StorageType {
                $crate::StorageType::Instance
            }
        }
        $crate::impl_storage_data!($type, $crate::with_instance_storage);
    };
    ($type:ty, Persistent) => {
        impl $crate::StorageTypeInfo for $type {
            fn get_storage_type() -> $crate::StorageType {
                $crate::StorageType::Persistent
            }
        }
        $crate::impl_storage_data!($type, $crate::with_persistent_storage);
    };
    ($type:ty, Temporary) => {
        impl $crate::StorageTypeInfo for $type {
            fn get_storage_type() -> $crate::StorageType {
                $crate::StorageType::Temporary
            }
        }
        $crate::impl_storage_data!($type, $crate::with_temporary_storage);
    };
}

pub fn load_data<T: StorageData<K>, K>(env: &Env, key: &K) -> T
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    T::load(env, key)
}

pub fn has_data<T, K>(env: &Env, key: &K) -> bool
where
    T: StorageData<K>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized, // Add the key type constraints
{
    T::has(env, key)
}

pub fn delete_data<T, K>(env: &Env, key: &K)
where
    T: StorageData<K>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    T::delete(env, key)
}

pub fn save_data<T, K>(env: &Env, key: &K, data: &T)
where
    T: StorageData<K>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    data.save(env, key)
}

pub fn bump_data<T, K>(
    env: &Env,
    key: &K,
    low_expiration_watermark: u64,
    hi_expiration_watermark: u64,
) where
    T: StorageTypeInfo + StorageData<K>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    match T::get_storage_type() {
        StorageType::Instance => {
            with_instance_storage(env, |storage| {
                storage.bump(low_expiration_watermark as u32, hi_expiration_watermark as u32);
            });
        }
        StorageType::Persistent => {
            with_persistent_storage(env, |storage| {
                storage.bump(key, low_expiration_watermark as u32, hi_expiration_watermark as u32);
            });
        }
        StorageType::Temporary => {
            with_temporary_storage(env, |storage| {
                storage.bump(key, low_expiration_watermark as u32, hi_expiration_watermark as u32);
            });
        }
    }
}

pub fn bump_data_conv<T, K, F>(
    env: &Env,
    key: &K,
    low_expiration_watermark: u64,
    hi_expiration_watermark: u64,
    conv_func: Option<F>,
) where
    T: StorageTypeInfo + StorageData<K>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
    F: Fn(u64) -> u64,
{
    let (lo_exp, hi_exp) = match conv_func {
        Some(convert) => (
            convert(low_expiration_watermark),
            convert(hi_expiration_watermark),
        ),
        None => (
            low_expiration_watermark,
            hi_expiration_watermark,
        ),
    };

    bump_data::<T, K>(
        env,
        key,
        lo_exp,
        hi_exp,
    );
}


