#[derive(Clone, Debug, PartialEq)]

pub enum ParseObj {
    Char(char),
    Unit(usize),
    Int(isize),
    Float(f64),
    Str(String),
    Keyword(String),
    Ident(String),
    Bool(bool),
    List(Vec<ParseObj>),
    // Box用于在堆上分配空间并存储值，这在你需要存储大型数据或具有递归数据类型的时候特别有用。
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

        return ParseResult::Err(ParseErr::Unexpected("".to_string(), "".to_string(), 0));
    };
}

fn whitespace() -> impl Fn(String) -> ParseResult {
    let sp = parse_char(' ');
    let tab = parse_char('\t');
    let newline = parse_char('\n');

    return any_of(vec![sp, tab, newline]);
}

fn one_or_more(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let mut result = Vec::new();

        match parser(input.clone()) {
            Ok((remains, parsed)) => {
                input = remains;
                result.push(parsed)
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
fn parse_chars(chars: &str) -> impl Fn(String) -> ParseResult {
    let parsers = chars.chars().map(|c| parse_char(c)).collect();

    return any_of(parsers);
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
                            " a char".to_string(),
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

fn decl(mut input: String) {
    // 去掉变量前面的空格
    let (remains, _) = whitespace()(input.clone()).unwrap();
    println!("whitespace remains{:?}", remains);

    let (remains, obj) = ident(remains).unwrap();
    println!("ident remains{:?}", remains);
}

#[test]
fn test_parse_decl_bool() {
    let decl_res = decl(" a = false".to_string());

    // assert!(decl_res.is_ok());

    // let noen: Box<Option<ParseObj>> = Box::new(None);

    // if let (_, ParseObj::Decl(name, none, be)) = decl_res.unwrap() {
    //     assert_eq!(name, "a");
    //     assert_eq!(be, Box::new(ParseObj::Bool(false)));
    // } else {
    //     assert!(false)
    // }
}
