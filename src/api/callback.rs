use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::Serialize;

// Define our own callback types for the ones that don't exist in steamworks-rs
mod custom_callbacks {
    use serde::Serialize;
    use std::os::raw::c_void;
    use steamworks::SteamId;

    #[derive(Serialize)]
    pub struct AvatarImageLoaded {
        pub steam_id: SteamId,
        pub image: i32,
        pub width: i32,
        pub height: i32,
    }

    unsafe impl steamworks::Callback for AvatarImageLoaded {
        const ID: i32 = 13;
        const SIZE: i32 = std::mem::size_of::<Self>() as i32;

        unsafe fn from_raw(raw: *mut c_void) -> Self {
            let raw = raw as *mut Self;
            std::ptr::read(raw)
        }
    }
}

use custom_callbacks::*;

#[napi]
pub mod callback {
    use super::*;
    use napi::threadsafe_function::{
        ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode,
    };

    #[napi]
    pub struct Handle {
        handle: Option<steamworks::CallbackHandle>,
    }

    #[napi]
    impl Handle {
        #[napi]
        pub fn disconnect(&mut self) {
            if let Some(handle) = self.handle.take() {
                handle.disconnect();
            }
        }
    }

    #[napi]
    pub enum SteamCallback {
        PersonaStateChange,
        SteamServersConnected,
        SteamServersDisconnected,
        SteamServerConnectFailure,
        LobbyDataUpdate,
        LobbyChatUpdate,
        P2PSessionRequest,
        P2PSessionConnectFail,
        GameLobbyJoinRequested,
        MicroTxnAuthorizationResponse,
        AvatarImageLoaded,
    }

    #[napi(ts_generic_types = "C extends keyof import('./callbacks').CallbackReturns")]
    pub fn register(
        #[napi(ts_arg_type = "C extends keyof import('./callbacks').CallbackReturns ? C : never")]
        steam_callback: SteamCallback,
        #[napi(ts_arg_type = "(value: import('./callbacks').CallbackReturns[C]) => void")] handler: JsFunction,
    ) -> Handle {
        // Create a threadsafe function
        let threadsafe_handler = handler
            .create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))
            .unwrap();

        // Register the callback
        let handle = match steam_callback {
            SteamCallback::PersonaStateChange => {
                register_callback::<steamworks::PersonaStateChange>(threadsafe_handler)
            }
            SteamCallback::SteamServersConnected => {
                register_callback::<steamworks::SteamServersConnected>(threadsafe_handler)
            }
            SteamCallback::SteamServersDisconnected => {
                register_callback::<steamworks::SteamServersDisconnected>(threadsafe_handler)
            }
            SteamCallback::SteamServerConnectFailure => {
                register_callback::<steamworks::SteamServerConnectFailure>(threadsafe_handler)
            }
            SteamCallback::LobbyDataUpdate => {
                register_callback::<steamworks::LobbyDataUpdate>(threadsafe_handler)
            }
            SteamCallback::LobbyChatUpdate => {
                register_callback::<steamworks::LobbyChatUpdate>(threadsafe_handler)
            }
            SteamCallback::P2PSessionRequest => {
                register_callback::<steamworks::P2PSessionRequest>(threadsafe_handler)
            }
            SteamCallback::P2PSessionConnectFail => {
                register_callback::<steamworks::P2PSessionConnectFail>(threadsafe_handler)
            }
            SteamCallback::GameLobbyJoinRequested => {
                register_callback::<steamworks::GameLobbyJoinRequested>(threadsafe_handler)
            }
            SteamCallback::MicroTxnAuthorizationResponse => {
                register_callback::<steamworks::MicroTxnAuthorizationResponse>(threadsafe_handler)
            }
            SteamCallback::AvatarImageLoaded => {
                register_callback::<AvatarImageLoaded>(threadsafe_handler)
            }
        };

        Handle {
            handle: Some(handle),
        }
    }

    fn register_callback<C>(
        threadsafe_handler: ThreadsafeFunction<serde_json::Value, ErrorStrategy::Fatal>,
    ) -> steamworks::CallbackHandle
    where
        C: steamworks::Callback + serde::Serialize,
    {
        let client = crate::client::get_client();
        client.register_callback(move |value: C| {
            let value = serde_json::to_value(&value).unwrap();
            threadsafe_handler.call(value, ThreadsafeFunctionCallMode::Blocking);
        })
    }
}
