use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::msg::{Wallet, MetadataMsg};

use cosmwasm_std::{Storage, Uint128, HumanAddr,StdResult};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton,bucket,bucket_read};
// use cw_storage_plus::Map;

pub static CONFIG_KEY: &[u8] = b"config";
pub static CONFIG_RANDOM: &[u8] = b"config_random";
pub static CONFIG_METADATA : &[u8] = b"config_metadata";
pub static CONFIG_MEMBERS: &[u8] = b"config_members";
pub const CONFIG_USERS: &[u8] = b"User";

// pub const USERS: Map<&str, Vec<String>> = Map::new("User");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: Uint128,
    pub total_supply:Uint128,
    pub admin: HumanAddr,
    pub maximum_count:Uint128,
    pub public_price:Uint128,
    pub private_price:Uint128,
    pub reward_wallet:Vec<Wallet>,
    pub public_mint:bool,
    pub private_mint : bool,
    pub nft_address:HumanAddr,
    pub nft_contract_hash:String,
    pub token_address:HumanAddr,
    pub token_contract_hash:String,
    pub check_minted : Vec<bool>
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn save_rand<S: Storage>(storage: &mut S) -> Singleton<S, u16> {
    singleton(storage, CONFIG_RANDOM)
}

pub fn read_rand<S: Storage>(storage: &S) -> ReadonlySingleton<S, u16> {
    singleton_read(storage, CONFIG_RANDOM)
}

pub fn save_metadata<S: Storage>(storage: &mut S) -> Singleton<S, Vec<MetadataMsg>> {
    singleton(storage, CONFIG_METADATA)
}

pub fn read_metadata<S: Storage>(storage: &S) -> ReadonlySingleton<S, Vec<MetadataMsg>> {
    singleton_read(storage, CONFIG_METADATA)
}


pub fn store_members<S: Storage>(storage: &mut S) -> Singleton<S, Vec<HumanAddr>> {
    singleton(storage, CONFIG_MEMBERS)
}

pub fn read_members<S: Storage>(storage: &S) -> ReadonlySingleton<S, Vec<HumanAddr>> {
    singleton_read(storage, CONFIG_MEMBERS)
}

pub fn store_user_info<S: Storage>(storage: &mut S, user: &str, user_info: Vec<String>) -> StdResult<()> {
    bucket(CONFIG_USERS, storage).save(user.as_bytes(), &user_info)
}

pub fn read_user_info<S: Storage>(storage: &S, user: &str) -> Option<Vec<String>> {
    match bucket_read(CONFIG_USERS, storage).load(user.as_bytes()) {
        Ok(v) => Some(v),
        _ => None,
    }
}

