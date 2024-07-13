//! The user API.

use anyhow::Result;

use crate::Renshuu;

/// Handler for Renshuu's user API.
///
/// Created with [`Renshuu::user`].
pub struct UserHandler<'kao> {
    renshuu: &'kao Renshuu,
}

impl<'kao> UserHandler<'kao> {
    pub(crate) fn new(renshuu: &'kao Renshuu) -> Self {
        Self { renshuu }
    }

    /// Get the user's profile.
    pub async fn get_profile(&self) -> Result<String> {
        Ok(self.renshuu.get("/profile").await?.text().await?)
    }

    /// Get the lists created by the user.
    pub async fn get_lists(&self) -> Result<String> {
        Ok(self.renshuu.get("/lists").await?.text().await?)
    }

    /// Get a user-created list by its identifier.
    pub async fn get_list_by(&self, id: &str) -> Result<String> {
        let route = format!("/list/{}", id);
        Ok(self.renshuu.get(&route).await?.text().await?)
    }

    /// Get a list of all terms of a given type that the user has studied.
    pub async fn get_list_of_all_terms_by(&self, termtype: &str) -> Result<String> {
        let route = format!("/list/all/{}", termtype);
        Ok(self.renshuu.get(&route).await?.text().await?)
    }
}
