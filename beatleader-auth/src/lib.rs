mod error;

pub use error::Error;

use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, RefreshToken,
    Scope, TokenUrl,
};
use oauth2::{EmptyExtraTokenFields, StandardTokenResponse, reqwest};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

pub async fn authenticate(
    oauth2_secret: &str,
) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Error> {
    // TODO: make this code run in the server so the client secret isnt leaked
    let client = auth_client(oauth2_secret)?;

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("clan".to_string()))
        .add_scope(Scope::new("offline_access".to_string()))
        .url();

    webbrowser::open(auth_url.as_str()).unwrap();

    let (code, state) = parse_out_code_and_state(get_first_line_of_request().await);

    assert_eq!(&state, csrf_token.secret());

    let res = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(&http_client())
        .await?;

    Ok(res)
}

pub async fn reauthenticate(
    refresh_token: &RefreshToken,
    oauth2_secret: &str,
) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Error> {
    let client = auth_client(oauth2_secret)?;

    let res = client
        .exchange_refresh_token(refresh_token)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("clan".to_string()))
        .add_scope(Scope::new("offline_access".to_string()))
        .request_async(&http_client())
        .await?;

    Ok(res)
}

type AuthClient = oauth2::Client<
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    oauth2::StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
>;

fn auth_client(oauth2_secret: &str) -> Result<AuthClient, Error> {
    let client = BasicClient::new(ClientId::new("bl_desktop".to_string()))
        .set_client_secret(ClientSecret::new(oauth2_secret.to_string()))
        .set_auth_uri(AuthUrl::new(
            "https://api.beatleader.com/oauth2/authorize".to_string(),
        )?)
        .set_token_uri(TokenUrl::new(
            "https://api.beatleader.com/oauth2/token".to_string(),
        )?)
        .set_redirect_uri(RedirectUrl::new("http://localhost:8888/".to_string())?);
    Ok(client)
}

fn http_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build")
}

async fn get_first_line_of_request() -> String {
    let mut stream = TcpListener::bind("localhost:8888")
        .await
        .unwrap()
        .accept()
        .await
        .unwrap()
        .0;

    let line = BufReader::new(&mut stream)
        .lines()
        .next_line()
        .await
        .unwrap()
        .unwrap();

    stream
        .write_all(
            "\nHTTP/1.1 200 OK\r\n
<!DOCTYPE html>
<html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <meta http-equiv=\"X-UA-Compatible\" content=\"ie=edge\">
        <title>Bl Desktop Auth</title>
        
    </head>
    <body>
        <main>
            <h1>got authentication token, you may close this window</h1>
        </main>
        <script>
            window.close();
        </script>
    </body>
</html>"
                .as_bytes(),
        )
        .await
        .unwrap();

    line
}

fn parse_out_code_and_state(uri: String) -> (String, String) {
    let quesries = uri.split_once("?").unwrap().1.split("&").map(|q| {
        let mut kv = q.split("=");
        let key = kv.next().unwrap();
        let value = kv.next().unwrap();
        assert!(kv.next().is_none());
        (key, value)
    });

    (
        quesries
            .clone()
            .find(|(k, _)| k == &"code")
            .unwrap()
            .1
            .to_string(),
        quesries
            .clone()
            .find(|(k, _)| k == &"state")
            .unwrap()
            .1
            .to_string(),
    )
}
