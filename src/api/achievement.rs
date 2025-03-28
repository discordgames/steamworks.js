use napi_derive::napi;

#[napi]
pub mod achievement {
    #[napi]
    pub fn activate(achievement: String) -> bool {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .set()
            .and_then(|_| client.user_stats().store_stats())
            .is_ok()
    }

    #[napi]
    pub fn is_activated(achievement: String) -> bool {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .get()
            .unwrap_or(false)
    }

    #[napi]
    pub fn clear(achievement: String) -> bool {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .clear()
            .and_then(|_| client.user_stats().store_stats())
            .is_ok()
    }

    #[napi]
    pub fn names() -> Vec<String> {
        let client = crate::client::get_client();
        client
            .user_stats()
            .get_achievement_names()
            .expect("Failed to get achievement names")
    }

    #[napi]
    pub fn get_num_achievements() -> u32 {
        let client = crate::client::get_client();
        client.user_stats().get_num_achievements().unwrap_or(0)
    }

    #[napi]
    pub fn get_achievement_percent(achievement: String) -> f32 {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .get_achievement_achieved_percent()
            .unwrap_or(0.0)
    }

    #[napi]
    pub fn get_achievement_display_name(achievement: String) -> Option<String> {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .get_achievement_display_attribute("name")
            .ok()
            .map(|s| s.to_string())
    }

    #[napi]
    pub fn get_achievement_description(achievement: String) -> Option<String> {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .get_achievement_display_attribute("desc")
            .ok()
            .map(|s| s.to_string())
    }

    #[napi]
    pub fn get_achievement_hidden(achievement: String) -> Option<bool> {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .get_achievement_display_attribute("hidden")
            .ok()
            .map(|value| value == "1")
    }
}
