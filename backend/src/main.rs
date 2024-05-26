#[macro_use]
extern crate rocket;
use std::time::Duration;

use rocket::{
  http::Status,
  response::{content, status},
  Config,
};
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::error::Error;

use rocket::http::Method;
use rocket::routes;
use rocket_cors::{AllowedOrigins, CorsOptions};

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
  let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
      vec![Method::Get, Method::Post, Method::Patch]
        .into_iter()
        .map(From::from)
        .collect(),
    )
    .allow_credentials(true);

  rocket::build()
    .configure(Config {
      port: 7777,
      ..Config::debug_default()
    })
    .mount("/", routes![execute])
    .attach(cors.to_cors().unwrap())
    .launch()
    .await?;

  Ok(())
}
