use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Lexer Error: {}", message))]
  Lexer { message: String },

  #[snafu(display("Parser Error: {}", message))]
  Parser { message: String },

  #[snafu(context(false), display("Readline Error: {}", source))]
  Readline {
    source: rustyline::error::ReadlineError,
  },

  #[snafu(context(false), display("Failed to parse float: {}", source))]
  ParseFloat { source: num::ParseFloatError },
}
