
// Okie so like I wanna make sumthing kinda like a code interpreter? I rlly don't know how to do it hto

/* So this needs three things, a lexer (Tokenizer), a parser, and an interpreter.
   Lets say I have like a let a = 10, lexer takes that ang gives [DECLARE] [VAR: A] [EQUALS] [NUM: 10]
   Parser fella goes like, oh yeah like declare -> var -> num
   And interpreter goes like, num 10, what do I do, var ok I have a 10 and a var named a, what next
   declare, okie. a = 10 */

fn main() {
  let keywords = {
    "let": DECLARE,
    "if": IFs
  }
}