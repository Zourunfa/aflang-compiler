// Vec<Token> -> Module
use crate::tokenizer::Errors;
use crate::tokenizer::{Token, TokenType};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Expr {
    Int(i64),
    Nil,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decl {
    pub name: String,
    pub expr: Expr,
}

// (1)
/*
在这里，as_ref() 方法应该是将 Option<String> 类型的值转换为 Option<&str> 类型的引用，
这里的 value 字段是一个 Option<String> 类型的值，它可能包含一个 String 类型的值，
也可能是 None。接着我们使用 as_ref() 方法将其中的 Option<&str> 类型的引用提取出来，
这样可以避免对 Option<String> 类型的值进行所有权的转移。
 */
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Decl>, Errors> {
    let mut decls = vec![];
    let mut current_decl: Option<Decl> = None;

    for (i, tok) in tokens.iter().enumerate() {
        if tok.ty == TokenType::Ident {
            if current_decl.is_none() {
                current_decl = Some(Decl {
                    // as_ref()将一个类型的引用转化为另一个类型的引用,这里是将 (1)
                    name: tok.value.as_ref().unwrap().to_string(),
                    expr: Expr::Nil,
                })
            }
        } else if tok.ty == TokenType::AssignOp {
            let next_tok = tokens.iter().nth(i + 1).unwrap();

            if next_tok.ty == TokenType::Number {
                // parse 将String类型转化为Number类型
                let number: i64 = next_tok.value.as_ref().unwrap().parse().unwrap();
                // as_mut() 方法用于将一个可变引用转换为对另一个类型的可变引用
                current_decl.as_mut().unwrap().expr = Expr::Int(number)
            }
        } else if tok.ty == TokenType::SemiColon {
            if let Some(decl) = current_decl.clone() {
                decls.push(decl.clone())
            }
        }
    }

    Ok(decls)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Decl;
    use super::Expr;
    use super::Token;
    use super::TokenType;

    fn eq_vecs<T: Eq + std::fmt::Debug>(v1: Vec<T>, v2: Vec<T>) -> bool {
        if v1.len() != v2.len() {
            assert_eq!(v1.len(), v2.len());
        }
        for i in 0..v1.len() {
            assert_eq!(v1[i], v2[i]);
            if v1[i] != v2[i] {
                return false;
            }
        }
        return true;
    }
    // 测试解析赋值语句
    #[test]
    fn parse_constant_assign() {
        let tokens: Vec<Token> = vec![
            Token {
                ty: TokenType::Ident,
                value: Some(String::from("x")),
            },
            Token {
                ty: TokenType::AssignOp,
                value: None,
            },
            Token {
                ty: TokenType::Number,
                value: Some(String::from("12")),
            },
            Token {
                ty: TokenType::SemiColon,
                value: None,
            },
        ];

        let decls = parse(tokens).unwrap();
        eq_vecs(
            decls,
            vec![Decl {
                name: "x".to_string(),
                expr: Expr::Int(12),
            }],
        );
    }
}
