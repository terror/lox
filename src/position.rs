#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Position {
  pub(crate) current: usize,
  pub(crate) line: usize,
  pub(crate) start: usize,
}
