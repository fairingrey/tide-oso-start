use tide::{Request, Response, Result, StatusCode};

use crate::expenses::DB;
use crate::models::User;
use crate::server::OsoState;

pub async fn get_expense(request: Request<OsoState>) -> Result<Response> {
    match request.param("id")?.parse::<usize>() {
        Ok(id) => {
            let expense = match DB.get(&id) {
                Some(expense) => expense,
                None => return Ok(Response::new(StatusCode::NotFound)),
            };

            let user = match request.ext::<User>() {
                Some(user) => user,
                None => return Ok(Response::new(StatusCode::Unauthorized)),
            };

            let is_allowed = request
                .state()
                .is_allowed(user.name.clone(), "GET", expense.clone())
                .await;

            if is_allowed {
                Ok(Response::builder(StatusCode::Ok)
                    .body(format!("{}\n", expense))
                    .build())
            } else {
                Ok(Response::builder(StatusCode::Forbidden)
                    .body("Not Authorized!")
                    .build())
            }
        }
        Err(e) => {
            tide::log::trace!("Error: {:?}", e);
            Ok(Response::new(StatusCode::BadRequest))
        }
    }
}
