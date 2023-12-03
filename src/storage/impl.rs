/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use core::marker::PhantomData;
use soroban_sdk::{
    storage::{Instance, Persistent, Temporary},
    Env, IntoVal, TryFromVal, Val,
};

pub fn with_instance_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(StorageType, &Instance) -> T,
{
    f(StorageType::Instance, &env.storage().instance())
}

pub fn with_persistent_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(StorageType, &Persistent) -> T,
{
    f(StorageType::Persistent, &env.storage().persistent())
}

pub fn with_temporary_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(StorageType, &Temporary) -> T,
{
    f(StorageType::Temporary, &env.storage().temporary())
}

pub struct KeyedData<K, T>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone,
{
    key: K,
    _phantom: PhantomData<*const T>,
}

impl<K, T> KeyedData<K, T>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone,
{
    pub fn new(key: K) -> Self {
        KeyedData {
            key,
            _phantom: PhantomData,
        }
    }

    pub fn get_key(&self) -> &K {
        &self.key
    }
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

pub trait StorageData<T> {
    fn load(&self, env: &Env) -> Option<T>;
    fn save(&self, env: &Env, data: &T);
    fn delete(&self, env: &Env);
    fn has(&self, env: &Env) -> bool;
    fn bump(&self, env: &Env, low_expiration_watermark: u64, hi_expiration_watermark: u64);
    fn get_storage_type(env: &Env) -> StorageType;
}

#[macro_export]
macro_rules! impl_storage_data {
    ($data_type:ty, $storage_func:expr) => {
        impl<K> $crate::StorageData<$data_type> for $crate::KeyedData<K, $data_type>
        where
            K: soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
                + soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>
                + Clone,
        {
            fn get_storage_type(env: &soroban_sdk::Env) -> $crate::StorageType {
                $storage_func(env, |storage_type, _storage| storage_type)
            }

            fn load(&self, env: &soroban_sdk::Env) -> Option<$data_type> {
                $storage_func(env, |_storage_type, storage| storage.get(self.get_key()))
                    .map(|data| data)
            }

            fn save(&self, env: &soroban_sdk::Env, data: &$data_type) {
                $storage_func(env, |_storage_type, storage| {
                    storage.set(self.get_key(), data)
                });
            }

            fn delete(&self, env: &soroban_sdk::Env) {
                $storage_func(env, |_storage_type, storage| storage.remove(self.get_key()));
            }

            fn has(&self, env: &soroban_sdk::Env) -> bool {
                $storage_func(env, |_storage_type, storage| storage.has(self.get_key()))
            }

            fn bump(
                &self,
                env: &soroban_sdk::Env,
                low_expiration_watermark: u64,
                hi_expiration_watermark: u64,
            ) {
                match Self::get_storage_type(&env) {
                    $crate::StorageType::Instance => {
                        $crate::with_instance_storage(env, |_storage_type, storage| {
                            storage.bump(
                                low_expiration_watermark as u32,
                                hi_expiration_watermark as u32,
                            );
                        });
                    }
                    $crate::StorageType::Persistent => {
                        $crate::with_persistent_storage(env, |_storage_type, storage| {
                            storage.bump(
                                self.get_key(),
                                low_expiration_watermark as u32,
                                hi_expiration_watermark as u32,
                            );
                        });
                    }
                    $crate::StorageType::Temporary => {
                        $crate::with_temporary_storage(env, |_storage_type, storage| {
                            storage.bump(
                                self.get_key(),
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
    ($data_type:ty, Instance) => {
        $crate::impl_storage_data!($data_type, $crate::with_instance_storage);
    };
    ($data_type:ty, Persistent) => {
        $crate::impl_storage_data!($data_type, $crate::with_persistent_storage);
    };
    ($data_type:ty, Temporary) => {
        $crate::impl_storage_data!($data_type, $crate::with_temporary_storage);
    };
}

pub fn load_data<T, K>(env: &Env, key: &K) -> T
where
    KeyedData<K, T>: StorageData<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone + ?Sized,
{
    KeyedData::<K, T>::new(key.clone()).load(env).unwrap()
}

pub fn load_data_or_else<T, K, F, R>(env: &Env, key: &K, handler: F) -> R
where
    KeyedData<K, T>: StorageData<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone + ?Sized,
    F: FnOnce(Option<T>) -> R,
{
    handler(KeyedData::<K, T>::new(key.clone()).load(env))
}

pub fn save_data<T, K>(env: &Env, key: &K, data: &T)
where
    KeyedData<K, T>: StorageData<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone + ?Sized,
{
    KeyedData::<K, T>::new(key.clone()).save(env, data);
}

pub fn has_data<T, K>(env: &Env, key: &K) -> bool
where
    KeyedData<K, T>: StorageData<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone + ?Sized,
{
    KeyedData::<K, T>::new(key.clone()).has(env)
}

pub fn delete_data<T, K>(env: &Env, key: &K)
where
    KeyedData<K, T>: StorageData<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone + ?Sized,
{
    KeyedData::<K, T>::new(key.clone()).delete(env);
}

pub fn bump_data<T, K>(
    env: &Env,
    key: &K,
    low_expiration_watermark: u64,
    hi_expiration_watermark: u64,
) where
    KeyedData<K, T>: StorageData<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone + ?Sized,
{
    KeyedData::<K, T>::new(key.clone()).bump(
        env,
        low_expiration_watermark,
        hi_expiration_watermark,
    );
}