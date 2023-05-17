#[derive(Debug)]
enum Errors {}

#[derive(PartialEq, Eq, Debug, Clone)]
enum TokenType {
    SemiColon,
    StringLiteral,
    DoubleQuote,
    Ident,
    Number,
}

#[derive(Debug, Clone)]
struct Token {
    pub ty: TokenType,
    pub value: String,
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
        if c == ';' {
            if let Some(tok) = &current_token {
                tokens.push(tok.clone())
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
                ty: TokenType::SemiColon,
                value: String::from(";"),
            })
        } else if tokens.last().is_some() && tokens.last().unwrap().ty == TokenType::DoubleQuote {
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
            || (current_token.is_some() && current_token.clone().unwrap().ty == TokenType::Ident)
        {
            if let Some(tok) = &mut current_token {
                tok.value.push(c)
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
        }
    }

    Ok(tokens)
}

fn main() {
    let token = tokenize("x=2;").unwrap();

    println!("{:?}", token)
}
