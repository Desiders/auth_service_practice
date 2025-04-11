use crate::domain::openid::OpenIdClientEntity;

use axum::extract::FromRef;

#[derive(Clone)]
pub struct App {
    pub identity_provider_client: OpenIdClientEntity,
}

impl FromRef<App> for OpenIdClientEntity {
    fn from_ref(state: &App) -> Self {
        state.identity_provider_client.clone()
    }
}
