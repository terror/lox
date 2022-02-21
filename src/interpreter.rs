use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct Interpreter;

impl Visitor<Literal> for Interpreter {
  fn visit_expr(&self, expr: Expr) -> Literal {
    match expr {
      Expr::Literal { value } => value,
      Expr::Grouping { .. } => self.clone().eval(expr),
      Expr::Unary { operator, right } => {
        let right = self.clone().eval(*right);

        if let Literal::Number(value) = right {
          return match operator.kind {
            Minus => Literal::Number(-1.0 * value),
            _ => Literal::Nil,
          };
        }

        if let Literal::Boolean(value) = right {
          return match operator.kind {
            Bang => Literal::Boolean(!value),
            _ => Literal::Nil,
          };
        }

        Literal::Nil
      }
      Expr::Binary {
        left,
        operator,
        right,
      } => {
        let left = self.clone().eval(*left);
        let right = self.clone().eval(*right);

        if let Literal::Number(l) = left {
          if let Literal::Number(r) = right {
            return match operator.kind {
              BangEqual => Literal::Boolean(l != r),
              Equal => Literal::Boolean(l == r),
              Greater => Literal::Boolean(l > r),
              GreaterEqual => Literal::Boolean(l >= r),
              Less => Literal::Boolean(l < r),
              LessEqual => Literal::Boolean(l <= r),
              Minus => Literal::Number(l - r),
              Plus => Literal::Number(l + r),
              Slash => Literal::Number(l / r),
              Star => Literal::Number(l * r),
              _ => Literal::Nil,
            };
          }
        }

        if let Literal::String(l) = left {
          if let Literal::String(r) = right {
            return match operator.kind {
              Plus => Literal::String(format!("{}{}", l, r)),
              _ => Literal::Nil,
            };
          }
        }

        Literal::Nil
      }
      _ => Literal::Number(1.0),
    }
  }
}

impl Interpreter {
  pub(crate) fn new() -> Self {
    Self
  }

  pub(crate) fn eval(self, expr: Expr) -> Literal {
    expr.accept(self)
  }
}
