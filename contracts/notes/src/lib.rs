#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec
};

// =======================
// 📦 STRUCT IPHONE
// =======================
#[contracttype]
#[derive(Clone, Debug)]
pub struct Iphone {
    id: u64,
    model: String,
    price_per_day: u64,
    status: String, // Available | Rented | Maintenance
}

// =======================
// 👤 STRUCT CUSTOMER
// =======================
#[contracttype]
#[derive(Clone, Debug)]
pub struct Customer {
    id: u64,
    name: String,
}

// =======================
// 📄 STRUCT RENTAL
// =======================
#[contracttype]
#[derive(Clone, Debug)]
pub struct Rental {
    id: u64,
    iphone_id: u64,
    customer_id: u64,
    days: u32,
    total_price: u64,
    status: String, // Ongoing | Returned
}

// =======================
// 🔑 STORAGE KEYS
// =======================
const IPHONE_DATA: Symbol = symbol_short!("IPHONE");
const CUSTOMER_DATA: Symbol = symbol_short!("CUST");
const RENTAL_DATA: Symbol = symbol_short!("RENT");

// =======================
// 🚀 CONTRACT
// =======================
#[contract]
pub struct IRentContract;

#[contractimpl]
impl IRentContract {

    // =======================
    // 📱 CREATE IPHONE
    // =======================
    pub fn add_iphone(env: Env, model: String, price_per_day: u64) -> String {
        let mut iphones: Vec<Iphone> =
            env.storage().instance().get(&IPHONE_DATA).unwrap_or(Vec::new(&env));

        let phone = Iphone {
            id: env.prng().gen::<u64>(),
            model,
            price_per_day,
            status: String::from_str(&env, "Available"),
        };

        iphones.push_back(phone);
        env.storage().instance().set(&IPHONE_DATA, &iphones);

        String::from_str(&env, "📱 iPhone berhasil ditambahkan")
    }

    // =======================
    // 👤 CREATE CUSTOMER
    // =======================
    pub fn add_customer(env: Env, name: String) -> String {
        let mut customers: Vec<Customer> =
            env.storage().instance().get(&CUSTOMER_DATA).unwrap_or(Vec::new(&env));

        let customer = Customer {
            id: env.prng().gen::<u64>(),
            name,
        };

        customers.push_back(customer);
        env.storage().instance().set(&CUSTOMER_DATA, &customers);

        String::from_str(&env, "👤 Customer berhasil ditambahkan")
    }

    // =======================
    // 📥 GET ALL IPHONES
    // =======================
    pub fn get_iphones(env: Env) -> Vec<Iphone> {
        env.storage().instance().get(&IPHONE_DATA).unwrap_or(Vec::new(&env))
    }

    // =======================
    // 🔎 GET AVAILABLE IPHONES
    // =======================
    pub fn get_available_iphones(env: Env) -> Vec<Iphone> {
        let iphones = Self::get_iphones(env.clone());
        let mut result = Vec::new(&env);

        for i in 0..iphones.len() {
            let phone = iphones.get(i).unwrap();
            if phone.status == String::from_str(&env, "Available") {
                result.push_back(phone);
            }
        }

        result
    }

    // =======================
    // 📄 CREATE RENTAL
    // =======================
    pub fn rent_iphone(env: Env, iphone_id: u64, customer_id: u64, days: u32) -> String {
        let mut iphones: Vec<Iphone> =
            env.storage().instance().get(&IPHONE_DATA).unwrap_or(Vec::new(&env));

        let mut rentals: Vec<Rental> =
            env.storage().instance().get(&RENTAL_DATA).unwrap_or(Vec::new(&env));

        // cari iphone
        for i in 0..iphones.len() {
            let mut phone = iphones.get(i).unwrap();

            if phone.id == iphone_id {
                if phone.status != String::from_str(&env, "Available") {
                    return String::from_str(&env, "❌ iPhone tidak tersedia");
                }

                let total_price = phone.price_per_day * days as u64;

                let rental = Rental {
                    id: env.prng().gen::<u64>(),
                    iphone_id,
                    customer_id,
                    days,
                    total_price,
                    status: String::from_str(&env, "Ongoing"),
                };

                phone.status = String::from_str(&env, "Rented");

                iphones.set(i, phone);
                rentals.push_back(rental);

                env.storage().instance().set(&IPHONE_DATA, &iphones);
                env.storage().instance().set(&RENTAL_DATA, &rentals);

                return String::from_str(&env, "✅ Berhasil menyewa iPhone");
            }
        }

        String::from_str(&env, "❌ iPhone tidak ditemukan")
    }

    // =======================
    // 🔄 RETURN IPHONE
    // =======================
    pub fn return_iphone(env: Env, rental_id: u64) -> String {
        let mut rentals: Vec<Rental> =
            env.storage().instance().get(&RENTAL_DATA).unwrap_or(Vec::new(&env));

        let mut iphones: Vec<Iphone> =
            env.storage().instance().get(&IPHONE_DATA).unwrap_or(Vec::new(&env));

        for i in 0..rentals.len() {
            let mut rental = rentals.get(i).unwrap();

            if rental.id == rental_id {
                rental.status = String::from_str(&env, "Returned");

                // ubah status iphone
                for j in 0..iphones.len() {
                    let mut phone = iphones.get(j).unwrap();

                    if phone.id == rental.iphone_id {
                        phone.status = String::from_str(&env, "Available");
                        iphones.set(j, phone);
                        break;
                    }
                }

                rentals.set(i, rental);

                env.storage().instance().set(&RENTAL_DATA, &rentals);
                env.storage().instance().set(&IPHONE_DATA, &iphones);

                return String::from_str(&env, "🔄 iPhone berhasil dikembalikan");
            }
        }

        String::from_str(&env, "❌ Rental tidak ditemukan")
    }

    // =======================
    // 📊 STATS
    // =======================
    pub fn get_stats(env: Env) -> (u32, u32) {
        let iphones = Self::get_iphones(env.clone());

        let mut available = 0;
        let mut rented = 0;

        for i in 0..iphones.len() {
            let phone = iphones.get(i).unwrap();

            if phone.status == String::from_str(&env, "Available") {
                available += 1;
            } else {
                rented += 1;
            }
        }

        (available, rented)
    }
}

mod test;