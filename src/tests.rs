use super::parser;
use super::parser::{Expr, FnCall};
use super::tokenizer;

#[test]
fn nested_fn_call() {
    let tokens = tokenizer::tokenize("fn1(fn2(fn3(12)),12,34)").unwrap();
    println!("tokens:{:?}", tokens);
    let mut parser = parser::Parser::new(tokens);

    assert_eq!(
        parser.parse_next_expr().unwrap(),
        Expr::FnCall(FnCall {
            name: "fn1".to_string(),
            args: vec![
                Expr::FnCall(FnCall {
                    name: "fn2".to_string(),
                    args: vec![Expr::FnCall(FnCall {
                        name: "fn3".to_string(),
                        args: vec![Expr::Int(12)]
                    })]
                }),
                Expr::Int(12),
                Expr::Int(34)
            ]
        })
    )
}
