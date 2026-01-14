
// Okie so like I wanna make sumthing kinda like a code interpreter? I rlly don't know how to do it hto

/* So this needs three things, a lexer (Tokenizer), a parser, and an interpreter.
   Lets say I have like a let a = 10, lexer takes that ang gives [DECLARE] [VAR: A] [EQUALS] [NUM: 10]
   Parser fella goes like, oh yeah like declare -> var -> num
   And interpreter goes like, num 10, what do I do, var ok I have a 10 and a var named a, what next
   declare, okie. a = 10 */

use std::collections::HashMap;

fn main() {
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
    EOF,
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
  }

  let my_code: &str = "let a = 5;";

  let mut lexer: Lexer<'_> = Lexer::new(my_code);





  // STARTING THE LEXER FUNCS FINALLY I THINK

}