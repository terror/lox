use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(context(false), display("Readline Error: {}", source))]
  Readline {
    source: rustyline::error::ReadlineError,
  },

  #[snafu(display("Lexer Error: {}", message))]
  Lexer { message: String },

  #[snafu(display("Unexpected token start"))]
  UnexpectedTokenStart,
}
