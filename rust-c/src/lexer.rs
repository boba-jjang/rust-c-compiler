// pub mod token;
pub mod scan;

pub fn lex(input: &str) {
    scan::scan(input);
}