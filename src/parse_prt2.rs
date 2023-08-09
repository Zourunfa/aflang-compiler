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
                Ok((remains, parsedObj)) => return Ok((remains, parsedObj)),
                Err(err) => continue,
            }
        }
        return Err(ParseErr::Unexpected("".to_string(), "".to_string(), 0));
    };
}

fn any_whitespace() -> impl Fn(String) -> ParseResult {
    let space = parse_char(' ');
    let tab = parse_char('\t');
    let newline = parse_char('\n');

    return any_of(vec![space, tab, newline]);
}

fn zero_or_more(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let mut result = Vec::new();

        while let Ok((remains, parsedObj)) = parser(input.clone()) {
            input = remains;
            result.push(parsedObj);
        }

        return ParseResult::Ok((input.clone(), ParseObj::List(result)));
    };
}

fn whitespace() -> impl Fn(String) -> ParseResult {
    return zero_or_more(any_whitespace());
}

fn one_or_more(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let mut result = Vec::new();
        match parser(input.clone()) {
            Ok((remains, parsedObj)) => {
                input = remains;
                result.push(parsedObj);
            }
            Err(err) => {
                return Err(err);
            }
        }

        while let Ok((remains, parsedObj)) = parser(input.clone()) {
            input = remains;
            result.push(parsedObj);
        }

        return ParseResult::Ok((input.clone(), ParseObj::List(result)));
    };
}

fn ident(input: String) -> ParseResult {
    match one_or_more(parse_chars(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_",
    ))(input)
    {
        Ok((remains, parsedObj)) => {
            let name = String::new();
            for po in parsedObj {
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

            return Ok((remains, ParseObj::ident(name)));
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
    // 线性从左至右解析

    // 去掉空格制表符，缩进等
    let (remains, _) = whitespace()(input.clone()).unwrap();

    println!("whitespace remains{:?}", remains);

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

    let mut ty:Option<ParseObj> = None;
    

}

#[test]
fn test_decl_bool() {
    let decl_res = decl(" \n\t a = false".to_string());
    // assert!(decl_res.is_ok());

    // let none: Box<Option<ParseObj>> = Box::new(None);
    // if let (_, ParseObj::Decl(name, none, be)) = decl_res.unwrap() {
    //     assert_eq!(name, "a");
    //     assert_eq!(be, Box::new(ParseObj::Bool(false)));
    // } else {
    //     assert!(false)
    // }
}
