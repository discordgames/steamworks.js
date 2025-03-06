use napi::bindgen_prelude::*;
use napi_derive::napi;
use steamworks::SteamId;

use crate::client;

#[napi]
pub mod friends {
    use super::*;

    #[napi]
    pub fn get_small_friend_avatar(steam_id_64: BigInt) -> Option<Buffer> {
        let client = client::get_client();
        let (_, value, _) = steam_id_64.get_u64();
        let steam_id = SteamId::from_raw(value);
        
        // Get the friend object
        let friend = client.friends().get_friend(steam_id);
        
        // Get the avatar data
        if let Some(avatar_data) = friend.small_avatar() {
            return Some(Buffer::from(avatar_data));
        }
        
        None
    }

    #[napi]
    pub fn get_medium_friend_avatar(steam_id_64: BigInt) -> Option<Buffer> {
        let client = client::get_client();
        let (_, value, _) = steam_id_64.get_u64();
        let steam_id = SteamId::from_raw(value);
        
        // Get the friend object
        let friend = client.friends().get_friend(steam_id);
        
        // Get the avatar data
        if let Some(avatar_data) = friend.medium_avatar() {
            return Some(Buffer::from(avatar_data));
        }
        
        None
    }

    #[napi]
    pub fn get_large_friend_avatar(steam_id_64: BigInt) -> Option<Buffer> {
        let client = client::get_client();
        let (_, value, _) = steam_id_64.get_u64();
        let steam_id = SteamId::from_raw(value);
        
        // Get the friend object
        let friend = client.friends().get_friend(steam_id);
        
        // Get the avatar data
        if let Some(avatar_data) = friend.large_avatar() {
            return Some(Buffer::from(avatar_data));
        }
        
        None
    }
} 