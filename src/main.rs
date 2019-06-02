extern crate regex;

mod lang;

use lang::scanner::tokenize;


fn main() {
    let _tokens = dbg!(tokenize(&mut "a = \\x.x+1;".as_bytes()));
}
