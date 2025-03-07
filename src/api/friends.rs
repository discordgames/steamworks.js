use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ffi::CString;
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

    #[napi]
    pub fn set_rich_presence(key: String, value: Option<String>) -> bool {
        let client = client::get_client();
        client.friends().set_rich_presence(&key, value.as_deref())
    }

    #[napi]
    pub fn clear_rich_presence() {
        let client = client::get_client();
        client.friends().clear_rich_presence();
    }
}
