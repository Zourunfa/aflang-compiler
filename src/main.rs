mod parser;
#[cfg(test)]
mod tests;
mod tokenizer;

// 63c24cd
fn main() {
    let tokens = tokenizer::tokenize("fn_name(fn_name2(fn_name_3(12)),12,34)").unwrap();
    println!(
        "{:?}",
        parser::Parser::new(tokens).parse_next_expr().unwrap()
    );
}
