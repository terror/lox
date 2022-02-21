use crate::common::*;

#[derive(Debug, StructOpt)]
pub(crate) struct Arguments {
  path: Option<PathBuf>,
}

impl Arguments {
  pub(crate) fn run(&self) -> Result<()> {
    match self.path.to_owned() {
      Some(path) => self.file(path),
      None => self.prompt(),
    }
  }

  fn file(&self, _path: PathBuf) -> Result<()> {
    Ok(())
  }

  fn prompt(&self) -> Result<()> {
    let history = home_dir().unwrap_or_default().join(".lox_history");

    let mut editor = Editor::<()>::new();
    editor.load_history(&history).ok();

    let interpreter = Interpreter::new();

    loop {
      let line = editor.readline("> ")?;

      editor.add_history_entry(line.as_str());
      editor.save_history(&history)?;

      println!(
        "{}",
        interpreter.clone().eval(Parser::parse(Lexer::lex(&line)?)?)
      );
    }
  }
}
