use crate::expr::Atom;
use std::iter::Peekable;

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
        // skip any whitespace
        if c.is_whitespace() {
            iter.next();
            continue;
        }
        match c {
            // String
            '\"' => {
                iter.next(); // skip quote
                res.push(get_string(c, &mut iter));
            }
            // Integer
            '0'..='9' => {
                iter.next();
                res.push(get_number(c, &mut iter));
            }
            // Builtin operator OR symbol
            '+' | '*' | '-' | '/' | '%' | '!' | '^' | '=' => {
                // first check if this is a symbol
                iter.next();
                if let Some(&c1) = iter.peek() {
                    if !(char::is_whitespace(c1) || is_paren(c1)) {
                        res.push(get_symbol(c, &mut iter));
                        continue;
                    }
                }
                // otherwise, it is a builtin function
                res.push(Token::Atom(Atom::Builtin(c.to_string())));
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
            'A'..='Z' | 'a'..='z' => {
                iter.next();
                res.push(get_symbol(c, &mut iter));
            }
            _ => {
                return Err(format!("Unexpected character {}", c));
            }
        }
    }
    Ok(res)
}

fn is_paren(c: char) -> bool {
    match c {
        '(' | ')' | '[' | ']' | '{' | '}' => true,
        _                                 => false,
    }
}

fn get_number<T>(c: char, iter: &mut Peekable<T>) -> Token
where
    T: Iterator<Item = char>,
{
    let mut number = c
        .to_string()
        .parse::<f64>()
        .expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<f64>()) {
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
    while let Some(Ok(&chr)) = iter.peek().map(|c| match c {
        '\"' => Err(..),
        s @ _ => Ok(s),
    }) {
        res.push(chr);
        iter.next();
    }
    iter.next(); // skip closing quote
    Token::Atom(Atom::Str(res))
}

fn get_symbol<T>(c: char, iter: &mut Peekable<T>) -> Token
where
    T: Iterator<Item = char>,
{
    let mut res = String::with_capacity(20);
    res.push(c);
    while let Some(Ok(&chr)) = iter.peek().map(|c| {
        if char::is_whitespace(*c) || is_paren(*c) {
            Err(..)
        } else {
            Ok(c)
        }
    }) {
        res.push(chr);
        iter.next();
    }
    Token::Atom(Atom::Symbol(res))
}
#[cfg(test)]
mod test {
    use super::lex;

    #[test]
    fn lexer_basic() {
        let res = lex(&String::from("(+ 1 2)")).unwrap();
        assert_eq!(
            format!("{:?}", res).to_string(),
            "[LParen, Atom(Builtin(\"+\")), Atom(Number(1.0)), Atom(Number(2.0)), RParen]"
        );
    }

    #[test]
    fn lexer_str() {
        let res = lex(&String::from("\"this is a string\" 1")).unwrap();
        assert_eq!(
            format!("{:?}", res).to_string(),
            "[Atom(Str(\"this is a string\")), Atom(Number(1.0))]"
        );
    }

    #[test]
    fn lexer_symbol() {
        let res = lex(&String::from("(if *abc*)")).unwrap();
        assert_eq!(
            format!("{:?}", res).to_string(),
            "[LParen, Atom(Symbol(\"if\")), Atom(Symbol(\"*abc*\")), RParen]"
        );
    }
}
