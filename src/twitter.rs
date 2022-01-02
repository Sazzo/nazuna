use egg_mode::{self, entities::MediaType, KeyPair, Token};
use loading::Loading;

pub async fn get_con_and_request_tokens(api_key: String, api_secret: String) -> (KeyPair, KeyPair) {
    let con_token = egg_mode::KeyPair::new(api_key, api_secret);
    let request_token = egg_mode::auth::request_token(&con_token, "oob")
        .await
        .unwrap();

    (con_token, request_token)
}

pub async fn get_authorize_url(request_token: &KeyPair) -> String {
    let authorize_url = egg_mode::auth::authorize_url(request_token);

    authorize_url
}

pub async fn get_access_token(con_token: KeyPair, request_token: KeyPair, pin: String) -> Token {
    let token = egg_mode::auth::access_token(con_token, &request_token, pin)
        .await
        .unwrap();

    token.0
}

pub async fn get_tweet_video_resolutions(access_token: &Token, tweet_id: u64) -> Vec<String> {
    let mut loading = Loading::new();
    loading.start();
    loading.text("Fetching video resolutions...");    

    let tweet = egg_mode::tweet::show(tweet_id, access_token)
        .await
        .expect("Can't fetch the Tweet details.");

    let tweet_extended_entities = tweet
        .extended_entities
        .as_ref()
        .expect("The specified tweet don't have a media.");
    let tweet_video_media_entity = tweet_extended_entities
        .media
        .iter()
        .find(|media| media.media_type == MediaType::Video)
        .expect("The specified tweet have a media, but it's not a video.");

    let resolutions = tweet_video_media_entity
        .video_info
        .as_ref()
        .unwrap()
        .variants
        .iter()
        .filter(|variant| variant.content_type == "video/mp4")
        .map(|variant| {
            let variant_url_without_https = variant.url.replace("https://", "");
            let variant_url_splitted = variant_url_without_https.split("/").collect::<Vec<&str>>();

            variant_url_splitted.get(5).unwrap().to_string()
        })
        .collect::<Vec<String>>();

    loading.success(format!("Fetched {} different video resolutions from Twitter.", resolutions.len()));    
    loading.end();
    
    resolutions
}

pub async fn get_tweet_video_url_by_resolution(
    access_token: &Token,
    tweet_id: u64,
    resolution: String,
) -> String {
    let mut loading = Loading::new();
    loading.start();
    loading.text("Fetching tweet video URL...");

    let tweet = egg_mode::tweet::show(tweet_id, access_token)
        .await
        .expect("Can't fetch the Tweet details.");
        
    let tweet_extended_entities = tweet
        .extended_entities
        .as_ref()
        .expect("The specified tweet don't have a media.");
    let tweet_video_media_entity = tweet_extended_entities
        .media
        .iter()
        .find(|media| media.media_type == MediaType::Video)
        .expect("The specified tweet have a media, but it's not a video.");

    let video = tweet_video_media_entity
        .video_info
        .as_ref()
        .unwrap()
        .variants
        .iter()
        .find(|variant| {
            let variant_url_without_https = variant.url.replace("https://", "");
            let variant_url_splitted = variant_url_without_https.split("/").collect::<Vec<&str>>();
            let variant_resolution = variant_url_splitted.get(5).unwrap().to_string();

            variant_resolution == resolution
        })
        .expect("Can't find the video variant with the resolution specified.");

    loading.success("Fetched tweet video url successfully.");
    loading.end();
        
    video.url.to_string()
}
