use crate::common::*;

pub(crate) trait Visitor<T> {
  fn visit_expr(&self, expr: Expr) -> T;
}
