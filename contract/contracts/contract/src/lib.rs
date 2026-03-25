#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, Vec
};

#[contracttype]
#[derive(Clone)]
pub struct Listing {
    pub seller: Address,
    pub price: i128,
    pub sold: bool,
}

#[contract]
pub struct CampusBookContract;

const LISTING_COUNT: Symbol = symbol_short!("COUNT");

#[contractimpl]
impl CampusBookContract {

    // Đăng bán sách
    pub fn create_listing(env: Env, seller: Address, price: i128) -> u32 {
        seller.require_auth();

        let mut count: u32 = env.storage().instance()
            .get(&LISTING_COUNT)
            .unwrap_or(0);

        let listing = Listing {
            seller: seller.clone(),
            price,
            sold: false,
        };

        env.storage().instance().set(&count, &listing);

        count += 1;
        env.storage().instance().set(&LISTING_COUNT, &count);

        count - 1
    }

    // Mua sách (escrow giữ tiền)
    pub fn buy_book(env: Env, buyer: Address, id: u32) {
        buyer.require_auth();

        let mut listing: Listing = env.storage()
            .instance()
            .get(&id)
            .unwrap();

        if listing.sold {
            panic!("Already sold");
        }

        // ở MVP chỉ đánh dấu sold (giả lập escrow)
        listing.sold = true;

        env.storage().instance().set(&id, &listing);
    }

    // Xác nhận nhận sách → chuyển tiền cho seller
    pub fn confirm_receive(env: Env, id: u32) {

        let listing: Listing = env.storage()
            .instance()
            .get(&id)
            .unwrap();

        // logic chuyển token sẽ thêm ở production
        // env.invoke_contract(token,...)

        env.storage().instance().remove(&id);
    }

    pub fn get_listing(env: Env, id: u32) -> Listing {
        env.storage().instance().get(&id).unwrap()
    }
}