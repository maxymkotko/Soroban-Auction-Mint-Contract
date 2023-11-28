/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_sdk::{contracttype, Address, Env, Vec};

pub trait StorageData {
    fn save(&self, env: &Env, key: &DataKey);
    fn load(env: &Env, key: &DataKey) -> Self where Self: Sized;
    fn delete(env: &Env, key: &DataKey);
    fn has(env: &Env, key: &DataKey) -> bool;
}

pub trait StorageTypeInfo {
    fn get_storage_type() -> StorageType;
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

#[allow(dead_code)]
pub enum StorageType {
    Instance,
    Persistent,
    Temporary,
}

mod ledger_times {
    pub const LEDGERS_PER_MINUTE: u64 = 10; // Assuming 6 seconds average time per ledger.
    pub const LEDGERS_PER_HOUR: u64 = LEDGERS_PER_MINUTE * 60;
    pub const LEDGERS_PER_DAY: u64 = LEDGERS_PER_HOUR * 24;
    pub const LEDGERS_PER_YEAR: u64 = LEDGERS_PER_DAY * 365;
}
pub use ledger_times::*;