
// Okie so like I wanna make sumthing kinda like a code interpreter? I rlly don't know how to do it hto

/* So this needs three things, a lexer (Tokenizer), a parser, and an interpreter.
   Lets say I have like a let a = 10, lexer takes that ang gives [DECLARE] [VAR: A] [EQUALS] [NUM: 10]
   Parser fella goes like, oh yeah like declare -> var -> num
   And interpreter goes like, num 10, what do I do, var ok I have a 10 and a var named a, what next
   declare, okie. a = 10 */

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
  // Keywords
  Declare,
  If,
  Print,

  // Literals
  Identifier,
  Number,

  // Operations
  Plus,
  Minus,
  Equals,
  Star,
  Slash,

  // Misc
  LeftParen,
  RightParen,
  Semicolon,
  EOF,
}

fn main() {

  #[derive(Debug)]
  struct Token {
    token_type: TokenType,
    literal: String,
  }

  // Gonna need a hashmap for them keywords
  let keywords = {
    let mut m = HashMap::new();
    m.insert("let", TokenType::Declare);
    m.insert("if", TokenType::If);
    m.insert("print", TokenType::Print);
    m
  };

  struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    read_position: usize,
    ch: Option<char>,
  }

  impl <'a> Lexer<'a> {
    //* Self::new() */
    fn new(input: &'a str) -> Self {
      let mut lexer = Lexer {
        input,
        pos: 0,
        read_position: 0,
        ch: None,
      };

      lexer.read_char();
      lexer
    }

    //* Read character */
    fn read_char(&mut self) {
      if self.read_position >= self.input.len() {
        self.ch = None;
      } else {
        self.ch = self.input[self.read_position..].chars().next();
      }

      self.pos = self.read_position;

      if let Some(c) = self.ch {
        self.read_position += c.len_utf8();
      } else {
        self.read_position += 1;
      }
    }

    //* Helper func: Identifiers */
    fn read_identifier(&mut self) -> String {
      let start = self.pos;
      while let Some(c) = self.ch {
        if is_letter(c) {
          self.read_char();
        } else {
          break;
        }
      }
      self.input[start..self.pos].to_string()
    }

    //* Helper func: Numbers */
    fn read_number(&mut self) -> String {
      let start = self.pos;
      while let Some(c) = self.ch {
        if c.is_ascii_digit() {
          self.read_char();
        } else {
          break;
        }
      }
      self.input[start..self.pos].to_string()
    }

    //* Match characters */
    fn next_token(&mut self, keywords: &HashMap<&str, TokenType>) -> Token {
      
      // Like skio if it's not good enuff
      while matches!(self.ch, Some(' ' | '\n' | '\t' | '\r')) {
        self.read_char();
      }

      let tok: Token = match self.ch {
        //* Case = */
        Some('=') => Token {
          token_type: TokenType::Equals,
          literal: "=".to_string(),
        },

        //* Case + */
        Some('+') => Token {
          token_type: TokenType::Plus,
          literal: "+".to_string(),
        },

        //* Case - */
        Some('-') => Token {
          token_type: TokenType::Minus,
          literal: "-".to_string(),
        },

        //* Case * */
        Some('*') => Token {
          token_type: TokenType::Star,
          literal: "*".to_string(),
        },

        //* Case / */
        Some('/') => Token {
          token_type: TokenType::Slash,
          literal: "/".to_string(),
        },

        //* Case ; */
        Some(';') => Token {
          token_type: TokenType::Semicolon,
          literal: ";".to_string(),
        },

        //* Case EOF */
        None => Token {
          token_type: TokenType::EOF,
          literal: "".to_string(),
        },

        //* Case Identifier */
        Some(c) if is_letter(c) => {
          let ident = self.read_identifier();
          let token_type = keywords
            .get(ident.as_str())
            .cloned()
            .unwrap_or(TokenType::Identifier);

          return Token {
            token_type,
            literal: ident,
          };
        },

        //* Case Number */
        Some(c) if c.is_ascii_digit() => {
          let num = self.read_number();
          return Token {
            token_type: TokenType::Number,
            literal: num,
          }
        },

        _ => {
          // Unknown character
          Token {
            token_type: TokenType::EOF,
            literal: "".to_string(),
          }
        }
      };

      self.read_char();
      tok
    }
  }

  //* Helper func: is_letter() */
  fn is_letter(c: char) -> bool {
    c.is_alphabetic() || c == '_'
  }

  let my_code: &str = "let a = 5;";

  let mut lexer: Lexer<'_> = Lexer::new(my_code);





  // START TESTING THE LEXER FUNCS FINALLY I THINK
  loop {
    let tok = lexer.next_token(&keywords);
    println!("{:?}", tok);

    if tok.token_type == TokenType::EOF {
      break;
    }
  }
}