/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_sdk::{Address, Env};
use crate::{storage::*, types::{AuctionData, DataKey}};

pub struct AscendingPriceAuction;

// AscendingPriceAuction (aka English Auction).
impl super::behavior::BaseAuction for AscendingPriceAuction {
    fn resolve(&self, env: &Env, seller: &Address) -> bool {
        let auction_data = load_data::<DataKey, AuctionData>(env, &DataKey::AuctionData(seller.clone()));

        // Retrieve the highest bid.
        if let Some(bid) = auction_data.bids.iter().max_by_key(|bid| bid.amount) {
            // Check that the reserve is met and
            // either the auction time has expired or the ask price is met.
            if bid.amount >= auction_data.reserve_price
                && (auction_data.start_time + auction_data.duration < env.ledger().timestamp()
                    || (auction_data.ask_price > auction_data.reserve_price
                        && bid.amount >= auction_data.ask_price)) {
                return self.finalize(env, seller, Some(&bid));
            }
        }
        else {
            // Auction has expired.
            if auction_data.start_time + auction_data.duration < env.ledger().timestamp() {
                return self.finalize(env, seller, None);    
            }
        }
        false
    }

    fn calculate_price(&self, env: &Env, seller: &Address) -> i128 {
        load_data::<DataKey, AuctionData>(env, &DataKey::AuctionData(seller.clone())).reserve_price
    }
}



