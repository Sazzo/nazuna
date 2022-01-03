use dialoguer::console::style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::json;
use std::cmp::min;
use std::{
    fs::{self, File},
    io::Write,
};
use structopt::StructOpt;

mod models;
mod twitter;

use models::{APICredentials, OAuthCredentials};

#[derive(StructOpt)]
struct Cli {
    // Tweet URL
    url: String,

    // Output File (optional)
    #[structopt(short, long, default_value = "output.mp4", required = false)]
    output: String,
}

const NAZUNA_ASCII_TEXT: &str = r#"                                       
                                  
 _ __   __ _ _____   _ _ __   __ _ 
| '_ \ / _` |_  / | | | '_ \ / _` |
| | | | (_| |/ /| |_| | | | | (_| |
|_| |_|\__,_/___|\__,_|_| |_|\__,_|
                                                                      
"#;

const FIRST_TIME_MESSAGE: &str = r#"
Welcome to Nazuna! Looks like it's your first time that you're using me!
Before I can start, I need some Twitter credentials (API Key and API Secret).

Please, take a read at https://github.com/Sazzo/Nazuna#configuration and when you're ready, provide me the keys below!
"#;

const OAUTH_MESSAGE: &str = r#"
Now, I need you to authenticate using your Twitter account. I'll try to open your browser with the OAuth URL.
If this doesn't works, you can try opening the URL manually in your browser.
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", NAZUNA_ASCII_TEXT);

    let args = Cli::from_args();
    let home_dir = dirs::home_dir().expect("I can't find your home folder!");

    let nazuna_folder_path: String = format!("{}/.nazuna", home_dir.to_str().unwrap());
    let nazuna_folder = fs::read_dir(&nazuna_folder_path);

    if nazuna_folder.is_err() {
        println!("{}", FIRST_TIME_MESSAGE);
        let credentials = request_twitter_credentials();

        fs::create_dir(&nazuna_folder_path)
            .expect("I can't create the .nazuna folder in your home directory!");

        let credentials_file_content = json!({
            "api_key": credentials.api_key,
            "api_secret": credentials.api_secret,
        });

        let mut credentials_file =
            File::create(format!("{}/credentials.json", &nazuna_folder_path)).unwrap();
        credentials_file
            .write_all(credentials_file_content.to_string().as_bytes())
            .unwrap();

        println!(
            "\nAwesome! Your credentials are stored at {}. Let's proceed now!\n",
            &nazuna_folder_path
        )
    }

    let credentials_file =
        fs::read_to_string(format!("{}/credentials.json", &nazuna_folder_path)).unwrap();
    let oauth_credentials = fs::read(format!("{}/oauth_credentials.json", &nazuna_folder_path));

    if oauth_credentials.is_err() {
        let credentials: APICredentials = serde_json::from_str(&credentials_file)?;
        let (con_token, request_token) =
            twitter::get_con_and_request_tokens(credentials.api_key, credentials.api_secret).await;
        let authorize_url = twitter::get_authorize_url(&request_token).await;

        println!("{}\n", OAUTH_MESSAGE);
        println!("{}\n", authorize_url);

        // Try opening the authorize URL on the browser.
        open::that(authorize_url).unwrap();

        let twitter_oauth_pin = request_twitter_oauth_pin();
        let access_token =
            twitter::get_access_token(con_token, request_token, twitter_oauth_pin).await;

        let oauth_credentials_file_content = json!({
            "access_token": access_token,
        });

        let mut oauth_credentials_file =
            File::create(format!("{}/oauth_credentials.json", &nazuna_folder_path)).unwrap();
        oauth_credentials_file
            .write_all(oauth_credentials_file_content.to_string().as_bytes())
            .unwrap();
    }

    let oauth_credentials_file =
        fs::read_to_string(format!("{}/oauth_credentials.json", &nazuna_folder_path)).unwrap();
    let oauth_credentials: OAuthCredentials = serde_json::from_str(&oauth_credentials_file)?;

    let tweet_url_without_https = args.url.replace("https://", "");
    let tweet_splitted = tweet_url_without_https.split("/").collect::<Vec<&str>>();
    let tweet_id = tweet_splitted.last().unwrap().parse::<u64>().unwrap();

    let tweet_video_resolutions =
        twitter::get_tweet_video_resolutions(&oauth_credentials.access_token, tweet_id).await;
    let selected_resolution_index = select_twitter_video_resolution(&tweet_video_resolutions);
    let selected_resolution = tweet_video_resolutions
        .get(selected_resolution_index)
        .unwrap();

    let video_url = twitter::get_tweet_video_url_by_resolution(
        &oauth_credentials.access_token,
        tweet_id,
        selected_resolution.to_string(),
    )
    .await;

    download_video(video_url, args.output).await.expect("Error trying to download the video.");

    Ok(())
}

async fn download_video(video_url: String, output: String) -> Result<(), String> {
    // https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
    let response = reqwest::get(video_url)
        .await
        .expect("Error trying to download the video.");
    let video_size = response.content_length().expect("Cannot fetch the video size. Please try again.");

    let progress_bar = ProgressBar::new(video_size);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    progress_bar.set_message("Downloading video...");

    let mut file = File::create(&output).unwrap();
    let mut downloaded_size: u64 = 0;
    let mut response_stream = response.bytes_stream();

    while let Some(item) = response_stream.next().await {
        let chunk = item.unwrap();
        file.write(&chunk).unwrap();

        let new_progress = min(downloaded_size + (chunk.len() as u64), video_size);
        downloaded_size = new_progress;

        progress_bar.set_position(downloaded_size);
    }

    progress_bar.finish_with_message(format!(
        "{} Downloaded video to {}",
        style("✔".to_string()).green(),
        output
    ));
    Ok(())
}

fn select_twitter_video_resolution(resolutions: &Vec<String>) -> usize {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the video resolution")
        .items(&resolutions[..])
        .interact()
        .unwrap();

    selection
}

fn request_twitter_oauth_pin() -> String {
    let twitter_oauth_pin: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Twitter PIN:")
        .interact_text()
        .unwrap();

    twitter_oauth_pin
}

fn request_twitter_credentials() -> APICredentials {
    let twitter_api_key: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Twitter API Key:")
        .interact_text()
        .unwrap();

    let twitter_api_secret: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Twitter API Secret:")
        .interact_text()
        .unwrap();

    let credentials = APICredentials {
        api_key: twitter_api_key,
        api_secret: twitter_api_secret,
    };
    credentials
}
