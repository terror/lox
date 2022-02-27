use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct Printer {}

impl Visitor<String> for Printer {
  fn visit_expr(&self, expr: Expr) -> String {
    match expr {
      Expr::Literal { value } => self.visit_literal(value),
      Expr::Grouping { expression } => self.visit_grouping(*expression),
      Expr::Unary { operator, right } => self.visit_unary(operator, *right),
      Expr::Binary {
        left,
        operator,
        right,
      } => self.visit_binary(*left, operator, *right),
      _ => Literal::Nil.to_string(),
    }
  }
}

impl Printer {
  pub(crate) fn new() -> Self {
    Self {}
  }

  pub(crate) fn print(self, expr: Expr) -> String {
    expr.accept(self)
  }

  fn format(&self, name: &str, expr: Vec<Expr>) -> String {
    let mut result = String::from(format!("({name}"));

    expr.iter().for_each(|expr| {
      result.push_str(&format!(" {}", expr.clone().accept(self.clone())))
    });

    format!("{result})")
  }

  fn visit_literal(&self, value: Literal) -> String {
    value.to_string()
  }

  fn visit_grouping(&self, expr: Expr) -> String {
    self.format("group", vec![expr])
  }

  fn visit_unary(&self, operator: Token, right: Expr) -> String {
    self.format(operator.lexeme.unwrap_or_default(), vec![right])
  }

  fn visit_binary(&self, left: Expr, operator: Token, right: Expr) -> String {
    self.format(operator.lexeme.unwrap_or_default(), vec![left, right])
  }
}
