
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

#[derive(Debug, Clone, PartialEq)]
enum Statement {
  Let {
    name: String,
    value: String,
  },
  Print {
    value: String,
  }
}

fn main() {

  // ! Lexer or sum idk Im not that good at rust

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

        //* Case ( */
        Some('(') => Token {
          token_type: TokenType::LeftParen,
          literal: "(".to_string(),
        },

        //* Case ) */
        Some(')') => Token {
          token_type: TokenType::RightParen,
          literal: ")".to_string(),
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



  // ! PARSER STUFF LET'S FUCKING GO STEP 2 OUT OF 3 IM SO GOATED
  
  struct Parser {
    tokens: Vec<Token>,
    pos: usize,
  }

  impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
      Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
      self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
      let tok = self.tokens.get(self.pos);
      self.pos += 1;
      tok
    }
  
    fn parse_statement(&mut self) -> Option<Statement> {
      match self.current()?.token_type {
        TokenType::Declare => self.parse_let(),
        TokenType::Print => self.parse_print(),
        TokenType::EOF => None,
        _=> {
          self.next();
          None
        }
      }
    }

    fn parse_let(&mut self) -> Option<Statement> {
      self.next()?; // let

      let name = self.current()?.literal.clone();
      self.next()?; // identifier

      self.next()?; // =

      let value = self.current()?.literal.clone();
      self.next()?; // number

      self.next()?; // ;

      Some(Statement::Let { name, value })
    }

    fn parse_print(&mut self) -> Option<Statement> {
      self.next()?; // print

      self.next()?; // (

      let value = self.current()?.literal.clone();
      self.next()?; // value

      self.next()?; // )

      self.next()?; // ;

      Some(Statement::Print { value })
    }
  }

  // ! YO LET'S FUCKING GO NOW I JUST NEED THE EVALUATOR
  // ! I HAVENT EVEN COMMITED SINCE I DID THE PARSER
  // ? Which for the memo is in the end of the script


  // ! START TESTING THE LEXER FUNCS FINALLY I THINK
  
  let my_code: &str = "let var = 20; let bro_it_works = 10; print(var); print(bro_it_works);";
  let mut lexer: Lexer<'_> = Lexer::new(my_code);
  let mut tokens: Vec<_> = Vec::new();
  println!("Code to interpret: {}", my_code);
  loop {
    let tok = lexer.next_token(&keywords);
    //? println!("{:?}", tok);
    
    if tok.token_type == TokenType::EOF {
      tokens.push(tok);
      break;
    }
    tokens.push(tok);
  }
  
  // ! Parser type shit
  let mut parser: Parser = Parser::new(tokens);
  let mut statements: Vec<_> = Vec::new();
  let mut env: HashMap<String, i64> = HashMap::new();

  while let Some(stmt) = parser.parse_statement() {
    statements.push(stmt.clone());
    
      // ! Evaluator type shit
    
      match stmt {
        Statement::Let { name, value } => {
          env.insert(name, value.parse::<i64>().unwrap());
        },
        Statement::Print { value } => {
          println!("{}", env.get(&value).unwrap()); // ! IT WORKS CHARLIE KIRK WOULD BE PROUD OF ME
        }
      }
  }
}