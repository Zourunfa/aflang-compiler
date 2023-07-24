// #[cfg(test)]
/**
Rust的闭包是一种可以捕获其环境并作为匿名函数使用的特殊函数类型。闭包可以在定义时捕获外部变量，并在后续调用中访问和修改这些变量，即使这些变量在闭包被创建时已经超出了其作用域。

闭包在Rust中的语法与普通函数相似，但有一些额外的功能和限制。闭包可以通过 move 关键字来强制所有捕获的变量的所有权转移给闭包，这在需要在闭包中使用捕获的变量的所有权时非常有用。闭包还可以使用 Fn、FnMut 或 FnOnce trait
 来指定其对捕获变量的访问方式，这允许闭包在调用时对捕获的变量进行不同程度的修改。
Rust的闭包也是类型安全的，它们的类型是由编译器自动推断的。闭包可以像普通函数一样作为参数传递给其他函数，也可以在需要函数类型的地方直接使用。闭包还可以通过使用 || 语法来定义多个参数，并使用 |...| 语法来指定参数的模式匹配。
总体来说，Rust的闭包提供了一种灵活且安全的方式来处理需要捕获上下文的匿名函数，使得代码更加简洁和可读。
 */

#[derive(Clone, Debug, PartialEq)]
pub enum ParseObj {
    Char(char),
    Uint(usize),
    Int(isize),
    Float(f64),
    Str(String),
    Keyword(String),
    Ident(String),
    Bool(bool),
    List(Vec<ParseObj>),
    Decl(String, Box<Option<ParseObj>>, Box<ParseObj>),
    FnCall(String, Vec<ParseObj>),
    Struct(Vec<(ParseObj, ParseObj)>),
    Fn(Vec<(ParseObj, ParseObj)>, Box<ParseObj>, Box<ParseObj>),
    Array(Box<Option<ParseObj>>, Box<ParseObj>),
    Stmt(Box<ParseObj>),
    Block(Vec<ParseObj>),
    If(Box<ParseObj>, Box<ParseObj>),
    ForC(Box<ParseObj>, Box<ParseObj>, Box<ParseObj>, Box<ParseObj>),
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseErr {
    // unexpected (expected, found, location)
    Unexpected(String, String, u64),
    Unknown(String),
}
impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unexpected(_, _, _) => f.write_fmt(format_args!("{:?}", self)),
            Self::Unknown(msg) => f.write_fmt(format_args!("{}", msg)),
            _ => unreachable!(),
        }
    }
}
impl std::error::Error for ParseErr {}

type ParseResult = Result<(String, ParseObj), ParseErr>;

fn parse_char(c: char) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        if input.len() < 1 {
            return ParseResult::Err(ParseErr::Unexpected(
                c.to_string(),
                "nothing".to_string(),
                0,
            ));
        }
        if input.chars().nth(0).unwrap() == c.clone() {
            return ParseResult::Ok((input[1..].to_string(), ParseObj::Char(c)));
        }
        return ParseResult::Err(ParseErr::Unexpected(
            c.to_string(),
            input.chars().nth(0).unwrap().to_string(),
            0,
        ));
    };
}

fn any_of(parsers: Vec<impl Fn(String) -> ParseResult>) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        for parser in parsers.iter() {
            match parser(input.clone()) {
                Ok((remaining, parsed)) => return Ok((remaining, parsed)),
                Err(err) => continue,
            }
        }
        return Err(ParseErr::Unexpected("".to_string(), "".to_string(), 0));
    };
}

fn any_whitespace() -> impl Fn(String) -> ParseResult {
    let sp = parse_char(' ');
    let tab = parse_char('\t');
    let newline = parse_char('\n');
    return any_of(vec![sp, tab, newline]);
}

fn zero_or_more(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let mut result = Vec::new();
        while let Ok((remains, parsed)) = parser(input.clone()) {
            input = remains;
            result.push(parsed);
        }
        return Ok((input.clone(), ParseObj::List(result)));
    };
}

fn whitespace() -> impl Fn(String) -> ParseResult {
    return zero_or_more(any_whitespace());
}

fn decl(mut input: String) {
    let (remains, _) = whitespace()(input.clone()).unwrap();
    println!("whitespace remains{:?}", remains);
}
#[test]
fn test_parse_decl_bool() {
    let decl_res = decl("a = false".to_string());
    // assert!(decl_res.is_ok());
    // let none: Box<Option<ParseObj>> = Box::new(None);
    // if let (_, ParseObj::Decl(name, none, be)) = decl_res.unwrap() {
    //     assert_eq!(name, "a");
    //     assert_eq!(be, Box::new(ParseObj::Bool(false)));
    // } else {
    //     assert!(false);
    // }
}
