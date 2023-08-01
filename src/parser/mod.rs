#![allow(dead_code)]
/*TODO
    - for
        - c syntax
        - foreach
        - while syntax
    - interface
    - operator expressions
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
    // 首先，枚举类型ParseErr有两种可能的值：Unexpected和Unknown。

    // 这种类型的错误包含三个部分的信息：期望的内容（类型为String），实际找到的内容（类型为String），以及错误发生的位置（类型为u64）。
    Unexpected(String, String, u64),
    // Unknown: 这种类型的错误包含一个消息，这个消息是一个String，描述了未知的错误内容。
    Unknown(String),
}

// std::fmt::Display trait。这个trait是Rust标准库中用于处理字符串显示的trait。实现Display trait就是为了自定义ParseErr枚举的字符串显示方式。
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

fn any_whitespace() -> impl Fn(String) -> ParseResult {
    let sp = parse_char(' ');
    let tab = parse_char('\t');
    let newline = parse_char('\n');
    return any_of(vec![sp, tab, newline]);
}
fn whitespace() -> impl Fn(String) -> ParseResult {
    return zero_or_more(any_whitespace());
}

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

fn one_or_more(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let mut result = Vec::new();

        // we should first try to get one, if can't it's a parse error
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
            result.push(parsed);
        }
        return Ok((input.clone(), ParseObj::List(result)));
    };
}
fn parse_chars(chars: &str) -> impl Fn(String) -> ParseResult {
    let parsers = chars.chars().map(|c| parse_char(c)).collect();

    let res1 = chars.chars().map(|c| parse_char(c));
    println!("chars.chars().map(|c| parse_char {:?}", res1);

    return any_of(parsers);
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

fn decl(mut input: String) -> ParseResult {
    // ident: expr = expr;
    // 1.去掉前面的换行空格和缩进
    let (remains, _) = whitespace()(input.clone())?;

    println!("whitespace remains{:?}", remains);
    // 2.获取变量
    let (remains, obj) = ident(remains)?;
    println!("ident remains{:?}", remains);
    let mut identifier = "".to_string();
    match obj {
        ParseObj::Ident(i) => identifier = i,
        _ => {
            return Err(ParseErr::Unexpected(
                "ident".to_string(),
                format!("{:?}", obj),
                0,
            ))
        }
    }
    println!("ident: {} remains: \"{}\"", identifier, remains);
    // 继续去掉空格
    let (mut remains, _) = whitespace()(remains)?;
    let mut ty: Option<ParseObj> = None;
    let colon_res = parse_char(':')(remains.clone());
    println!("colon_res: {:?}", colon_res);
    match colon_res {
        Ok((r, ParseObj::Char(':'))) => {
            let ty_res = expr(r)?;
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
    // bool
    // ident
    // String
    // int, uint, float
    // fn_call
    // fn_def
    // if
    let parsers: Vec<fn(String) -> Result<(String, ParseObj), ParseErr>> = vec![bool, ident];
    return any_of(parsers)(input);
}

#[test]
fn test_parse_decl_bool() {
    let decl_res = decl("a = false".to_string());
    assert!(decl_res.is_ok());
    let none: Box<Option<ParseObj>> = Box::new(None);
    if let (_, ParseObj::Decl(name, none, be)) = decl_res.unwrap() {
        assert_eq!(name, "a");
        assert_eq!(be, Box::new(ParseObj::Bool(false)));
    } else {
        assert!(false);
    }
}

#[test]
fn test_parse_decl_int() {
    let decl_res = decl("a = -2".to_string());
    assert!(decl_res.is_ok());
    let none: Box<Option<ParseObj>> = Box::new(None);
    if let (_, ParseObj::Decl(name, none, be)) = decl_res.unwrap() {
        assert_eq!(name, "a");
        assert_eq!(be, Box::new(ParseObj::Int(-2)));
    } else {
        assert!(false);
    }
}
