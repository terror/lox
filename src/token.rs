use crate::common::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token<'src> {
  pub(crate) kind:     TokenKind,
  pub(crate) lexeme:   &'src str,
  pub(crate) position: Position,
}
