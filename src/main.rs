mod tests;
mod tokenizer;
mod parser_combinator;
mod parser;
// 63c24cd
// #[cfg(test)]

fn main() {
    let tokens = tokenizer::tokenize("fn_name(fn_name2(fn_name_3(12)),12,34)").unwrap();
    println!(
        "{:?}",
        parser::Parser::new(tokens).parse_next_expr().unwrap()
    );
}
