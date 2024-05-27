#[macro_use]
extern crate rocket;
use std::{net::Ipv4Addr, time::Duration};

use rocket::{
  fs::FileServer,
  http::Status,
  response::{content, status},
  routes, Config,
};
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::error::Error;

use stack_core::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct ExecutionResult {
  stack: Vec<Expr>,
  error: Option<RunError>,
}

impl Serialize for ExecutionResult {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: rocket::serde::Serializer,
  {
    let mut response = serializer.serialize_struct("response", 2)?;

    let stack: Vec<String> = self
      .stack
      .iter()
      .map(|expr| expr.to_string())
      .collect::<Vec<_>>();

    let error: String = match &self.error {
      Some(err) => err.to_string(),
      None => "".to_owned(),
    };

    response.serialize_field("stack", &stack)?;
    response.serialize_field("error", &error)?;

    response.end()
  }
}

#[post("/execute", data = "<code>")]
fn execute(code: &str) -> status::Custom<content::RawText<String>> {
  let code = code.to_owned();
  let result = std::thread::spawn(|| {
    let source = Source::new("runner", code);
    let mut lexer = Lexer::new(source);
    let exprs = parse(&mut lexer).unwrap();

    let mut engine = Engine::new();
    let context = Context::new();

    engine.add_module(stack_std::str::module());
    engine.add_module(stack_std::fs::module(false));
    engine.add_module(stack_std::scope::module());

    match engine.run_with_timeout(context, exprs, Duration::from_secs(5)) {
      Ok(context) => status::Custom(
        Status::Accepted,
        content::RawText(
          serde_json::to_string(&ExecutionResult {
            stack: context.stack().to_vec(),
            error: None,
          })
          .unwrap_or_default(),
        ),
      ),
      Err(err) => status::Custom(
        Status::BadRequest,
        content::RawText(
          serde_json::to_string(&ExecutionResult {
            stack: Vec::new(),
            error: Some(err),
          })
          .unwrap_or_default(),
        ),
      ),
    }
  })
  .join();

  match result {
    Ok(x) => x,
    Err(_) => status::Custom(
      Status::BadRequest,
      content::RawText("program failed to execute correctly".into()),
    ),
  }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
  rocket::build()
    .configure(Config {
      port: 5173,
      address: Ipv4Addr::new(0, 0, 0, 0).into(),
      ..Config::debug_default()
    })
    .mount("/", FileServer::from("../frontend/dist"))
    .mount("/", FileServer::from("../frontend/src/assets"))
    .mount("/", routes![execute])
    .launch()
    .await?;

  Ok(())
}
