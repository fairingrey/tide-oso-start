use async_std::sync::{Arc, Mutex};
use oso::{Oso, OsoError, PolarClass};
use tide::log::LogMiddleware;

use crate::endpoints;
use crate::expenses::Expense;
use crate::middleware::user_loader;

#[derive(Clone)]
pub struct OsoState {
    pub oso: Arc<Mutex<Oso>>,
}

impl OsoState {
    /// Create a new OsoState instance
    pub fn try_new() -> Result<OsoState, OsoError> {
        let oso = Arc::new(Mutex::new(oso()?));

        Ok(OsoState { oso })
    }

    pub async fn is_allowed(&self, actor: String, action: &str, resource: Expense) -> bool {
        let guard = self.oso.lock().await;

        guard
            .is_allowed(actor, action.to_string(), resource)
            .unwrap()
    }
}

pub fn oso() -> Result<Oso, OsoError> {
    let mut oso = Oso::new();
    oso.register_class(Expense::get_polar_class())?;

    oso.load_file("expenses.polar")?;

    Ok(oso)
}

pub async fn run() -> Result<(), tide::Error> {
    let mut app = tide::with_state(OsoState::try_new()?);

    app.with(LogMiddleware::new());
    app.with(user_loader);

    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/expenses/:id").get(endpoints::get_expense);

    println!("running at {}", std::env::var("ADDR")?);
    app.listen(std::env::var("ADDR")?).await?;
    Ok(())
}
