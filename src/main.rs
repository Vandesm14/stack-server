#[macro_use]
extern crate rocket;
use rocket::{
  http::Status,
  response::{content, status},
};
use stack_core::prelude::*;

#[post("/execute", data = "<code>")]
fn execute(code: &str) -> status::Custom<content::RawText<String>> {
  let source = Source::new("runner", code);
  let mut lexer = Lexer::new(source);
  let exprs = parse(&mut lexer).unwrap();

  let engine = Engine::new();
  let context = Context::new();

  match engine.run(context, exprs) {
    Ok(context) => status::Custom(
      Status::Accepted,
      content::RawText(
        context
          .stack()
          .iter()
          .map(|expr| expr.to_string())
          .collect::<String>(),
      ),
    ),
    Err(err) => {
      status::Custom(Status::BadRequest, content::RawText(err.to_string()))
    }
  }
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![execute])
}
