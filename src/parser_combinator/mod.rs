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
            return ParseResult::Err(ParseErr::new("expeted a char"));
        }

        if input.chars().nth(0).unwrap() == c.clone() {
            return ParseResult::Ok((input[1..].to_string(), ParseObj::Char(c)));
        }

        return ParseResult::Err(ParseErr::new("has a parse char err"));
    };
}

fn any_of(parsers: Vec<impl Fn(String) -> ParseResult>) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        for parser in parsers.iter() {
            let res = parser(input.clone());

            match res {
                Ok((remains, Parsed)) => return ParseResult::Ok((remains, Parsed)),
                Err(err) => continue,
            }
        }
        return ParseResult::Err(ParseErr::new("any_of err"));
    };
}

fn one_of_more(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let mut res = Vec::new();

        match parser(input.clone()) {
            Ok((remains, Parsed)) => {
                input = remains;
                res.push(Parsed);
            }
            Err(err) => {
                return Err(ParseErr::wrap("one_or_more err", err));
            }
        }

        while let Ok((remains, parsed)) = parser(input.clone()) {
            input = remains;
            res.push(parsed);
        }

        return Ok((input.clone(), ParseObj::List(res)));
    };
}

fn one_or_zero(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        if let Ok((remains, parsed)) = parser(input.clone()) {
            return Ok((remains, ParseObj::Char('-')));
        }
        return Ok((input, ParseObj::Empty));
    };
}

fn digit(input: String) -> ParseResult {
    return any_of(vec![
        parse_char('0'),
        parse_char('1'),
        parse_char('2'),
        parse_char('3'),
        parse_char('4'),
        parse_char('5'),
        parse_char('6'),
        parse_char('7'),
        parse_char('8'),
        parse_char('9'),
    ])(input);
}

fn uint(input: String) -> ParseResult {
    match one_of_more(digit)(input) {
        Ok((reamins, ParseObj::List(parser_vec))) => {
            let mut number = String::new();
            for d in parser_vec {
                match d {
                    ParseObj::Char(c) => {
                        number.push(c);
                    }
                    _ => unreachable!(),
                }
            }
            let number: usize = number.parse().unwrap();
            Ok((reamins, ParseObj::Uint(number)))
        }
        Err(err) => return Err(err),
        _ => unreachable!(),
    }
}

fn int(input: String) -> ParseResult {
    let sign = one_or_zero(parse_char('-'));

    match sign(input.clone()) {
        Ok((input, ParseObj::Char('-'))) => match uint(input) {
            Ok((remains, ParseObj::Uint(number))) => {
                return Ok((remains, ParseObj::Int(-1 * number as isize)))
            }
            _ => Err(ParseErr::new("uint err")),
        },
        Ok((input, ParseObj::Empty)) => match uint(input) {
            Ok((remains, ParseObj::Uint(number))) => {
                return Ok((remains, ParseObj::Int(number as isize)))
            }
            _ => Err(ParseErr::new("uint err")),
        },
        _ => Err(ParseErr::new("uint err")),
    }
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
// #[test]
// fn test_parse_keyword() {
//     assert_eq!(
//         keyword("struct".to_string())("struct name".to_string()),
//         ParseResult::Ok((" name".to_string(), ParseObj::Keyword("struct".to_string())))
//     );
// }

// #[test]
// fn test_parse_bool() {
//     assert_eq!(
//         bool("truesomeshitaftertrue".to_string()),
//         ParseResult::Ok(("someshitaftertrue".to_string(), ParseObj::Bool(true)))
//     );
//     assert_eq!(
//         bool("falsesomeshitaftertrue".to_string()),
//         ParseResult::Ok(("someshitaftertrue".to_string(), ParseObj::Bool(false),))
//     );
// }

// #[test]
// fn test_parse_float() {
//     assert_eq!(
//         float("4.2".to_string()),
//         ParseResult::Ok(("".to_string(), ParseObj::Float(4.2)))
//     );
// }
