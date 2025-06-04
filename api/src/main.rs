#![allow(async_fn_in_trait)]

pub mod config;
pub mod domain;
pub mod presentation;

use axum::{response::Redirect, routing::get, Router};
use config::{Config, IdentityProviderConfig, LoggingConfig, ServeConfig};
use domain::openid::OpenIdClientEntity;
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest, ClientId, ClientSecret, IssuerUrl,
};
use presentation::rest::{
    handlers::{root, ApiDoc},
    AppState,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, info, instrument, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};
use utoipa::OpenApi as _;
use utoipa_rapidoc::RapiDoc;

#[instrument(skip_all, fields(%issuer_url))]
async fn get_openid_client(
    IdentityProviderConfig {
        issuer_url,
        client_id,
        client_secret,
    }: IdentityProviderConfig,
) -> Result<OpenIdClientEntity, anyhow::Error> {
    info!("Getting provider info");
    let provider_metadata =
        CoreProviderMetadata::discover_async(IssuerUrl::new(issuer_url)?, &reqwest::Client::new())
            .await?;
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    );
    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let Config {
        serve: ServeConfig { rest_addr },
        logging: LoggingConfig { dirs },
        identity_provider,
    } = config::parse_from_fs(&*config::get_path())?;
    let parsed_rest_addr: SocketAddr = rest_addr.parse()?;

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::builder().parse_lossy(dirs))
        .init();

    let identity_provider_client = get_openid_client(identity_provider).await?;
    let app_state = AppState {
        identity_provider_client,
    };

    #[rustfmt::skip]
    let router = Router::new()
        .route("/ping", get(root::ping))
        .route("/", get(|| async { Redirect::permanent("/docs") }))
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/docs"));

    let app = router.with_state(app_state);

    info!("Run application");

    match tokio::spawn(async move {
        let listener = TcpListener::bind(parsed_rest_addr).await?;
        axum::serve(listener, app).await
    })
    .await
    {
        Ok(Ok(())) => {
            warn!("Rest server stopped");
            Ok(())
        }
        Ok(Err(err)) => {
            error!(%err, "Error while bind or serve server listener");
            Err(err.into())
        }
        Err(err) => {
            error!(%err, "Error while spawn task");
            Err(err.into())
        }
    }
}
