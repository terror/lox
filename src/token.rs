use crate::common::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token<'src> {
  pub(crate) kind: TokenKind,
  pub(crate) lexeme: Option<&'src str>,
  pub(crate) position: Position,
}
