use log::{debug, trace};
use oauth2::AccessToken;
use objects::common::{Identity, LeaderboardId, PlayerId, ScoreId};
use objects::{
    leaderboard_api::Leaderboard, player_api::Player, score_api::Score, scores_api::Scores,
};
use reqwest::{Client, IntoUrl, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, sync::OnceLock};
use tokio::sync::RwLockReadGuard;

use crate::auth::AccessTokenKey;
use crate::{Error, Result};

pub mod objects;

macro_rules! api {
    ($endpoint:tt) => {
        format!("https://api.beatleader.com{}", format!($endpoint))
    };
}

static CLIENT_STORAGE: OnceLock<Client> = OnceLock::new();

#[derive(Debug)]
pub struct BlApi {
    request_builder: RequestBuilder,
    /// used for making sure the auth code doesnt change mid request
    auth_lock: Option<RwLockReadGuard<'static, AccessToken>>,
}

impl BlApi {
    fn client() -> &'static Client {
        match CLIENT_STORAGE.get() {
            Some(client) => client,
            None => {
                CLIENT_STORAGE.set(Client::new()).unwrap();
                CLIENT_STORAGE.get().unwrap()
            }
        }
    }

    fn new(method: Method, url: impl IntoUrl) -> Self {
        Self {
            request_builder: Self::client().request(method, url),
            auth_lock: None,
        }
    }

    async fn send_api<T: DeserializeOwned>(method: Method, url: impl IntoUrl) -> Result<T> {
        let text = Self::new(method, url).send().await?.text().await?;
        let deserializer = &mut serde_json::Deserializer::from_str(&text);
        let result = serde_path_to_error::deserialize(deserializer);

        Ok(result.map_err(|err| crate::Error::Deserialization { error: err })?)
    }

    async fn send_api_authed<T: DeserializeOwned>(method: Method, url: impl IntoUrl) -> Result<T> {
        let text = Self::new(method, url)
            .authenticate()
            .await
            .send()
            .await?
            .text()
            .await?;

        let deserializer = &mut serde_json::Deserializer::from_str(&text);
        let result = serde_path_to_error::deserialize(deserializer);

        Ok(result.map_err(|err| crate::Error::Deserialization { error: err })?)
    }

    async fn authenticate(mut self) -> Self {
        let auth_lock = AccessTokenKey::get().await;
        self.request_builder = self.request_builder.bearer_auth(auth_lock.deref().secret());

        self.auth_lock = Some(auth_lock);
        self
    }

    fn query(mut self, key: &str, value: &impl Serialize) -> Self {
        self.request_builder = self.request_builder.query(&[(key, value)]);
        self
    }

    async fn send(self) -> Result<reqwest::Response> {
        let res = self.request_builder.send().await?;

        let status = res.status();
        if !status.is_success() {
            let text = res.text().await?;
            return Err(Error::HttpError { code: status, text });
        }

        Ok(res)
    }

    pub async fn identity() -> Result<Identity> {
        Self::send_api_authed(Method::GET, api!("/oauth2/identity")).await
    }

    pub async fn player_scores(PlayerId(id): &PlayerId) -> Result<Scores> {
        Self::send_api(Method::GET, api!("/player/{id}/scores")).await
    }

    pub async fn score(ScoreId(id): &ScoreId) -> Result<Score> {
        Self::send_api(Method::GET, api!("/score/{id}")).await
    }

    pub async fn download_image(image_url: impl IntoUrl) -> Result<Vec<u8>> {
        let response = Self::new(Method::GET, image_url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn player(PlayerId(id): &PlayerId) -> Result<Player> {
        Self::send_api(Method::GET, api!("/player/{id}")).await
    }

    pub async fn leaderboard(LeaderboardId(id): &LeaderboardId) -> Result<Leaderboard> {
        Self::send_api(Method::GET, api!("/leaderboard/{id}")).await
    }
}
