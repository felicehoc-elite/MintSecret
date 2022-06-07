use cosmwasm_std::{HumanAddr, Uint128, Decimal, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::snip721::{Trait};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
   pub white_members : Vec<HumanAddr>,
   pub admin : HumanAddr,
   pub total_supply:Uint128,
   pub maximum_count:Uint128,
   pub public_price:Uint128,
   pub private_price:Uint128,
   pub reward_wallet:Vec<Wallet>,
   pub presale_period:u64,
   pub presale_start:u64,
   pub denom : String,
   pub token_address:HumanAddr,
   pub token_contract_hash:String,
   pub check_minted : Vec<bool>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    
    Receive{sender:HumanAddr,from:HumanAddr,amount:Uint128,msg:Binary},
    SetTotalSupply{amount: Uint128},
    SetMaximumNft{amount:Uint128},
    SetPrice{public_price:Uint128,private_price:Uint128},
    SetRewardWallet{wallet : Vec<Wallet>},
    ChangeAdmin{address:HumanAddr},
    SetMintFlag {flag:bool},
    SetMintTime{presale_start:u64,presale_period:u64},
    SetWhiteUsers{members:Vec<HumanAddr>},
    AddWhiteUser{member:HumanAddr},
    SetNftAddress{nft_address:HumanAddr},
    SetTokenAddres{token_address:HumanAddr,token_contract_hash:String}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetStateInfo {},
    GetWhiteUsers{},
    GetUserInfo{address:HumanAddr},
   
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Wallet {
    pub address: HumanAddr,
    pub portion : Decimal
}


#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct MetadataMsg {
    pub tokenId:Option<String>,
     /// name of the item
    pub name: Option<String>,
    /// item description
    pub description: Option<String>,
    /// item attributes
    pub attributes: Option<Vec<Trait>>,
   
    /// url to the image
    pub image: Option<String>,  
    /// a select list of trait_types that are in the private metadata.  This will only ever be used
    /// in public metadata
    pub protected_attributes: Option<Vec<String>>,
    pub code_hash: Option<String>,
    pub number:Option<i32>
}

