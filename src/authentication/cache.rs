use async_lock::RwLock;
use oauth2::{AccessToken};
use futures::Future;
use std::collections::HashMap;
use clap::builder::styling::Reset;
use tracing::trace;

#[derive(Debug)]
pub(crate) struct TokenCache(pub(crate) RwLock<HashMap<Vec<String>, AccessToken>>);

impl TokenCache {
    pub(crate) fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }

    #[allow(dead_code)]
    pub(crate) async fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut token_cache = self.0.write().await;
        token_cache.clear();
        Ok(())
    }

    pub(crate) async fn get_token(&self) -> Option<AccessToken> {
        let token_cache = self.0.read().await;
        if let Some(token) = token_cache.get(&vec![]) {
            //if !token.is_expired(None) {
            trace!("returning cached token");
            return Some(token.clone());
            //}
        }
        
        drop(token_cache);
        
        // otherwise, drop the read lock and get a write lock to refresh the token
        let mut token_cache = self.0.write().await;

        return token_cache.remove(&vec![]);
    }
}

impl Default for TokenCache {
    fn default() -> Self {
        TokenCache::new()
    }
}
