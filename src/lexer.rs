use crate::common::*;

#[derive(Debug)]
pub(crate) struct Lexer<'src> {
  position: Position,
  src:      &'src str,
  tokens:   Vec<Token<'src>>,
}

lazy_static! {
  static ref KEYWORDS: HashMap<&'static str, TokenKind> = {
    let mut m = HashMap::new();
    m.insert("and", And);
    m.insert("class", Class);
    m.insert("else", Else);
    m.insert("false", False);
    m.insert("for", For);
    m.insert("fun", Fun);
    m.insert("nil", Nil);
    m.insert("or", Or);
    m.insert("print", Print);
    m.insert("return", Return);
    m.insert("super", Super);
    m.insert("this", This);
    m.insert("true", True);
    m.insert("var", Var);
    m.insert("while", While);
    m
  };
}

impl<'src> Lexer<'src> {
  /// Lex `src`.
  pub(crate) fn lex(src: &'src str) -> Result<Vec<Token<'src>>> {
    Lexer::new(src).tokenize()
  }

  /// Create and return a new `Lexer` instance based on `src`.
  fn new(src: &'src str) -> Self {
    Lexer {
      position: Position {
        start:   0,
        current: 0,
        line:    1,
      },
      src,
      tokens: Vec::new(),
    }
  }

  /// The main loop. Keep lexing tokens while we haven't reached the end of the
  /// source code.
  fn tokenize(mut self) -> Result<Vec<Token<'src>>> {
    while !self.is_end() {
      self.position.start = self.position.current;
      self.lex_token()?;
    }

    self.token(Eof)?;

    Ok(self.tokens)
  }

  /// Increment our current position and return the characters that resides at
  /// that position.
  fn advance(&mut self) -> Result<char> {
    self.position.current += 1;
    self
      .src
      .chars()
      .nth(self.position.current - 1)
      .ok_or_else(|| Error::Lexer {
        message: "Lexer advanced passed end of line.".into(),
      })
  }

  /// Return the character that resides at our current position without
  /// incrementing the current position.
  fn peek(&self) -> char {
    if self.is_end() {
      return '\0';
    }
    self.src.chars().nth(self.position.current).unwrap()
  }

  /// Return the character that resides at the position one over from our
  /// current position.
  fn peek_next(&self) -> char {
    if self.position.current + 1 >= self.src.len() {
      return '\0';
    }
    self.src.chars().nth(self.position.current + 1).unwrap()
  }

  /// Check if our current position is greater than the length of `src`.
  fn is_end(&self) -> bool {
    self.position.current >= self.src.len()
  }

  /// Check if a character is a digit.
  fn is_digit(&self, c: char) -> bool {
    c >= '0' && c <= '9'
  }

  /// Check if a character is a letter.
  fn is_alpha(&self, c: char) -> bool {
    c >= 'a' && c <= 'z' || (c >= 'A' && c <= 'Z') || c == '_'
  }

  /// Check if a character is a letter or a number.
  fn is_alphanumeric(&self, c: char) -> bool {
    self.is_digit(c) || self.is_alpha(c)
  }

  /// Conditionally consume the current character if it matches `expected`.
  fn match_token(&mut self, expected: char) -> Result<bool> {
    if self.is_end() {
      return Ok(false);
    }

    if self.peek() != expected {
      return Ok(false);
    }

    self.position.current += 1;
    Ok(true)
  }

  /// Lex a token given its starting character.
  fn lex_token(&mut self) -> Result<()> {
    let start = self.advance()?;

    match start {
      '(' => self.lex_single(ParenL),
      ')' => self.lex_single(ParenR),
      '*' => self.lex_single(Star),
      '+' => self.lex_single(Plus),
      ',' => self.lex_single(Comma),
      '-' => self.lex_single(Minus),
      ';' => self.lex_single(Semicolon),
      '{' => self.lex_single(BraceL),
      '}' => self.lex_single(BraceR),
      '!' => self.lex_choice('=', (BangEqual, Bang)),
      '<' => self.lex_choice('=', (LessEqual, Less)),
      '=' => self.lex_choice('=', (EqualEqual, Equal)),
      '>' => self.lex_choice('=', (GreaterEqual, Greater)),
      '"' => self.lex_string_literal(),
      '/' => {
        if self.match_token('/')? {
          self.lex_comment()
        } else {
          self.lex_single(Slash)
        }
      }
      '\n' => {
        self.position.line += 1;
        Ok(())
      }
      ' ' | '\t' | '\r' => Ok(()),
      _ => {
        if self.is_digit(start) {
          self.lex_number()
        } else if self.is_alpha(start) {
          self.lex_ident()
        } else {
          Err(Error::Lexer {
            message: format!("Unexpected character: {}.", start),
          })
        }
      }
    }
  }

  /// Lex a single token.
  fn lex_single(&mut self, kind: TokenKind) -> Result<()> {
    self.token(kind)
  }

  /// Choose `then` if the current character matches `expected`, else choose
  /// `otherwise`.
  fn lex_choice(&mut self, expected: char, choices: (TokenKind, TokenKind)) -> Result<()> {
    let (then, otherwise) = choices;
    match self.match_token(expected)? {
      true => self.token(then),
      false => self.token(otherwise),
    }
  }

  /// Comments get ignored until we are either at the end of a line or at the
  /// end of the source code.
  fn lex_comment(&mut self) -> Result<()> {
    while self.peek() != '\n' && !self.is_end() {
      self.advance()?;
    }
    Ok(())
  }

  /// Lex a string literal.
  fn lex_string_literal(&mut self) -> Result<()> {
    while self.peek() != '"' && !self.is_end() {
      if self.peek() == '\n' {
        self.position.line += 1;
      }
      self.advance()?;
    }

    if self.is_end() {
      return Err(Error::Lexer {
        message: "Unterminated string.".into(),
      });
    }

    self.advance()?;

    self.token(StringLiteral)
  }

  /// Lex a number.
  fn lex_number(&mut self) -> Result<()> {
    while self.is_digit(self.peek()) {
      self.advance()?;
    }

    // Look for the decimal
    if self.peek() == '.' && self.is_digit(self.peek_next()) {
      // Consume the `.`
      self.advance()?;
      // Get the rest of the number
      while self.is_digit(self.peek()) {
        self.advance()?;
      }
    }

    self.token(Number)
  }

  /// Lex an identifier.
  fn lex_ident(&mut self) -> Result<()> {
    while self.is_alphanumeric(self.peek()) {
      self.advance()?;
    }

    self.token(
      KEYWORDS
        .get(&self.src[self.position.start..self.position.current])
        .ok_or_else(|| Error::Lexer {
          message: "Unexpected identifier.".into(),
        })?
        .clone(),
    )
  }

  /// Add a token to `self.tokens` given a `TokenKind`.
  fn token(&mut self, kind: TokenKind) -> Result<()> {
    let lexeme = &self.src[self.position.start..self.position.current];

    self.tokens.push(Token {
      kind,
      lexeme,
      position: self.position.clone(),
    });

    Ok(())
  }
}
