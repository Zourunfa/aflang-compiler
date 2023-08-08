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
                c.to_string(),
                "nothing".to_string(),
                0,
            ));
        }

        if input.chars().nth(0).unwrap() == c {
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
                Ok((remains, parserd)) => return Ok((remains, parserd)),
                Err(err) => continue,
            }
        }

        return ParseResult::Err(ParseErr::Unexpected(
            input.to_string(),
            "nothing".to_string(),
            0,
        ));
    };
}

fn zero_or_more(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let mut result = Vec::new();
        while let Ok((remains, parserd)) = parser(input.clone()) {
            input = remains;
            result.push(parserd);
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

fn one_or_more(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let mut result = Vec::new();
        match parser(input.clone()) {
            Ok((remains, parsed)) => {
                input = remains;
                result.push(parsed)
            }
            Err(e) => return Err(e),
        }

        while let Ok((remains, parsed)) = parser(input.clone()) {
            input = remains;
            result.push(parsed)
        }

        return Ok((input.clone(), ParseObj::List(result)));
    };
}

fn parse_chars(chars: String) -> impl Fn(String) -> ParseResult {
    let parsers = chars.chars().map(|c| parse_char(c)).collect();
    return any_of(parsers);
}

fn ident(input: String) -> ParseResult {
    match one_or_more(parse_chars(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_".to_string(),
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

fn whitespace() -> impl Fn(String) -> ParseResult {
    return zero_or_more(any_whitespace());
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
    let (remains, _) = whitespace()(input.clone()).unwrap();
    println!("whitespace remains{:?}", remains);

    let (remains, obj) = ident(remains).unwrap();
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
    let (mut remains, _) = whitespace()(remains).unwrap();
    let mut ty: Option<ParseObj> = None;
    // let colon_res = parse_char(':')(remains.clone());

    // match colon_res {
    //   Ok((r,ParseObj::Char(':')))=>{
    // let ty_res = expr(r)?;
    // remains = ty_res.0;
    // ty = Some(ty_res.1);
    //   }
    // }
    let (remains, _) = parse_char('=')(remains)?;
    println!("remains =: \"{}\"", remains);
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
