/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_sdk::{contracttype, Address, Env, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    CommissionRate,
    AntiSnipeTime,
    AuctionData(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BidData {
    pub buyer: Address,
    pub amount: i128,
    pub sniper: bool,
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

pub fn save_auction_data(env: &Env, seller: &Address, auction_data: &AuctionData) {
    env.storage()
        .instance()
        .set(&DataKey::AuctionData(seller.clone()), auction_data);
}

pub fn load_auction_data(env: &Env, seller: &Address) -> AuctionData {
    env.storage()
        .instance()
        .get(&DataKey::AuctionData(seller.clone()))
        .unwrap()
}

pub fn delete_auction_data(env: &Env, seller: &Address) {
    env.storage()
        .instance()
        .remove(&DataKey::AuctionData(seller.clone()))
}

pub fn has_auction_data(env: &Env, seller: &Address) -> bool {
    env.storage()
        .instance()
        .has(&DataKey::AuctionData(seller.clone()))
}

pub fn save_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, &admin);
}

pub fn load_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn save_anti_snipe_time(env: &Env, anti_snipe_time: u64) {
    env.storage()
        .instance()
        .set(&DataKey::AntiSnipeTime, &anti_snipe_time);
}

pub fn load_anti_snipe_time(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::AntiSnipeTime)
        .unwrap_or(0)
}

pub fn save_commission_rate(env: &Env, commission_rate: i128) {
    env.storage()
        .instance()
        .set(&DataKey::CommissionRate, &commission_rate);
}

pub fn load_commission_rate(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::CommissionRate)
        .unwrap_or(0)
}
