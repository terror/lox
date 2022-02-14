use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) enum Expr<'src> {
  Assign {
    name: Token<'src>,
    value: Box<Expr<'src>>,
  },
  Binary {
    left: Box<Expr<'src>>,
    operator: Token<'src>,
    right: Box<Expr<'src>>,
  },
  Call {
    callee: Box<Expr<'src>>,
    paren: Token<'src>,
    arguments: Vec<Box<Expr<'src>>>,
  },
  Get {
    object: Box<Expr<'src>>,
    name: Token<'src>,
  },
  Grouping {
    expression: Box<Expr<'src>>,
  },
  Literal {
    value: Literal,
  },
  Logical {
    left: Box<Expr<'src>>,
    operator: Token<'src>,
    right: Box<Expr<'src>>,
  },
  Set {
    object: Box<Expr<'src>>,
    name: Token<'src>,
    value: Box<Expr<'src>>,
  },
  Super {
    keyword: Token<'src>,
    method: Token<'src>,
  },
  This {
    keyword: Token<'src>,
  },
  Unary {
    operator: Token<'src>,
    right: Box<Expr<'src>>,
  },
  Variable {
    name: Token<'src>,
  },
}

impl Expr<'_> {
  pub(crate) fn accept<T>(self, visitor: impl Visitor<T>) -> T {
    visitor.visit_expr(self)
  }
}
