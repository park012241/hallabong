#[cfg(feature = "translate")]
extern crate google_translate3 as translate3;
extern crate html_escape;
extern crate hyper;
extern crate hyper_rustls;
#[cfg(feature = "translate")]
extern crate yup_oauth2 as oauth2;

#[cfg(feature = "translate")]
use std::collections::HashMap;
#[cfg(feature = "translate")]
use std::default::Default;
use std::env;
#[cfg(feature = "translate")]
use std::sync::Arc;

use serenity::client::ClientBuilder;
use serenity::prelude::GatewayIntents;
#[cfg(feature = "translate")]
use tokio::sync::RwLock;
#[cfg(feature = "translate")]
use translate3::Translate;

#[cfg(feature = "translate")]
use framework::groups::translate::{GoogleProjectId, GoogleTranslate, LastTranslationLanguageCache};

use crate::framework::AttachableClientBuilder;

mod framework;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILDS
        .union(GatewayIntents::GUILD_MESSAGES)
        .union(GatewayIntents::MESSAGE_CONTENT)
        .union(GatewayIntents::GUILD_MESSAGE_REACTIONS)
        .union(GatewayIntents::GUILD_VOICE_STATES);

    let mut client = ClientBuilder::new(&token, intents)
        .build()
        .await
        .expect("Err creating client");

    #[cfg(feature = "translate")]
    {
        let secret: oauth2::ApplicationSecret = oauth2::read_application_secret(".google_auth.json").await.expect("There is no .google_auth.json");
        let google_project_id = secret.project_id.clone().expect("Project Id is needed");

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        ).persist_tokens_to_disk(".google_auth.cache.json").build().await.unwrap();

        let hub = Translate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);

        let mut data = client.data.write().await;

        data.insert::<GoogleTranslate>(Arc::new(RwLock::new(hub)));
        data.insert::<LastTranslationLanguageCache>(Arc::new(RwLock::new(HashMap::default())));
        data.insert::<GoogleProjectId>(Arc::new(google_project_id));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
