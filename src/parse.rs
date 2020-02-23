use std::iter::Peekable;
use crate::expr::Atom;

// Lexing
#[derive(Debug)]
enum Token {
    LParen,
    RParen,
    Quasiquote,
    Unquote,
    Atom(Atom),
}

fn lex(input: &String) -> Result<Vec<Token>, String> {
    let mut iter = input.chars().peekable();
    let mut res = Vec::new();
    while let Some(&c) = iter.peek() {
        match c {
            ' ' | '\u{0009}' /*TAB*/ | '\u{000A}' /*LF*/ | '\u{000D}' /*CR*/ => {
                // skip any whitespace
                iter.next();
            }
            '\"'  => {
                // skip over quote, parse only string
                iter.next();
                res.push(get_string(c, &mut iter));
            }
            '0'..='9' => {
                iter.next(); // consume
                res.push(get_number(c, &mut iter));
            }
            // TODO: add more builtin functions
            '+' | '*' | '-' | '/' | '%' => {
                res.push(Token::Atom(Atom::Builtin(c.to_string())));
                iter.next();
            }
            '(' | '[' | '{' => {
                res.push(Token::LParen);
                iter.next();
            }
            ')' | ']' | '}' => {
                res.push(Token::RParen);
                iter.next();
            }
            '`' | '\'' => {
                res.push(Token::Quasiquote);
                iter.next();
            }
            ',' => {
                res.push(Token::Unquote);
                iter.next();
            }
            // TODO: tokenize the rest of Atom (boolean, char, and symbols)
            _ => {
                return Err(format!("Unexpected character {}", c));
            }
        }
    }
    Ok(res)
}

fn get_number<T>(c: char, iter: &mut Peekable<T>) -> Token
where
    T: Iterator<Item = char>,
{
    let mut number = c
        .to_string()
        .parse::<f64>()
        .expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = 
        iter.peek().map(|c| c.to_string().parse::<f64>())
    {
        number = number * 10. + digit;
        iter.next();
    }
    Token::Atom(Atom::Number(number))
}

fn get_string<T>(c: char, iter: &mut Peekable<T>) -> Token
where
    T: Iterator<Item = char>,
{
    let mut res = String::with_capacity(20);
    while let Some(Ok(&chr)) =
        iter.peek().map(|c| match c {
            '\"' => Err(..),
            s@_  => Ok(s)
        })
    {
       res.push(chr);
       iter.next();
    }
    iter.next(); // skip closing quote
    Token::Atom(Atom::Str(res))
}

#[cfg(test)]
mod test {
    use super::lex;

    #[test]
    fn lexer_basic() {
        let res = lex(&String::from("(+ 1 2)")).unwrap();
        assert_eq!(format!("{:?}", res).to_string(), 
            "[LParen, Atom(Builtin(\"+\")), Atom(Number(1.0)), Atom(Number(2.0)), RParen]");
    }

    #[test]
    fn lexer_str() {
        let res = lex(&String::from("\"this is a string\" 1")).unwrap();
        assert_eq!(format!("{:?}", res).to_string(), 
            "[Atom(Str(\"this is a string\")), Atom(Number(1.0))]");
    }
}
