#[derive(Debug)]
enum Errors {
    CannotCreateStringWhileInOtherToken,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum TokenType {
    SemiColon,
    StringLiteral,
    DoubleQuoteStart,
    DoubleQuoteEnd,
    Ident,
    Number,
    Assign,
    // Bind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    pub ty: TokenType,
    pub value: String,
}

fn main() {
    let token = tokenize("x = 123232;").unwrap();

    println!("{:?}", token)
}
/**
 * tokenize 函数接受一个字符串 code 作为输入，并返回一个 Result<Vec<Token>, Errors> 类型的结果。它使用一个循环来遍历输入代码的每个字符，并根据字符的类型将其转换为一个或多个标记。

在循环中，我们使用一个 current_token 变量来跟踪当前正在解析的标记。
如果我们遇到一个分号或等号，我们会将当前标记添加到标记列表中，并创建一个新的标记来表示分号或等号。
如果我们遇到一个字母或下划线，
我们会将其添加到当前标记的值中。如果当前标记为空，我们会创建一个新的标记来表示标识符。
 *
 *
 *
 *
 *
 *
 */

fn tokenize(code: &str) -> Result<Vec<Token>, Errors> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token: Option<Token> = None;
    // 遍历输入代码的每个字符

    for c in code.chars() {
        println!("current char is:{} ", c);
        if tokens.last().is_some()
            && tokens.last().unwrap().ty == TokenType::DoubleQuoteStart
            && c != '"'
        {
            if let Some(tok) = &mut current_token {
                tok.value.push(c);
                println!("tok_clone:{:?}", tok.clone());
            } else {
                current_token = Some(Token {
                    ty: TokenType::StringLiteral,
                    value: c.to_string(),
                })
            }
        } else if c == ';' {
            if let Some(tok) = &current_token {
                tokens.push(tok.clone());
                println!("tok_clone:{:?}", tok.clone());
            }
            tokens.push(Token {
                ty: TokenType::SemiColon,
                value: String::from(";"),
            });

            current_token = None;
        } else if c == '=' {
            if let Some(tok) = &current_token {
                tokens.push(tok.clone())
            }
            tokens.push(Token {
                ty: TokenType::Assign,
                value: String::from("="),
            })
        } else if tokens.last().is_some() && tokens.last().unwrap().ty == TokenType::DoubleQuoteEnd
        {
            if let Some(tok) = &mut current_token {
                tok.value.push(c)
            } else {
                current_token = Some(Token {
                    ty: TokenType::StringLiteral,
                    value: String::from(c.to_string()),
                })
            }
        } else if (c >= 'A' && c <= 'z')
            || c == '_'
            || (current_token.is_some()
                && current_token.clone().unwrap().ty == TokenType::Ident
                && c != ' ')
        {
            if let Some(tok) = &mut current_token {
                println!("tok:{:?}", tok);
                tok.value.push(c);

                println!("tok:{:?}", tok);
            } else {
                current_token = Some(Token {
                    ty: TokenType::Ident,
                    value: String::from(c.to_string()),
                })
            }
        } else if c == ' ' {
            if let Some(tok) = &current_token {
                tokens.push(tok.clone())
            }
            current_token = None;
        } else if c >= '0' && c <= '9' {
            if let Some(tok) = &mut current_token {
                tok.value.push(c)
            } else {
                current_token = Some(Token {
                    ty: TokenType::Number,
                    value: String::from(c.to_string()),
                })
            }
        } else if c == '"' {
            println!("7");
            if current_token.is_some()
                && current_token.clone().unwrap().ty != TokenType::StringLiteral
            {
                return Err(Errors::CannotCreateStringWhileInOtherToken);
            } else if current_token.is_some()
                && current_token.clone().unwrap().ty == TokenType::StringLiteral
            {
                tokens.push(current_token.clone().unwrap());
                tokens.push(Token {
                    ty: TokenType::DoubleQuoteEnd,
                    value: String::from("\""),
                });

                current_token = None
            } else {
                tokens.push(Token {
                    ty: TokenType::DoubleQuoteStart,
                    value: String::from("\""),
                })
            }
        }
    }

    if let Some(tok) = current_token {
        tokens.push(tok)
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    // <T: Eq> 是一个泛型类型参数，用于指定 Vec 中元素的类型必须实现了 Eq trait。
    fn eq_vecs<T: Eq>(v1: Vec<T>, v2: Vec<T>) -> bool {
        if v1.len() != v2.len() {
            return false;
        }
        for i in 0..v1.len() {
            if v1[i] != v2[i] {
                return false;
            }
        }
        return true;
    }

    use super::*;
    #[test]
    fn number_token() {
        let tokens = tokenize("123");
        // 用于检查 tokens 是否为 Ok 枚举值，
        assert!(tokens.is_ok());

        let tokens = tokens.unwrap();
        assert!(eq_vecs(
            tokens,
            vec![Token {
                ty: TokenType::Number,
                value: String::from("123")
            }]
        ))
    }

    #[test]
    fn test_assign_number() {
        let tokens = tokenize("x =  123;");
        // 用于检查 tokens 是否为 Ok 枚举值，
        assert!(tokens.is_ok());

        let tokens = tokens.unwrap();
        assert!(eq_vecs(
            tokens,
            vec![
                Token {
                    ty: TokenType::Ident,
                    value: String::from("x")
                },
                Token {
                    ty: TokenType::Assign,
                    value: String::from("=")
                },
                Token {
                    ty: TokenType::Number,
                    value: String::from("123")
                },
                Token {
                    ty: TokenType::SemiColon,
                    value: String::from(";")
                }
            ]
        ));
    }

    #[test]
    fn test_assign_string() {
        let tokens = tokenize("x = \"af\";");
        // 用于检查 tokens 是否为 Ok 枚举值，
        assert!(tokens.is_ok());

        let tokens = tokens.unwrap();
        println!("{:?}", tokens);
        assert!(eq_vecs(
            tokens,
            vec![
                Token {
                    ty: TokenType::Ident,
                    value: String::from("x")
                },
                Token {
                    ty: TokenType::Assign,
                    value: String::from("=")
                },
                Token {
                    ty: TokenType::DoubleQuoteStart,
                    value: String::from("\"")
                },
                Token {
                    ty: TokenType:: StringLiteral,
                    value: String::from("af")
                },
                Token {
                    ty: TokenType::DoubleQuoteEnd,
                    value: String::from("\"")
                },
                Token {
                    ty: TokenType::SemiColon,
                    value: String::from(";")
                },
            ]
        ));
    }
}
