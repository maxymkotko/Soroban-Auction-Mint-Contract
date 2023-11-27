/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_sdk::{contracttype, storage::{Instance, Persistent}, Address, Env, Vec};

fn with_instance_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Instance) -> T,
{
    let storage = env.storage().instance();
    f(&storage)
}

fn _with_persistent_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Persistent) -> T,
{
    let storage = env.storage().persistent();
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

impl_storage_data!(AdminData, with_instance_storage);
impl_storage_data!(AuctionData, with_instance_storage);

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

pub trait StorageData {
    fn save(&self, env: &Env, key: &DataKey);
    fn load(env: &Env, key: &DataKey) -> Self where Self: Sized;
    fn delete(env: &Env, key: &DataKey);
    fn has(env: &Env, key: &DataKey) -> bool;
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    AdminData,
    AuctionData(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BidData {
    pub buyer: Address,
    pub amount: i128,
    pub sniper: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdminData {
    pub admin: Address,
    pub anti_snipe_time: u64,
    pub commission_rate: i128,
    pub extendable_auctions: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct AuctionData {
    pub token: Address,
    pub amount: i128,
    pub duration: u64,
    pub start_time: u64,
    pub market: Address,
    pub reserve_price: i128,
    pub ask_price: i128,
    pub discount_percent: u32,
    pub discount_frequency: u64,
    pub compounded_discount: bool,
    pub bids: Vec<BidData>,
}
