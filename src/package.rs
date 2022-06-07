use schemars:: JsonSchema;
use serde:: { Deserialize, Serialize };
use crate::msg::Metadata;
use cosmwasm_std::{HumanAddr};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  MintNft { token_id: Option<String>,
        /// optional owner address. if omitted, owned by the message sender
        owner: Option<HumanAddr>,
        /// optional public metadata that can be seen by everyone
        public_metadata: Option<Metadata>,
        /// optional private metadata that can only be seen by the owner and whitelist
        private_metadata: Option<Metadata>,
        /// optional serial number for this token
        serial_number: Option<SerialNumber>,
        /// optional royalty information for this token.  This will be ignored if the token is
        /// non-transferable
        royalty_info: Option<RoyaltyInfo>,
        /// optionally true if the token is transferable.  Defaults to true if omitted
        transferable: Option<bool>,
        /// optional memo for the tx
        memo: Option<String>,
        /// optional message length padding
        padding: Option<String>, },
}

/// Serial number to give an NFT when minting
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SerialNumber {
    /// optional number of the mint run this token will be minted in.  A mint run represents a
    /// batch of NFTs released at the same time.  So if a creator decided to make 100 copies
    /// of an NFT, they would all be part of mint run number 1.  If they sold quickly, and
    /// the creator wanted to rerelease that NFT, he could make 100 more copies which would all
    /// be part of mint run number 2.
    pub mint_run: Option<u32>,
    /// serial number (in this mint run).  This is used to serialize
    /// identical NFTs
    pub serial_number: u32,
    /// optional total number of NFTs minted on this run.  This is used to
    /// represent that this token is number m of n
    pub quantity_minted_this_run: Option<u32>,
}

/// data for a single royalty
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Royalty {
    /// address to send royalties to
    pub recipient: HumanAddr,
    /// royalty rate
    pub rate: u16,
}

/// all royalty information
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct RoyaltyInfo {
    /// decimal places in royalty rates
    pub decimal_places_in_rates: u8,
    /// list of royalties
    pub royalties: Vec<Royalty>,
}

