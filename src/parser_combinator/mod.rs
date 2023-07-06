#[derive(Debug, PartialEq)]
enum ParseObj {
    Char(char),
    Uint(usize),
    Int(isize),
    Str(String),
    Keyword(String),
    Bool(bool),
    List(Vec<ParseObj>),
    Float(f64),
    Empty,
    Ident(String),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseErr {
    msg: String,
}

impl ParseErr {
    pub fn wrap(msg: &str, inner: ParseErr) -> Self {
        return Self {
            msg: format!("{}: {}", msg, inner),
        };
    }
    pub fn new(msg: &str) -> Self {
        return Self {
            msg: String::from(msg),
        };
    }
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.msg))
    }
}
impl std::error::Error for ParseErr {}

type ParseResult = Result<(String, ParseObj), ParseErr>;

fn parse_char(c: char) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        if input.len() < 1 {
            return ParseResult::Err(ParseErr::new("expected at least one character"));
        }

        if input.chars().nth(0).unwrap() == c {
            return ParseResult::Ok((input[1..].to_string(), ParseObj::Char(c)));
        }

        return ParseResult::Err(ParseErr::new("parse_char error"));
    };
}

fn parse_chars(chars: String) -> impl Fn(String) -> ParseResult {
    let parsers = chars.chars().map(|c: char| parse_char(c)).collect();
    return any_of(parsers);
}

fn any_of(parses: Vec<impl Fn(String) -> ParseResult>) -> impl Fn(String) -> ParseResult {
    println!("len of parsers created {}", parses.len());
    return move |input: String| {
        for parser in parses.iter() {
            let res = parser(input.clone());
            match res {
                Ok((remains, parsed)) => return Ok((remains, parsed)),
                Err(err) => continue,
            }
        }
        return Err(ParseErr::new("any_of error"));
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
                return Err(ParseErr::wrap("one_or_more err", err));
            }
        }
        while let Ok((remains, parsed)) = parser(input.clone()) {
            input = remains;
            result.push(parsed);
        }
        return Ok((input.clone(), ParseObj::List(result)));
    };
}

fn uint(input: String) -> ParseResult {
    match one_or_more(digit)(input) {
        Ok((remains, ParseObj::List(_digits))) => {
            let mut number = String::new();
            for digit in _digits {
                match digit {
                    ParseObj::Char(c) => number.push(c),
                    _ => unreachable!(),
                }
            }
            let _number: usize = number.parse().unwrap();
            Ok((remains, ParseObj::Uint(_number)))
        }
        Err(err) => return Err(err),
        _ => unreachable!(),
    }
}

fn zero_or_one(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        if let Ok((remains, parsed)) = parser(input.clone()) {
            return Ok((remains, ParseObj::Char('-')));
        }
        return Ok((input, ParseObj::Empty));
    };
}
fn int(input: String) -> ParseResult {
    let sign = zero_or_one(parse_char('-'));

    match sign(input.clone()) {
        Ok((input, ParseObj::Char('-'))) => match uint(input) {
            Ok((remains, ParseObj::Uint(num))) => {
                return Ok((remains, ParseObj::Int(-1 * num as isize)));
            }
            _ => Err(ParseErr::new("Err")),
        },
        Ok((input, ParseObj::Empty)) => match uint(input) {
            Ok((remains, ParseObj::Uint(num))) => {
                return Ok((remains, ParseObj::Int(num as isize)));
            }
            _ => Err(ParseErr::new("Err")),
        },
        _ => Err(ParseErr::new("Err")),
    }
}

fn digit(input: String) -> ParseResult {
    return parse_chars("0123456789".to_string())(input);
}

fn ident(input: String) -> ParseResult {
    match one_or_more(parse_chars(
        "abcdefghijklmnopqrstuvwxzABCDEFGHIJKLMNOPQRSTUVWXZ_".to_string(),
    ))(input)
    {
        Ok((remains, ParseObj::List(str_lists))) => {
            let mut name = String::new();
            for po in str_lists.iter() {
                match po {
                    ParseObj::Char(c) => name.push(c.clone()),
                    _ => return Err(ParseErr::new("ident should be valid")),
                }
            }
            return Ok((remains, ParseObj::Ident(name)));
        }
        Ok(_) => return Err(ParseErr::new("idents should be valid")),
        Err(err) => return Err(ParseErr::new("ident err")),
    }
}

fn float(input: String) -> ParseResult {
    if let (remains, ParseObj::Int(int_part)) = int(input)? {
        println!("floats1: {:?},{:?}", remains, int_part);
        if let (remains, _) = parse_char('.')(remains)? {
            println!("float2: {:?},{:?}", remains, int_part);
            if let (remains, ParseObj::Uint(float_part)) = uint(remains)? {
                println!("floats3: {:?},{:?}", remains, float_part);
                let float_str = format!("{}.{}", int_part, float_part);
                let float: f64 = float_str.parse().unwrap();
                Ok((remains, ParseObj::Float(float)))
            } else {
                return Err(ParseErr::new("some"));
            }
        } else {
            return Err(ParseErr::new("some"));
        }
    } else {
        return Err(ParseErr::new("some"));
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

    let (remains, bool_parsed) = any_of(vec![_true, _false])(input).unwrap();

    if let ParseObj::Keyword(key) = bool_parsed {
        return Ok((remains, ParseObj::Bool(key == "true")));
    } else {
        unreachable!()
    }
}

fn expr(input: String) -> ParseResult {
    let parsers: Vec<fn(String) -> Result<(String, ParseObj), ParseErr>> =
        vec![float, uint, int, bool, ident];
    return any_of(parsers)(input);
}

#[test]
fn test_parse_sigle_digits() {
    assert_eq!(
        digit("1AB".to_string()),
        ParseResult::Ok(("AB".to_string(), ParseObj::Char('1')))
    );
}

#[test]
fn test_parse_uint() {
    assert_eq!(
        uint("1234AB".to_string()),
        ParseResult::Ok(("AB".to_string(), ParseObj::Uint(1234)))
    );
}

#[test]
fn test_parse_int() {
    assert_eq!(
        int("-1234AB".to_string()),
        ParseResult::Ok(("AB".to_string(), ParseObj::Int(-1234)))
    );
    assert_eq!(
        int("1234AB".to_string()),
        ParseResult::Ok(("AB".to_string(), ParseObj::Int(1234)))
    );
}

#[test]
fn test_parse_float() {
    assert_eq!(
        float("4.2".to_string()),
        ParseResult::Ok(("".to_string(), ParseObj::Float(4.2)))
    );
}

#[test]
fn test_parse_ident() {
    assert_eq!(
        ident("name".to_string()),
        ParseResult::Ok(("".to_string(), ParseObj::Ident("name".to_string())))
    );
    assert_eq!(
        ident("name_str".to_string()),
        ParseResult::Ok(("".to_string(), ParseObj::Ident("name_str".to_string()),))
    );
}

#[test]
fn test_parse_bool() {
    assert_eq!(
        bool("truesomeshitaftertrue".to_string()),
        ParseResult::Ok(("someshitaftertrue".to_string(), ParseObj::Bool(true)))
    );
    assert_eq!(
        bool("falsesomeshitaftertrue".to_string()),
        ParseResult::Ok(("someshitaftertrue".to_string(), ParseObj::Bool(false),))
    );
}
#[test]
fn test_parse_expr() {
    assert_eq!(
        expr("true".to_string()),
        ParseResult::Ok(("".to_string(), ParseObj::Bool(true)))
    );
    assert_eq!(
        expr("false".to_string()),
        ParseResult::Ok(("".to_string(), ParseObj::Bool(false)))
    );
    assert_eq!(
        expr("12".to_string()),
        ParseResult::Ok(("".to_string(), ParseObj::Uint(12)))
    );
    assert_eq!(
        expr("-12".to_string()),
        ParseResult::Ok(("".to_string(), ParseObj::Int(-12)))
    );
    assert_eq!(
        expr("12.2".to_string()),
        ParseResult::Ok(("".to_string(), ParseObj::Float(12.2)))
    );
    assert_eq!(
        expr("-12.2".to_string()),
        ParseResult::Ok(("".to_string(), ParseObj::Float(-12.2)))
    );
}
