use crate::common::*;

#[derive(Debug)]
pub(crate) struct Parser<'src> {
  next: Cell<usize>,
  tokens: Vec<Token<'src>>,
}

impl<'src> Parser<'src> {
  /// Parse `tokens`.
  pub(crate) fn parse(tokens: Vec<Token<'src>>) -> Result<Expr<'src>> {
    Self::new(tokens).parse_ast()
  }

  /// Construct and return a new `Parser` instance based on `tokens`.
  fn new(tokens: Vec<Token<'src>>) -> Self {
    Self {
      next: Cell::new(0),
      tokens,
    }
  }

  /// Parse a lox flat token stream.
  fn parse_ast(&self) -> Result<Expr<'src>> {
    self.expression()
  }

  /// Advance the current position.
  fn advance(&self) -> Token<'src> {
    if !self.is_end() {
      self.next.set(self.next.get() + 1);
    }
    self.prev()
  }

  /// Advance and return the token at our current position if the current token
  /// kind matches `kind`.
  fn consume<'a>(
    &self,
    kind: TokenKind,
    message: &'a str,
  ) -> Result<Token<'src>> {
    if self.check(kind.clone()) {
      return Ok(self.advance());
    }

    Err(Error::Parser {
      message: message.to_owned(),
    })
  }

  /// Return the token at located at the previous position.
  fn prev(&self) -> Token<'src> {
    self.tokens[self.next.get() - 1].clone()
  }

  /// Return the token located at the current position.
  fn peek(&self) -> Token<'_> {
    self.tokens[self.next.get()].clone()
  }

  /// Check if any token in `kinds` matches the current token, and then advance
  /// our current position.
  fn match_kinds(&self, kinds: Vec<TokenKind>) -> bool {
    for kind in kinds {
      if self.check(kind) {
        self.advance();
        return true;
      }
    }
    false
  }

  /// Check if the token kind at our current position matches `kind`, and if so,
  /// advance our current position.
  fn match_kind(&self, kind: TokenKind) -> bool {
    if self.check(kind) {
      self.advance();
      return true;
    }

    false
  }

  /// Check to see if the token kind `kind` matches the token kind at our
  /// current position.
  fn check(&self, kind: TokenKind) -> bool {
    if self.is_end() {
      return false;
    }
    self.peek().kind == kind
  }

  /// Check if we're at the end of the token stream.
  fn is_end(&self) -> bool {
    self.peek().kind == Eof
  }

  /// Method for the `expression` grammar rule.
  fn expression(&self) -> Result<Expr<'src>> {
    self.equality()
  }

  /// Method for the `equality` grammar rule.
  fn equality(&self) -> Result<Expr<'src>> {
    let mut expr = self.comparison()?;

    while self.match_kinds(vec![BangEqual, EqualEqual]) {
      expr = Expr::Binary {
        left: Box::new(expr),
        operator: self.prev(),
        right: Box::new(self.comparison()?),
      };
    }

    Ok(expr)
  }

  /// Method for the `comparison` grammar rule.
  fn comparison(&self) -> Result<Expr<'src>> {
    let mut expr = self.term()?;

    while self.match_kinds(vec![Greater, GreaterEqual, Less, LessEqual]) {
      expr = Expr::Binary {
        left: Box::new(expr),
        operator: self.prev(),
        right: Box::new(self.term()?),
      };
    }

    Ok(expr)
  }

  /// Method for the `term` grammar rule.
  fn term(&self) -> Result<Expr<'src>> {
    let mut expr = self.factor()?;

    while self.match_kinds(vec![Minus, Plus]) {
      expr = Expr::Binary {
        left: Box::new(expr),
        operator: self.prev(),
        right: Box::new(self.factor()?),
      };
    }

    Ok(expr)
  }

  /// Method for the `factor` grammar rule.
  fn factor(&self) -> Result<Expr<'src>> {
    let mut expr = self.unary()?;

    while self.match_kinds(vec![Slash, Star]) {
      expr = Expr::Binary {
        left: Box::new(expr),
        operator: self.prev(),
        right: Box::new(self.unary()?),
      };
    }

    Ok(expr)
  }

  /// Method for the `unary` grammar rule.
  fn unary(&self) -> Result<Expr<'src>> {
    if self.match_kinds(vec![Bang, Minus]) {
      return Ok(Expr::Unary {
        operator: self.prev(),
        right: Box::new(self.unary()?),
      });
    }

    self.primary()
  }

  /// Method for the `primary` grammar rule.
  fn primary(&self) -> Result<Expr<'src>> {
    if self.match_kind(False) {
      return Ok(Expr::Literal {
        value: Literal::Boolean(false),
      });
    }

    if self.match_kind(Nil) {
      return Ok(Expr::Literal {
        value: Literal::Nil,
      });
    }

    if self.match_kind(Number) {
      return Ok(Expr::Literal {
        value: Literal::Number(self.prev().lexeme.parse()?),
      });
    }

    if self.match_kind(StringLiteral) {
      return Ok(Expr::Literal {
        value: Literal::String(
          // chop off the surrdounding quotes
          self.prev().lexeme[1..self.prev().lexeme.len() - 1].to_string(),
        ),
      });
    }

    if self.match_kind(True) {
      return Ok(Expr::Literal {
        value: Literal::Boolean(false),
      });
    }

    if self.match_kind(ParenL) {
      let expr = self.expression()?;
      self.consume(ParenR, "Expected closing )")?;
      return Ok(Expr::Grouping {
        expression: Box::new(expr),
      });
    }

    Err(Error::Parser {
      message: "Invalid expression".into(),
    })
  }

  /// Synchronize the parser.
  fn sync(&self) {
    self.advance();

    while !self.is_end() {
      if self.prev().kind == Semicolon {
        return;
      }

      match self.peek().kind {
        Class | Fun | Var | For | If | While | Print | Return => {
          return;
        }
        _ => {}
      }

      self.advance();
    }
  }
}
