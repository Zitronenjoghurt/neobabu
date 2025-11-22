use crate::state::ServerState;
use neobabu_core::database::entity::dashboard_session;
use neobabu_core::stores::dashboard_session::DashboardSessionStore;
use neobabu_core::stores::{IntoActiveModel, Set};
use std::fmt::Debug;
use std::sync::Arc;
use tower_sessions::cookie::time::Duration;
use tower_sessions::cookie::{Key, SameSite};
use tower_sessions::service::SignedCookie;
use tower_sessions::session::{Id, Record};
use tower_sessions::{Expiry, SessionManagerLayer, SessionStore};

pub fn build_session_layer(
    state: &ServerState,
) -> SessionManagerLayer<SessionStorage, SignedCookie> {
    let session_storage = SessionStorage::new(state.core.stores.dashboard_session.clone());
    let key = Key::from(state.config.session_secret.as_bytes());
    SessionManagerLayer::new(session_storage)
        .with_expiry(Expiry::OnInactivity(Duration::days(14)))
        .with_same_site(SameSite::Lax)
        .with_signed(key)
        .with_secure(false)
}

#[derive(Debug, Clone)]
pub struct SessionStorage {
    store: Arc<DashboardSessionStore>,
}

impl SessionStorage {
    pub fn new(store: Arc<DashboardSessionStore>) -> Self {
        Self { store }
    }

    fn map_err<E: std::error::Error>(err: E) -> tower_sessions::session_store::Error {
        tower_sessions::session_store::Error::Backend(err.to_string())
    }

    fn record_to_active_model(
        &self,
        record: &Record,
    ) -> Result<dashboard_session::ActiveModel, tower_sessions::session_store::Error> {
        Ok(dashboard_session::ActiveModel {
            id: Set(record.id.0.to_string()),
            data: Set(serde_json::to_string(&record.data).map_err(Self::map_err)?),
            offset_datetime: Set(serde_json::to_string(&record.expiry_date).map_err(Self::map_err)?),
            ..Default::default()
        })
    }

    fn model_to_record(
        &self,
        session: dashboard_session::Model,
    ) -> Result<Record, tower_sessions::session_store::Error> {
        Ok(Record {
            id: Id(session.id.parse().map_err(|_| {
                tower_sessions::session_store::Error::Backend("Failed to parse session id".into())
            })?),
            data: serde_json::from_str(&session.data).map_err(Self::map_err)?,
            expiry_date: serde_json::from_str(&session.offset_datetime).map_err(Self::map_err)?,
        })
    }
}

#[async_trait::async_trait]
impl SessionStore for SessionStorage {
    async fn save(&self, session_record: &Record) -> tower_sessions::session_store::Result<()> {
        let session_id = session_record.id.0.to_string();

        match self.store.find(&session_id).await.map_err(Self::map_err)? {
            Some(existing) => {
                let mut active = existing.into_active_model();
                active.data =
                    Set(serde_json::to_string(&session_record.data).map_err(Self::map_err)?);
                active.offset_datetime =
                    Set(serde_json::to_string(&session_record.expiry_date)
                        .map_err(Self::map_err)?);
                self.store.update(active).await.map_err(Self::map_err)?;
            }
            None => {
                let new = self.record_to_active_model(session_record)?;
                self.store.insert(new).await.map_err(Self::map_err)?;
            }
        }

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> tower_sessions::session_store::Result<Option<Record>> {
        self.store
            .find(&session_id.0.to_string())
            .await
            .map_err(Self::map_err)?
            .map(|s| self.model_to_record(s))
            .transpose()
    }

    async fn delete(&self, session_id: &Id) -> tower_sessions::session_store::Result<()> {
        let session_id = session_id.0.to_string();
        self.store
            .delete_id(&session_id)
            .await
            .map_err(|err| tower_sessions::session_store::Error::Backend(err.to_string()))?;
        Ok(())
    }
}
