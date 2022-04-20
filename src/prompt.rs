use dialoguer::{Select, theme::ColorfulTheme, Input};

use crate::models::APICredentials;

pub fn select_twitter_video_resolution(resolutions: &Vec<String>) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the video resolution")
        .items(&resolutions[..])
        .interact()
        .unwrap()
}

pub fn request_twitter_oauth_pin() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Twitter PIN:")
        .interact_text()
        .unwrap()
}

pub fn request_twitter_credentials() -> APICredentials {
    let api_key: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Twitter API Key:")
        .interact_text()
        .unwrap();

    let api_secret: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Twitter API Secret:")
        .interact_text()
        .unwrap();

    let credentials = APICredentials {
        api_key,
        api_secret,
    };
    credentials
}
