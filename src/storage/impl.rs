/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_sdk::{
    storage::{Instance, Persistent, Temporary},
    Env, IntoVal, TryFromVal, Val,
};

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

pub trait StorageData<K>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn load(env: &Env, key: &K) -> Self
    where
        Self: Sized;
    fn try_load(env: &Env, key: &K) -> Result<Self, StorageError>
    where
        Self: Sized;
    fn save(&self, env: &Env, key: &K);
    fn delete(&self, env: &Env, key: &K);
    fn has(&self, env: &Env, key: &K) -> bool;
    fn bump(&self, env: &Env, key: &K, low_expiration_watermark: u64, hi_expiration_watermark: u64);
}

#[macro_export]
macro_rules! impl_storage_data {
    ($type:ty, $storage_func:expr) => {
        impl<K> $crate::StorageData<K> for $type
        where
            K: soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
                + soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>,
        {
            fn load(env: &soroban_sdk::Env, key: &K) -> Self {
                $storage_func(env, |storage| storage.get(key).unwrap())
            }

            fn try_load(env: &soroban_sdk::Env, key: &K) -> Result<Self, $crate::StorageError> {
                match $storage_func(env, |storage| storage.get(key)) {
                    Some(data) => Ok(data),
                    None => Err($crate::StorageError::DataNotFound),
                }
            }

            fn save(&self, env: &soroban_sdk::Env, key: &K) {
                $storage_func(env, |storage| storage.set(key, self));
            }

            fn delete(&self, env: &soroban_sdk::Env, key: &K) {
                $storage_func(env, |storage| storage.remove(key));
            }

            fn has(&self, env: &soroban_sdk::Env, key: &K) -> bool {
                $storage_func(env, |storage| storage.has(key))
            }

            fn bump(
                &self,
                env: &soroban_sdk::Env,
                key: &K,
                low_expiration_watermark: u64,
                hi_expiration_watermark: u64,
            ) {
                match <$type as $crate::StorageTypeInfo>::get_storage_type() {
                    $crate::StorageType::Instance => {
                        $crate::with_instance_storage(env, |storage| {
                            storage.bump(
                                low_expiration_watermark as u32,
                                hi_expiration_watermark as u32,
                            );
                        });
                    }
                    $crate::StorageType::Persistent => {
                        $crate::with_persistent_storage(env, |storage| {
                            storage.bump(
                                key,
                                low_expiration_watermark as u32,
                                hi_expiration_watermark as u32,
                            );
                        });
                    }
                    $crate::StorageType::Temporary => {
                        $crate::with_temporary_storage(env, |storage| {
                            storage.bump(
                                key,
                                low_expiration_watermark as u32,
                                hi_expiration_watermark as u32,
                            );
                        });
                    }
                }
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
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    T::try_load(env, key).is_ok()
}

pub fn delete_data<T, K>(env: &Env, key: &K)
where
    T: StorageData<K>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    T::load(env, key).delete(env, key);
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
    T::load(env, key).bump(env, key, low_expiration_watermark, hi_expiration_watermark);
}

pub enum StorageError {
    DataNotFound,
}

#[allow(dead_code)]
pub enum StorageType {
    Instance,
    Persistent,
    Temporary,
}

pub trait StorageTypeInfo {
    fn get_storage_type() -> StorageType;
}

#[allow(dead_code)]
pub struct KeyedData<T, K> {
    pub data: T,
    pub key: K,
}

#[allow(dead_code)]
impl<T, K> KeyedData<T, K>
where
    T: StorageData<K> + StorageTypeInfo,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone,
{
    pub fn new(data: T, key: K) -> Self {
        KeyedData { data, key }
    }

    pub fn save(&self, env: &Env) {
        self.data.save(env, &self.key);
    }

    pub fn has(&self, env: &Env) -> bool {
        self.data.has(env, &self.key)
    }

    pub fn delete(&self, env: &Env) {
        self.data.delete(env, &self.key);
    }

    pub fn bump(&self, env: &Env, low_expiration_watermark: u64, hi_expiration_watermark: u64) {
        self.data.bump(
            env,
            &self.key,
            low_expiration_watermark,
            hi_expiration_watermark,
        );
    }

    pub fn get(&self) -> &T {
        &self.data
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }
}
