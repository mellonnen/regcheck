use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct LexerError(String);

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There occurred an error in lexer: {}", self.0)
    }
}

impl Error for LexerError {}

#[derive(Debug)]
pub enum Parenthesis {
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug)]
pub enum CurlyBrace {
    RightCurlyBrace,
    LeftCurlyBrace,
}

#[derive(Debug)]
pub enum Bracket {
    RightBracket,
    LeftBracket,
}
#[derive(Debug)]
pub enum Quantifier {
    ZeroOrMore(ZeroOrMore),
    OneOrMore,
}
#[derive(Debug)]
pub enum ZeroOrMore {
    Asterisk,
    QuestionMark,
}

#[derive(Debug)]
pub enum Token {
    ElementToken(char),
    WildCardToken,
    StartToken,
    EndToken,
    CommaToken,
    Parenthesis(Parenthesis),
    CurlyBrace(CurlyBrace),
    Bracket(Bracket),
    Quantifier(Quantifier),
    OrToken,
    NotToken,
    DashToken,
}

pub fn lexer(s: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;
    let mut escape_found = false;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if escape_found {
            match c {
                't' => tokens.push(Token::ElementToken('\t')),
                _ => tokens.push(Token::ElementToken(c)),
            }
        } else if c == '\\' {
            escape_found = true;
            i += 1;
            continue;
        } else {
            match c {
                '^' => {
                    if i == 0 {
                        tokens.push(Token::StartToken)
                    } else {
                        tokens.push(Token::NotToken)
                    }
                }
                '$' => tokens.push(Token::EndToken),
                '.' => tokens.push(Token::WildCardToken),
                '*' => tokens.push(Token::Quantifier(Quantifier::ZeroOrMore(
                    ZeroOrMore::Asterisk,
                ))),
                '?' => tokens.push(Token::Quantifier(Quantifier::ZeroOrMore(
                    ZeroOrMore::QuestionMark,
                ))),
                '+' => tokens.push(Token::Quantifier(Quantifier::OneOrMore)),
                '|' => tokens.push(Token::OrToken),
                '(' => tokens.push(Token::Parenthesis(Parenthesis::LeftParenthesis)),
                ')' => tokens.push(Token::Parenthesis(Parenthesis::RightParenthesis)),
                '-' => tokens.push(Token::DashToken),
                '[' => tokens.push(Token::Bracket(Bracket::LeftBracket)),
                ']' => tokens.push(Token::Bracket(Bracket::RightBracket)),
                '{' => {
                    tokens.push(Token::CurlyBrace(CurlyBrace::LeftCurlyBrace));
                    i += 1;
                    while let Some(c) = chars.next() {
                        if c == ',' {
                            tokens.push(Token::CommaToken);
                        } else if c.is_digit(10) {
                            tokens.push(Token::ElementToken(c));
                        } else if c == '}' {
                            tokens.push(Token::CurlyBrace(CurlyBrace::RightCurlyBrace));
                            break;
                        } else {
                            return Err(Box::new(LexerError("".into())));
                        }
                    }
                }
                '}' => tokens.push(Token::CurlyBrace(CurlyBrace::RightCurlyBrace)),
                _ => tokens.push(Token::ElementToken(c)),
            }
        }
        i += 1;
        escape_found = false;
    }

    Ok(tokens)
}

#[test]
fn test_simple() {
    let tokens = lexer("a").unwrap();
    if let Token::ElementToken(c) = tokens.get(0).unwrap() {
        assert_eq!(*c, 'a');
    } else {
        assert!(false);
    }
}

#[test]
fn test_escaping_char() {
    let tokens = lexer("a\\a").unwrap();
    assert_eq!(tokens.len(), 2);
    if let Token::ElementToken(c) = tokens.get(0).unwrap() {
        assert_eq!(*c, 'a');
    } else {
        assert!(false);
    }

    if let Token::ElementToken(c) = tokens.get(1).unwrap() {
        assert_eq!(*c, 'a');
    } else {
        assert!(false);
    }
}

#[test]
fn test_escaped_tab() {
    let tokens = lexer("\t").unwrap();
    if let Token::ElementToken(c) = tokens.get(0).unwrap() {
        assert_eq!(*c, '\t');
    } else {
        assert!(false);
    }
}

#[test]
fn test_escape_wildcard() {
    let tokens = lexer("\\.").unwrap();
    if let Token::ElementToken(c) = tokens.get(0).unwrap() {
        assert_eq!(*c, '.');
    } else {
        assert!(false);
    }
}

#[test]
fn test_comma() {
    let tokens = lexer("a{3,5}").unwrap();
    if let Token::CommaToken = tokens.get(3).unwrap() {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn test_comma_is_element() {
    let tokens = lexer("a,").unwrap();
    if let Token::ElementToken(c) = tokens.get(1).unwrap() {
        assert_eq!(*c, ',');
    } else {
        assert!(false);
    }
}

#[test]
fn test_match_start() {
    let tokens = lexer("^a").unwrap();
    if let Token::StartToken = tokens.get(0).unwrap() {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn test_match_end() {
    let tokens = lexer("a$").unwrap();
    if let Token::EndToken = tokens.iter().last().unwrap() {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn test_fail_curly() {
    if let Err(_) = lexer("{a}") {
        assert!(true);
    } else {
        assert!(false);
    }
}
