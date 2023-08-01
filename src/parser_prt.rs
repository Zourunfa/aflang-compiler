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
// ling
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

fn parse_char(c: char) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        if input.is_empty() {
            return ParseResult::Err(ParseErr::Unexpected(
                input.to_string(),
                "nothing".to_string(),
                0,
            ));
        }

        if input.chars().nth(0).unwrap() == c {
            return ParseResult::Ok((input[1..].to_string(), ParseObj::Char(c)));
        }

        return ParseResult::Err(ParseErr::Unexpected(
            c.to_string(),
            "nothing".to_string(),
            0,
        ));
    };
}

fn any_of(parsers: Vec<impl Fn(String) -> ParseResult>) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        for parser in parsers.iter() {
            match parser(input.clone()) {
                Ok((remains, parsed)) => return Ok((remains, parsed)),
                Err(err) => continue,
            }
        }
        return Err(ParseErr::Unexpected("".to_string(), "".to_string(), 0));
    };
}

fn any_whitespace() -> impl Fn(String) -> ParseResult {
    let sp = parse_char(' ');
    let tab = parse_char('\t');
    let new_line = parse_char('\n');

    return any_of(vec![sp, tab, new_line]);
}

fn whitespace() -> impl Fn(String) -> ParseResult {
    return zero_or_more(any_whitespace());
}

/**
闭包在 Rust 中类似于函数，但有一个特殊的能力，
即可以捕获其所在环境的变量，使其在闭包内部可用。在这里，
闭包 |c| parse_char(c) 捕获了外部的字符 c，并将其传递给 parse_char 函数，
从而生成了针对每个字符的解析器。因为闭包捕获了 c，
所以即使在闭包外部 c 的作用域结束后，闭包仍然可以使用 c 来生成解析器。
*/

fn parse_chars(chars: &str) -> impl Fn(String) -> ParseResult {
    // println!("chars.chars().map(|c| parse_char {:?}", chars.chars().map(|c| parse_char(c)));
    let parsers = chars.chars().map(|c| parse_char(c)).collect();
    return any_of(parsers);
}
// 一次或多次匹配，至少有一次匹配
/**
one_or_more 函数的使用确保了在解析标识符时，至少有一个合法的字符被匹配，
否则会返回错误。这样可以避免在代码中处理空标识符或非法标识符的情况。
*/
fn one_or_more(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let mut result = Vec::new();

        match parser(input.clone()) {
            Ok((remains, parsed)) => {
                input = remains;
                result.push(parsed);
            }
            Err(err) => {
                return Err(err);
            }
        }

        while let Ok((remains, parsed)) = parser(input.clone()) {
            input = remains;
            result.push(parsed)
        }

        return Ok((input.clone(), ParseObj::List(result)));
    };
}

fn ident(input: String) -> ParseResult {
    match one_or_more(parse_chars(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_",
    ))(input)
    {
        Ok((remains, ParseObj::List(chars_parse_objects))) => {
            let mut name = String::new();

            for po in chars_parse_objects {
                match po {
                    ParseObj::Char(c) => name.push(c),
                    _ => {
                        return Err(ParseErr::Unexpected(
                            "a char".to_string(),
                            format!("{:?}", po),
                            0,
                        ))
                    }
                }
            }
            return Ok((remains, ParseObj::Ident(name)));
        }
        Ok((_, obj)) => {
            return Err(ParseErr::Unexpected(
                "list of chars".to_string(),
                format!("{:?}", obj),
                0,
            ))
        }
        Err(err) => Err(err),
    }
}

fn keyword(word: String) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let word_chars = word.chars();
        for c in word_chars {
            match parse_char(c)(input) {
                Ok((remains, _)) => input = remains,
                Err(err) => return Err(err),
            }
        }
        return Ok((input, ParseObj::Keyword(word.clone())));
    };
}

fn bool(input: String) -> ParseResult {
    let _true = keyword("true".to_string());
    let _false = keyword("false".to_string());
    let (remains, bool_parsed) = any_of(vec![_true, _false])(input)?;

    if let ParseObj::Keyword(b) = bool_parsed {
        return Ok((remains, ParseObj::Bool(b == "true")));
    } else {
        unreachable!()
    }
}

fn expr(input: String) -> ParseResult {
    let parsers: Vec<fn(String) -> Result<(String, ParseObj), ParseErr>> = vec![bool, ident];
    return any_of(parsers)(input);
}

fn decl(mut input: String) -> ParseResult {
    // 1.去掉前面的换行空格和缩进
    let (remains, _) = whitespace()(input.clone()).unwrap();

    println!("whitespace remains{:?}", remains);
    // 2.获取变量
    let (remains, obj) = ident(remains).unwrap();
    println!("ident remains{:?}", remains);
    let mut identifier = "".to_string();
    match obj {
        ParseObj::Ident(i) => identifier = i,
        _ => {
            // return Err(ParseErr::Unexpected(
            //     "ident".to_string(),
            //     format!("{:?}", obj),
            //     0,
            // ))
        }
    }
    println!("ident: {} remains: \"{}\"", identifier, remains);
    // 继续去掉空格
    let (mut remains, _) = whitespace()(remains).unwrap();

    let mut ty: Option<ParseObj> = None;
    let colon_res = parse_char(':')(remains.clone());
    println!("colon_res: {:?}", colon_res);
    match colon_res {
        Ok((r, ParseObj::Char(':'))) => {
            let ty_res = expr(r).unwrap();
            remains = ty_res.0;
            ty = Some(ty_res.1);
        }
        _ => {}
    }

    let (remains, _) = parse_char('=')(remains)?;
    let (remains, _) = whitespace()(remains)?;
    println!("remains: \"{}\"", remains);
    let (remains, e) = expr(remains)?;
    println!("expr: {:?} remains: \"{}\"", e, remains);
    return Ok((
        remains,
        ParseObj::Decl(identifier, Box::new(ty), Box::new(e)),
    ));
}

#[test]
fn test_parse_decl_bool() {
    let decl_res = decl("\n a = false".to_string());

    assert!(decl_res.is_ok());
    let none: Box<Option<ParseObj>> = Box::new(None);
    if let (_, ParseObj::Decl(name, none, be)) = decl_res.unwrap() {
        assert_eq!(name, "a");
        assert_eq!(be, Box::new(ParseObj::Bool(false)));
    } else {
        assert!(false);
    }
}
