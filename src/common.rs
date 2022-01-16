// stdlib
pub(crate) use std::{
  collections::HashMap,
  fmt::{self, Display, Formatter},
  path::PathBuf,
  process,
};

// dependencies
pub(crate) use {
  ansi_term::{Color::Red, Style},
  dirs::home_dir,
  lazy_static::lazy_static,
  rustyline::{error::ReadlineError, Editor},
  snafu::Snafu,
  structopt::StructOpt,
};

// structs and enums
pub(crate) use crate::{
  arguments::Arguments,
  error::Error,
  lexer::Lexer,
  position::Position,
  token::Token,
  token_kind::TokenKind::{self, *},
};

// type aliases
pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
