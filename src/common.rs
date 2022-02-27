// stdlib
pub(crate) use std::{
  cell::Cell,
  collections::HashMap,
  fmt::{self, Display, Formatter},
  num,
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
  expr::Expr,
  interpreter::Interpreter,
  lexer::Lexer,
  literal::Literal,
  parser::Parser,
  position::Position,
  printer::Printer,
  token::Token,
  token_kind::TokenKind::{self, *},
};

// traits
pub(crate) use crate::visitor::Visitor;

// type aliases
pub(crate) type Result<T = (), E = Error> = std::result::Result<T, E>;
