/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

#![no_std]

mod auctions;
mod storage;

use soroban_sdk::{contract, contractimpl, contractmeta, vec, Address, Env, Vec};
use storage::*;
use crate::auctions::{behavior::BaseAuction, behavior::Dispatcher};

contractmeta!(
    key="desc",
    val="Auction smart contract for the Litemint marketplace, implementing timed auctions with support for both ascending and descending price mechanisms.");

#[contract]
pub struct AuctionContract;

#[contractimpl]
impl AuctionContract {
    pub fn start(
        env: Env,
        seller: Address,
        token: Address,
        amount: i128,
        duration: u64,
        market: Address,
        reserve_price: i128,
        ask_price: i128,
        discount_percent: u32,
        discount_frequency: u64,
        compounded_discount: bool,
    ) {
        if !has_admin(&env) {
            panic!("Admin not set. Call initialize.");
        }

        seller.require_auth();

        let start_time = env.ledger().timestamp();
        let bids: Vec<BidData> = vec![&env];
        dispatcher!(discount_percent > 0 && discount_frequency > 0).start(
            &env,
            &seller,
            &AuctionData {
                token,
                amount,
                duration,
                start_time,
                market,
                reserve_price,
                ask_price,
                discount_percent,
                discount_frequency,
                compounded_discount,
                bids,
            },
        )
    }

    pub fn resolve(env: Env, seller: Address) {
        let auction_data = load_auction_data(&env, &seller);
        dispatcher!(auction_data.discount_percent > 0 && auction_data.discount_frequency > 0)
            .resolve(&env, &seller);
    }

    pub fn extend(env: Env, seller: Address, duration: u64) {
        seller.require_auth();

        let mut auction_data = load_auction_data(&env, &seller);
        auction_data.duration += duration;
        save_auction_data(&env, &seller, &auction_data);
    }

    pub fn place_bid(env: Env, seller: Address, buyer: Address, amount: i128) {
        buyer.require_auth();

        let auction_data = load_auction_data(&env, &seller);
        dispatcher!(auction_data.discount_percent > 0 && auction_data.discount_frequency > 0)
            .manage_bid(&env, &seller, &buyer, amount);
    }

    pub fn initialize(env: Env, admin: Address, anti_snipe_time: u64, commission_rate: i128) {
        if has_admin(&env) {
            panic!("Admin already set.");
        }

        save_admin(&env, &admin);
        save_anti_snipe_time(&env, anti_snipe_time.min(60));
        save_commission_rate(&env, commission_rate.max(0).min(100));
    }

    pub fn get_auction(env: Env, seller: Address) -> Option<AuctionData> {
        if has_auction_data(&env, &seller) {
            Some(load_auction_data(&env, &seller))
        }
        else {
            None
        }        
    }
}

#[cfg(test)]
mod test;
