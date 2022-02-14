use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) enum Literal {
  Boolean(bool),
  Number(f64),
  String(String),
}

impl Display for Literal {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Literal::Boolean(value) =>
          String::from(if *value { "true" } else { "false" }),
        Literal::Number(value) => (*value).to_string(),
        Literal::String(value) => value.to_string(),
      }
    )
  }
}
