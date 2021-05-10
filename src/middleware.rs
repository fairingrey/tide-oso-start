use std::future::Future;
use std::pin::Pin;

use tide::{Next, Request, Result as TideResult};

use crate::models::User;
use crate::server::OsoState;

/// Middleware for loading user
pub fn user_loader<'a>(
    mut request: Request<OsoState>,
    next: Next<'a, OsoState>,
) -> Pin<Box<dyn Future<Output = TideResult> + Send + 'a>> {
    Box::pin(async {
        if let Some(username_hval) = request.header("user") {
            let username = username_hval.get(0).unwrap().as_str().to_owned();
            tide::log::trace!("user loaded", { user: username });
            request.set_ext(User { name: username });
        }
        Ok(next.run(request).await)
    })
}
