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
    match (operator.kind, self.eval(right)) {
      (Minus, Literal::Number(value)) => Literal::Number(-1.0 * value),
      (Bang, Literal::Boolean(value)) => Literal::Boolean(!value),
      _ => Literal::Nil,
    }
  }

  fn visit_binary(&self, left: Expr, operator: Token, right: Expr) -> Literal {
    match (self.eval(left), operator.kind, self.eval(right)) {
      (Literal::Number(l), BangEqual, Literal::Number(r)) => {
        Literal::Boolean(l != r)
      }
      (Literal::Number(l), EqualEqual, Literal::Number(r)) => {
        Literal::Boolean(l == r)
      }
      (Literal::Number(l), Greater, Literal::Number(r)) => {
        Literal::Boolean(l > r)
      }
      (Literal::Number(l), GreaterEqual, Literal::Number(r)) => {
        Literal::Boolean(l >= r)
      }
      (Literal::Number(l), Less, Literal::Number(r)) => Literal::Boolean(l < r),
      (Literal::Number(l), LessEqual, Literal::Number(r)) => {
        Literal::Boolean(l <= r)
      }
      (Literal::Number(l), Minus, Literal::Number(r)) => Literal::Number(l - r),
      (Literal::Number(l), Plus, Literal::Number(r)) => Literal::Number(l + r),
      (Literal::Number(l), Slash, Literal::Number(r)) => Literal::Number(l / r),
      (Literal::Number(l), Star, Literal::Number(r)) => Literal::Number(l * r),
      (Literal::String(l), BangEqual, Literal::String(r)) => {
        Literal::Boolean(l != r)
      }
      (Literal::String(l), EqualEqual, Literal::String(r)) => {
        Literal::Boolean(l == r)
      }
      (Literal::String(l), Greater, Literal::String(r)) => {
        Literal::Boolean(l > r)
      }
      (Literal::String(l), GreaterEqual, Literal::String(r)) => {
        Literal::Boolean(l >= r)
      }
      (Literal::String(l), Less, Literal::String(r)) => Literal::Boolean(l < r),
      (Literal::String(l), LessEqual, Literal::String(r)) => {
        Literal::Boolean(l <= r)
      }
      (Literal::String(l), Plus, Literal::String(r)) => {
        Literal::String(format!("{}{}", l, r))
      }
      _ => Literal::Nil,
    }
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
