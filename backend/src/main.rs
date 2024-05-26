#[macro_use]
extern crate rocket;
use std::time::Duration;

use rocket::{
  fs::FileServer,
  http::Status,
  response::{content, status},
  Config,
};
use std::error::Error;

use rocket::http::Method;
use rocket::routes;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

use stack_core::prelude::*;

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
          context
            .stack()
            .iter()
            .enumerate()
            .map(|(i, expr)| {
              let mut string = String::new();
              if i != 0 {
                string.push_str(", ");
              }
              string.push_str(&expr.to_string());

              string
            })
            .collect::<String>(),
        ),
      ),
      Err(err) => {
        status::Custom(Status::BadRequest, content::RawText(err.to_string()))
      }
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
