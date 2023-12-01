/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_sdk::{contracttype, Address, Vec};

use crate::impl_storage;

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

// Implement AdminData with Instance storage.
impl_storage!(AdminData, Instance);

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

// Implement AuctionData with Persistent storage.
impl_storage!(AuctionData, Persistent);