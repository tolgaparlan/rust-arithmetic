use std::fmt;
use std::str::Chars;
use std::iter::{Peekable};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Token {
    Number(u64),
    Plus,
    Minus,
    Mult,
    Div,
    LeftPar,
    RightPar,
    End,
}

#[derive(PartialEq, Debug)]
pub struct TokenizeError {
    index: usize
}

impl std::error::Error for TokenizeError {}

struct CharStream<'a> {
    index: usize,
    iterator: Peekable<Chars<'a>>,
}

pub fn tokenize(line: String) -> Result<Vec<Token>, TokenizeError> {
    let mut char_stream = CharStream::new(line.chars().peekable());
    let mut tokens = vec![];

    loop {
        let c = char_stream.next();

        let tok = match c {
            '\0' => Token::End,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mult,
            '/' => Token::Div,
            '(' => Token::LeftPar,
            ')' => Token::RightPar,
            n if c.is_numeric() => {
                let mut chars = vec![n];
                while char_stream.peek().is_numeric() {
                    chars.push(char_stream.next());
                }
                let kek = chars.iter().collect::<String>();
                let n: u64 = match kek.parse::<u64>() {
                    Ok(n) => n,
                    Err(_) => return Err(TokenizeError { index: char_stream.index })
                };
                Token::Number(n)
            }
            _ if c.is_whitespace() => { continue; }
            _ => { return Err(TokenizeError { index: char_stream.index }); }
        };

        if let Token::End = tok {
            break;
        }

        tokens.push(tok);
    }

    Ok(tokens)
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid token at index: {}", self.index)
    }
}

impl<'a> CharStream<'a> {
    fn new(iterator: Peekable<Chars>) -> CharStream {
        CharStream {
            index: 0,
            iterator,
        }
    }

    fn next(&mut self) -> char {
        self.index += 1;
        self.iterator.next().unwrap_or('\0')
    }

    fn peek(&mut self) -> char {
        self.iterator.peek().cloned().unwrap_or('\0')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize(String::from("1+123*(12/234)")).unwrap(),
            vec![
                Token::Number(1),
                Token::Plus,
                Token::Number(123),
                Token::Mult,
                Token::LeftPar,
                Token::Number(12),
                Token::Div,
                Token::Number(234),
                Token::RightPar
            ]
        );
    }

    #[test]
    fn test_error() {
        assert_eq!(
            tokenize(String::from("1+asd*(12/234)")),
            Err(TokenizeError {
                index: 3
            })
        );
    }
}