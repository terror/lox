use crate::common::*;

mod arguments;
mod common;
mod error;
mod expr;
mod lexer;
mod literal;
mod parser;
mod position;
mod token;
mod token_kind;
mod visitor;

fn main() {
  if let Err(error) = Arguments::from_args().run() {
    if let Error::Readline { ref source } = error {
      if let ReadlineError::Interrupted | ReadlineError::Eof = source {
        return;
      }
    }

    eprintln!(
      "{}{}",
      Red.paint("error"),
      Style::new().bold().paint(format!(": {}", error))
    );

    process::exit(1);
  }
}
