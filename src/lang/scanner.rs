use regex::{Match, Regex};
use std::fmt;
use std::io::BufRead;

/// The Scanner is the first step of any non-trivial parsing task.
/// The responsibility of the scanner is to take a stream of raw text and
/// turn it into a list of tokens which can be used by later parts of an
/// interpreter or compiler.

/// Line, and symbol-position for parsed tokens
/// This is useful for later printing of debug- and error information
#[derive(PartialEq, Debug, Clone)]
pub struct TokenPosition {
    line: usize,
    position: usize,
}

impl fmt::Display for TokenPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} and position {}", self.line, self.position)
    }
}

/// Tokenization rules macro
/// 
/// Format is
/// [TokenName][Optional Parameters] = [Regex rule] => [Formatting],
/// 
/// Rules are prioritized in order
#[macro_export]
macro_rules! token_rules {
    ($($name:ident$($args:ty)? = $regex:expr => $rule:expr,)+) => {

        /// Internal token types used by the scanner to tag matched regexes
        #[repr(u8)]
        #[allow(dead_code)]
        #[derive(Debug)]
        enum TokenType {
            $($name, )+
            /* 
             * By encoding errors as a token we can continue parsing 
             * in order to capture as many scanner errors as possible in one go
             */
            Error
        }

        impl TokenType {
            /// Gets the TokenType from the unerlying index
            /// 
            /// When using regex to find tokens we get a capture index, we can then use this index to get the correct TokenType
            fn from_index(index: usize) -> TokenType {
                // Since we have set #[repr(u8)] on TokenType we can argue that this code will not cause undefined behaviour
                unsafe { ::std::mem::transmute::<u8, TokenType>(index as u8) }
            }
        }

        /// Tokens exposed by the scanner after a successfull scan
        #[derive(Debug, PartialEq)]
        pub enum Token {
            $($name$(($args))?, )+
        }

        /// Takes a regex of alternations and a string and converts it into a vector of disjoint matches
        fn find_matches<'a>(re: &Regex, line: &'a str) -> Vec<(TokenType, Match<'a>)> {

            let mut matches: Vec<(TokenType, Match)> = Vec::new();

            for cap in re.captures_iter(line) {
                matches.extend(
                    cap.iter().enumerate()
                    .skip(1)
                    .find(|(_,m)| m.is_some())
                    .map(|(i, m)| (TokenType::from_index(i - 1), m.unwrap()))
                );
            }

            matches
        }

        /// Returns a vector of tokens from a BufRead.
        /// The tokenizer takes the buffer and splits it into tokens as defined in this macro.
        pub fn tokenize<R: BufRead>(buf_reader: &mut R) -> Result<Vec<(TokenPosition, Token)>, Vec<(TokenPosition, String)>> {

            // Separate recording of valid tokens and errors allows for easy handling later
            let mut tokens: Vec<(TokenPosition, Token)> = Vec::new();
            let mut errors: Vec<(TokenPosition, String)> = Vec::new();

            // Regex responsible for parsing the lines
            let re = Regex::new(&concat!($("|(",$regex,")",)+r"|(\S+)")[1..]).expect("Invalid regex");

            for (line_num, line) in buf_reader.lines().enumerate() {
                for (token_type, cap) in find_matches(&re, line.as_ref().unwrap()) {

                    match token_type {
                        /*
                         * The macro expands into a complete pattern-match of
                         * all defined tokens.
                         */
                        $(TokenType::$name =>
                            {
                                use Token::*;
                                tokens.push((
                                    TokenPosition {
                                    line: line_num,
                                    position: cap.start()
                                },
                                $rule(&line.as_ref().unwrap()[cap.start()..cap.end()])
                                ));
                            }
                        ,)+
                        /*
                         * Since the Error token is special to the scanner and may result in
                         * scanner failure it is handled separately
                         */
                        TokenType::Error => {
                            errors.push((
                                TokenPosition {
                                    line: line_num,
                                    position: cap.start()
                                },
                                line.as_ref().unwrap()[cap.start()..cap.end()].to_string()
                            ));
                        }
                    };
                }
            }

            if !errors.is_empty() {
                return Err(errors);
            }

            Ok(tokens)
        }
    };
}

/// Tokenization rules
/// 
/// Format is
/// [TokenName][Optional Parameters] = [Regex rule] => [Formatting],
/// 
/// Rules are prioritized in order
token_rules! {
    // Numbers
    Float(f64) = r"[[:digit:]]*\.[[:digit:]]+" => |x: &str| Float(x.parse::<f64>().unwrap()),
    Int(i64) = r"[[:digit:]]+" => |x: &str| Int(x.parse::<i64>().unwrap()),
    Char(i64) = r"'[[[:alpha:]]|\n]'" => |x: &str| Char(x[1..].chars().next().unwrap() as i64),
    // Comparators
    Equal = r"==" => |_| Equal,
    Neq = r"!=" => |_| Neq,
    Leq = r"<=" => |_| Leq,
    Geq = r">=" => |_| Geq,
    Less = r"<" => |_| Less,
    Greater = r">" => |_| Greater,
    // Arithmetic operators
    Add = r"\+" => |_| Add,
    Sub = r"\-" => |_| Sub,
    Mul = r"\*" => |_| Mul,
    Div = r"/" => |_| Div,
    Assign = r"=" => |_| Assign,
    // Logical operators
    Not = r"!" => |_| Not,
    And = r"&&" => |_| And,
    Or = r"\|\|" => |_| Or,
    // Bitwise operators
    Bnot = r"~" => |_| Bnot,
    Band = r"&" => |_| Band,
    Bor = r"\|" => |_| Bor,
    Xor = r"\^" => |_| Xor,
    // Structure tokens
    Lambda = r"\\" => |_| Lambda,
    Comma = r"," => |_| Comma,
    Period = r"\." => |_| Period,
    Semicolon = r";" => |_| Semicolon,
    Lpar = r"\(" => |_| Lpar,
    Rpar = r"\)" => |_| Rpar,
    // Calls and bindings
    Call(String) = r"[[:alpha:]][[:word:]]*\(" => |x: &str| Call(x[..x.len()-1].to_string()),
    Builtin(String) = r"_[[:alpha:]][[:word:]]*\(" => |x: &str| Builtin(x[..x.len()-1].to_string()),
    Name(String) = r"[[:alpha:]][[:word:]]*" => |x: &str| Name(x.to_string()),
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_tokenize_ok {
        ($s:expr => $($name:expr,)*)  => {
            {
                use Token::*;
                let tokens = tokenize(&mut $s.as_bytes());
                println!("{:?}", tokens);
                assert!(tokens.is_ok());
                let tokens = tokens.unwrap();
                let mut i = 0;
                $(
                    assert_eq!(tokens[i].1, $name);
                    i = i + 1;
                )*
                assert_eq!(i, tokens.len());
            }
        }
    }

    macro_rules! test_tokenize_err {
        ($s:expr => $($args:expr,)*)  => {
            {
                let errs = tokenize(&mut $s.as_bytes());
                println!("{:?}", errs);
                assert!(errs.is_err());
                let errs = errs.unwrap_err();
                let mut i = 0;
                $(
                    assert_eq!(errs[i].1, $args);
                    i = i + 1;
                )*
                assert_eq!(i, errs.len());
            }
        }
    }

    #[test]
    fn test_single() {
        test_tokenize_ok!("\\" =>
            Lambda,
        );

        test_tokenize_ok!("'A'" =>
            Char(65),
        );
    }

    #[test]
    fn test_long() {
        test_tokenize_ok!("f = \\g x.g(g(x))" =>
            Name("f".to_string()),
            Assign,
            Lambda,
            Name("g".to_string()),
            Name("x".to_string()),
            Period,
            Call("g".to_string()),
            Call("g".to_string()),
            Name("x".to_string()),
            Rpar,
            Rpar,
        )
    }

    #[test]
    fn test_single_err() {
        test_tokenize_err!("¤" =>
            "¤",
        );
        test_tokenize_err!("error ¤" =>
            "¤",
        );
        test_tokenize_err!("¤ error" =>
            "¤",
        );
    }
}
