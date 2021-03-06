use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct Interpreter;

impl Visitor<Literal> for Interpreter {
  fn visit_expr(&self, expr: Expr) -> Literal {
    match expr {
      Expr::Literal { value } => self.visit_literal(value),
      Expr::Grouping { expression } => self.visit_grouping(*expression),
      Expr::Unary { operator, right } => self.visit_unary(operator, *right),
      Expr::Binary {
        left,
        operator,
        right,
      } => self.visit_binary(*left, operator, *right),
      _ => Literal::Nil,
    }
  }
}

impl Interpreter {
  pub(crate) fn new() -> Self {
    Self
  }

  pub(crate) fn eval(&self, expr: Expr) -> Literal {
    expr.accept(self.clone())
  }

  fn visit_literal(&self, value: Literal) -> Literal {
    value
  }

  fn visit_grouping(&self, expr: Expr) -> Literal {
    self.eval(expr)
  }

  fn visit_unary(&self, operator: Token, right: Expr) -> Literal {
    let right = self.eval(right);

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

  fn visit_binary(&self, left: Expr, operator: Token, right: Expr) -> Literal {
    let left = self.eval(left);
    let right = self.eval(right);

    if let Literal::Number(l) = left {
      if let Literal::Number(r) = right {
        return match operator.kind {
          BangEqual => Literal::Boolean(l != r),
          EqualEqual => Literal::Boolean(l == r),
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
          BangEqual => Literal::Boolean(l != r),
          EqualEqual => Literal::Boolean(l == r),
          Greater => Literal::Boolean(l > r),
          GreaterEqual => Literal::Boolean(l >= r),
          Less => Literal::Boolean(l < r),
          LessEqual => Literal::Boolean(l <= r),
          Plus => Literal::String(format!("{}{}", l, r)),
          _ => Literal::Nil,
        };
      }
    }

    Literal::Nil
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq};

  struct Test {
    interpreter: Interpreter,
    source: String,
    expected: String,
  }

  impl Test {
    fn new() -> Self {
      Self {
        interpreter: Interpreter::new(),
        source: String::new(),
        expected: String::new(),
      }
    }

    fn source(self, source: &str) -> Self {
      Self {
        source: source.to_owned(),
        ..self
      }
    }

    fn expected(self, expected: &str) -> Self {
      Self {
        expected: expected.to_owned(),
        ..self
      }
    }

    fn run(&self) -> Result {
      Ok(assert_eq!(
        self
          .interpreter
          .eval(Parser::parse(Lexer::lex(&self.source)?)?)
          .to_string(),
        self.expected
      ))
    }
  }

  #[test]
  fn arithmetic() -> Result {
    Test::new().source("1 + 1 / 2").expected("1.5").run()
  }

  #[test]
  fn grouping() -> Result {
    Test::new().source("(1 + 1) / 2").expected("1").run()
  }

  #[test]
  fn string_concatenation() -> Result {
    Test::new().source("\"1\" + \"1\"").expected("11").run()
  }
}
