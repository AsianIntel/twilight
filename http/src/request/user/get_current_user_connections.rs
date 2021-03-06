use crate::{
    client::Client,
    error::Error,
    request::{Pending, Request},
    routing::Route,
};
use twilight_model::user::Connection;

/// Get the current user's connections.
///
/// Requires the `connections` `OAuth2` scope.
pub struct GetCurrentUserConnections<'a> {
    fut: Option<Pending<'a, Vec<Connection>>>,
    http: &'a Client,
}

impl<'a> GetCurrentUserConnections<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetUserConnections);

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetCurrentUserConnections<'_>, Vec<Connection>);
