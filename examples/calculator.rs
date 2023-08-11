/*
# The "shunting yard" algorithm

Is a method of parsing arithmetic or logical operations that are written
in "infix notation" (the mathematical notation most people are used to)
to convert them into postfix notation or an AST.

The algorithm allocates 2 variable strings (input and output) and a stack that holds
operators not yet added to the output queue.

> "the shunting yard algorithm is stack-based"
> "Infix expressions are the form of mathematical notation most people are used to"

- See: https://www.youtube.com/watch?v=KJwfZ06Z6og
- See: https://en.wikipedia.org/wiki/Shunting_yard_algorithm
*/

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Define our possible valid operations
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Define our possible valid Token(s)
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char),
}

#[derive(Debug)]
/// Define our possible errors
pub enum Error {
    BadToken(char),
    MismatchedParens,
}

pub struct Calculator {}

impl Calculator {
    /// This is a static method that tokenize its input
    pub fn tokenize<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();

        // stack of characters waiting to be processed
        let chars = expr.chars();

        // stack of `Token` enums
        let mut tokens: Vec<Token> = vec![];

        // Stack of opening brackets (used to validate the number
        // of opening & closing parenthesis).
        let mut opening_brackets = vec![];

        for c in chars {
            println!("tokenizer/char/digest: '{}'", c);
            match c {
                // If this single char is a number (from 0 to 9), then
                // we have two options: either is a single digit number
                // that we can push onto the token's stack as it is, or,
                // is only one of a multi digit number.
                // ...
                '0'..='9' => match tokens.last_mut() {
                    // if there is a Token and is a Number
                    // then this is a multi digit number so we need to mutate it
                    // so the final number is represented correctly
                    Some(Token::Number(n)) => {
                        println!("joining number {} with {}", n, c);
                        *n = *n * 10 + (c as u32 - 48);
                    }
                    // otherwise just parse the char as a u32 number
                    // and push it into the token's stack
                    _ => {
                        let num = c as u32 - 48;
                        tokens.push(Token::Number(num));
                    }
                },
                '(' => {
                    println!("tokenize/char/push: '{}'", c);
                    tokens.push(Token::Bracket(c));
                    opening_brackets.push(c);
                }
                ')' => {
                    tokens.push(Token::Bracket(c));

                    if let Some(p) = opening_brackets.pop() {
                        if p != '(' {
                            eprintln!("Error!");
                            return Err(Error::MismatchedParens);
                        }
                    }
                }
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Sub)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                ' ' => {}
                '\n' => {}
                _ => return Err(Error::BadToken(c)),
            }
        }

        if opening_brackets.len() > 0 {
            return Err(Error::MismatchedParens);
        }

        Ok(tokens)
    }

    /// Convert the tokens into a "Reverse Polish Notation" (RPN).
    ///
    /// - See: https://en.wikipedia.org/wiki/Reverse_Polish_notation
    fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = vec![];
        let mut operations: Vec<Token> = vec![];

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => {
                    while !operations.is_empty() && operations[operations.len() - 1] >= token {
                        queue.push(operations.pop().unwrap());
                    }

                    operations.push(token);
                }
                Token::Bracket('(') => operations.push(token),
                Token::Bracket(')') => {
                    while !operations.is_empty()
                        && operations[operations.len() - 1] != Token::Bracket('(')
                    {
                        queue.push(operations.pop().unwrap());
                    }
                    // discard the last opening bracket
                    operations.pop();
                }
                Token::Bracket(_) => {}
            }
        }

        while operations.len() > 0 {
            queue.push(operations.pop().unwrap());
        }

        queue
    }

    fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();
        let mut stack: Vec<f32> = vec![];
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => {
                    stack.push(num as f32);
                    dbg!(num);
                }
                Token::Op(Operator::Add) => {
                    let l = stack.pop().unwrap();
                    let r = stack.pop().unwrap();

                    dbg!(l);
                    dbg!(r);
                    stack.push(l + r);
                }
                Token::Op(Operator::Sub) => {
                    let l = stack.pop().unwrap();
                    let r = stack.pop().unwrap();
                    stack.push(l - r);
                }
                Token::Op(Operator::Mul) => {
                    let l = stack.pop().unwrap();
                    let r = stack.pop().unwrap();
                    stack.push(l * r);
                }
                Token::Op(Operator::Div) => {
                    let l = stack.pop().unwrap();
                    let r = stack.pop().unwrap();
                    stack.push(r / l);
                }
                Token::Bracket(_) => {}
            }
        }

        if stack.len() > 1 {
            return None;
        }

        stack.pop()
    }
}

// Run the tests:
//
// ```
// cargo watch -q -c -w examples/ -x 'run --example calculator'
// ```
fn main() {
    let tokens = Calculator::tokenize("2 * 2 + 48 / 4").unwrap();
    let expr = Calculator::expression(tokens);
    let value = Calculator::evaluate(expr);

    dbg!(value);
}
