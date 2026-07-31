use beatleader_auth::{authenticate, reauthenticate};
use log::{debug, info, trace};
use oauth2::{
    AccessToken, EmptyExtraTokenFields, StandardTokenResponse, TokenResponse, basic::BasicTokenType,
};
use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};
use tokio::{
    sync::{RwLock, RwLockReadGuard},
    time::sleep,
};

pub use beatleader_auth::Error as AuthError;
pub(crate) use refresh_token_key::RefreshTokenKey;

const OATH2_SECRET: &str = env!("OAUTH2_SECRET");

pub struct AccessTokenKey;

static DATA: LazyLock<Arc<RwLock<Option<AccessToken>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(None)));

impl AccessTokenKey {
    pub async fn get() -> RwLockReadGuard<'static, AccessToken> {
        let lock = DATA.read().await;
        if lock.is_some() {
            trace!("Returning cached access token");
            RwLockReadGuard::map(lock, |v| v.as_ref().unwrap())
        } else {
            debug!("No access token found, fetching new one");
            drop(lock);
            let mut write = DATA.write().await;

            let tokens = Self::fetch_tokens().await;
            debug!("Got new access token, saving refresh token");
            RefreshTokenKey::set(tokens.refresh_token().unwrap()).await;
            debug!("Storing access token");
            *write = Some(tokens.access_token().clone());

            trace!("Cached access token saved, starting refresh watch thread");
            let data_clone = DATA.clone();
            tokio::spawn(async move {
                loop {
                    debug!("Waiting 30 minutes before refreshing access token");
                    sleep(Duration::from_secs(30 * 60)).await;
                    debug!("Refreshing access token");

                    let mut lock = data_clone.write().await;
                    let tokens = Self::fetch_tokens().await;
                    trace!("Got new access token, updating refresh token");
                    RefreshTokenKey::set(tokens.refresh_token().unwrap()).await;
                    trace!("Storing new access token");
                    *lock = Some(tokens.access_token().clone());

                    debug!("Access token refreshed successfully");
                }
            });

            debug!("Returning new access token");
            RwLockReadGuard::map(write.downgrade(), |v| v.as_ref().unwrap())
        }
    }

    async fn fetch_tokens() -> StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType> {
        debug!("Fetching new tokens");
        match reauthenticate(&*RefreshTokenKey::get().await, OATH2_SECRET).await {
            Ok(token) => {
                debug!("Successfully reauthenticated");
                token
            }
            Err(e) => {
                info!(
                    "Failed to reauthenticate. Getting a new refresh token. Error: {}",
                    e
                );
                authenticate(OATH2_SECRET).await.unwrap()
            }
        }
    }
}

mod refresh_token_key {
    use std::{
        path::PathBuf,
        sync::{Arc, LazyLock},
    };

    use beatleader_auth::authenticate;
    use directories::BaseDirs;
    use log::{debug, info, trace};
    use oauth2::{RefreshToken, TokenResponse};

    pub struct RefreshTokenKey;

    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
        task::spawn_blocking,
    };

    use crate::auth::OATH2_SECRET;
    static DATA: LazyLock<Arc<RwLock<Option<RefreshToken>>>> =
        LazyLock::new(|| Arc::new(RwLock::new(None)));

    impl RefreshTokenKey {
        async fn init() -> RwLockWriteGuard<'static, Option<RefreshToken>> {
            let lock = DATA.read().await;
            if lock.is_some() {
                drop(lock);
                debug!("Returning existing RefreshToken lock");
                return DATA.write().await;
            }

            drop(lock);
            let mut write = DATA.write().await;

            // Try loading from OS keyring; if missing, perform interactive auth and store.
            match spawn_blocking(|| {
                let kr = keyring::Entry::new("beatleader", "refresh_token").unwrap();
                kr.get_password()
            })
            .await
            .unwrap()
            {
                Ok(secret) => {
                    debug!("Loaded refresh token from keyring");
                    *write = Some(Self::deserialize(secret.as_bytes()));
                }
                Err(_) => {
                    info!(
                        "No existing refresh token found in keyring, authenticating to get new one"
                    );
                    let default = authenticate(OATH2_SECRET)
                        .await
                        .unwrap()
                        .refresh_token()
                        .unwrap()
                        .clone();

                    // store in keyring
                    let secret = default.secret().to_string();
                    spawn_blocking(move || {
                        let kr = keyring::Entry::new("beatleader", "refresh_token").unwrap();
                        kr.set_password(&secret).unwrap();
                    })
                    .await
                    .unwrap();

                    *write = Some(default);
                    debug!("New refresh token saved to keyring");
                }
            }

            write
        }

        pub async fn get() -> RwLockReadGuard<'static, RefreshToken> {
            let lock = DATA.read().await;
            if lock.is_some() {
                trace!("Returning cached refresh token");
                RwLockReadGuard::map(lock, |v| v.as_ref().unwrap())
            } else {
                drop(lock);
                debug!("Initializing refresh token");
                let write = Self::init().await;
                RwLockReadGuard::map(RwLockWriteGuard::downgrade(write), |v| v.as_ref().unwrap())
            }
        }

        pub async fn set(new: &RefreshToken) {
            debug!("Setting new refresh token in keyring");
            let secret = new.secret().to_string();
            spawn_blocking(move || {
                let kr = keyring::Entry::new("beatleader", "refresh_token").unwrap();
                kr.set_password(&secret).unwrap();
            })
            .await
            .unwrap();
            let mut write = Self::init().await;
            *write = Some(new.clone());
        }

        fn deserialize(contents: &[u8]) -> RefreshToken {
            trace!("Deserializing refresh token");
            RefreshToken::new(String::from_utf8(contents.to_vec()).unwrap())
        }

        fn serialize(_val: &RefreshToken) -> Vec<u8> {
            trace!("Serializing refresh token (unused)");
            Vec::new()
        }
    }
}
