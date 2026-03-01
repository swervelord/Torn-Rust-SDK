//! High-level ergonomic resource wrappers built on top of [`crate::TornClient`].

use std::path::Path;

use crate::client::{ClientError, TornClient};
use crate::transport::{HttpTransport, ReqwestTransport};

mod internal;

/// Shared SDK wrapper error types.
pub mod error;
/// Faction resource wrapper.
pub mod faction;
/// Forum resource wrapper.
pub mod forum;
/// Key resource wrapper.
pub mod key;
/// Market resource wrapper.
pub mod market;
/// Shared wrapper option types.
pub mod options;
/// Property resource wrapper.
pub mod property;
/// Racing resource wrapper.
pub mod racing;
/// Torn resource wrapper.
pub mod torn;
/// User resource wrapper.
pub mod user;

pub use error::SdkError;
pub use faction::{FactionApi, FactionOptions};
pub use forum::{ForumApi, ForumOptions};
pub use key::{KeyApi, KeyOptions};
pub use market::{MarketApi, MarketOptions};
pub use options::{BaseOptions, SortOrder};
pub use property::{PropertyApi, PropertyOptions};
pub use racing::{RacingApi, RacingOptions};
pub use torn::{TornApi, TornOptions};
pub use user::{UserApi, UserOptions};

#[derive(Debug)]
/// Top-level ergonomic SDK wrapper entrypoint.
pub struct TornSdk<T: HttpTransport> {
    client: TornClient<T>,
}

impl<T: HttpTransport> TornSdk<T> {
    /// Creates a wrapper facade from an initialized client.
    pub fn new(client: TornClient<T>) -> Self {
        Self { client }
    }

    /// Exposes the underlying low-level client.
    pub fn client(&self) -> &TornClient<T> {
        &self.client
    }

    /// Returns the `user` resource API.
    pub fn user(&self) -> UserApi<'_, T> {
        UserApi {
            client: &self.client,
        }
    }

    /// Returns the `faction` resource API.
    pub fn faction(&self) -> FactionApi<'_, T> {
        FactionApi {
            client: &self.client,
        }
    }

    /// Returns the `forum` resource API.
    pub fn forum(&self) -> ForumApi<'_, T> {
        ForumApi {
            client: &self.client,
        }
    }

    /// Returns the `key` resource API.
    pub fn key(&self) -> KeyApi<'_, T> {
        KeyApi {
            client: &self.client,
        }
    }

    /// Returns the `market` resource API.
    pub fn market(&self) -> MarketApi<'_, T> {
        MarketApi {
            client: &self.client,
        }
    }

    /// Returns the `property` resource API.
    pub fn property(&self) -> PropertyApi<'_, T> {
        PropertyApi {
            client: &self.client,
        }
    }

    /// Returns the `racing` resource API.
    pub fn racing(&self) -> RacingApi<'_, T> {
        RacingApi {
            client: &self.client,
        }
    }

    /// Returns the `torn` resource API.
    pub fn torn(&self) -> TornApi<'_, T> {
        TornApi {
            client: &self.client,
        }
    }
}

impl TornSdk<ReqwestTransport> {
    /// Builds a reqwest-backed SDK from environment configuration.
    pub fn from_env(capabilities_path: impl AsRef<Path>) -> Result<Self, ClientError> {
        TornClient::from_env(capabilities_path).map(Self::new)
    }
}
