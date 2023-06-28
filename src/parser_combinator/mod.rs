#[derive(Debug, PartialEq, Eq)]
enum ParseObj {
    Char(char),
    Uint(usize),
    Int(isize),
    Str(String),
    Keyword(String),
    Bool(bool),
    List(Vec<ParseObj>),
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

fn zero_or_one(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        let mut result = Vec::new();
        if let Ok((remains, parsed)) = parser(input.clone()) {
            input = remains;
            result.push(parsed);
        }
        println!("zero_or_one: {} {:?}", input.clone(), result);
        return Ok((input.clone(), ParseObj::List(result)));
    };
}

fn any_of(parsers: Vec<impl Fn(String) -> ParseResult>) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        for parse_char in parsers.iter() {
            let res = parse_char(input.clone());

            match res {
                Ok((remaing, parsed)) => return Ok((remaing, parsed)),
                Err(err) => continue,
            }
        }
        return ParseResult::Err(ParseErr::new("any_of err"));
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

                println!("one_or_more matc input,remains {},{:?}", input, result)
            }
            Err(err) => {
                return Err(ParseErr::wrap("one_or_more err", err));
            }
        }
        while let Ok((remains, parsed)) = parser(input.clone()) {
            input = remains;
            result.push(parsed);
            println!("one_or_more while input,result {} ,{:?}", input, result)
        }
        return Ok((input.clone(), ParseObj::List(result)));
    };
}

fn parse_char(c: char) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        if input.len() < 1 {
            ParseResult::Err(ParseErr::new("expected a char"));
        }

        if input.chars().nth(0).unwrap() == c.clone() {
            return ParseResult::Ok((input[1..].to_string(), ParseObj::Char(c)));
        }

        return ParseResult::Err(ParseErr::new("parse_char err"));
    };
}
fn uint(input: String) -> ParseResult {
    println!("uint_uint:{:?}", one_or_more(digit)(input.clone()));
    match one_or_more(digit)(input) {
        Ok((remains, ParseObj::List(_digits))) => {
            let mut number = String::new();
            println!("one_or_more_digits: {:?}", _digits);
            for d in _digits {
                match d {
                    ParseObj::Char(c) => {
                        number.push(c);
                    }
                    _ => unreachable!(),
                }
            }
            // 对于数字解析，可以使用 parse() 方法将字符串转换为对应的数值类型，例如 i32、f64
            let number: usize = number.parse().unwrap();
            println!("one_or_more_number: {:?}", number);
            Ok((remains, ParseObj::Uint(number)))
        }
        Err(err) => return Err(err),
        _ => unreachable!(),
    }
}

// 解析负数
fn int(input: String) -> ParseResult {
    let sign = zero_or_one(parse_char('-'));

    match sign(input) {
        Ok((input, _)) => match uint(input) {
            Ok((remains, ParseObj::Uint(num))) => {
                return Ok((remains, ParseObj::Int(-1 * num as isize)))
            }
            _ => Err(ParseErr::new("Err")),
        },
        _ => Err(ParseErr::new("Err")),
    }
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

fn keyword(word: String) -> impl Fn(String) -> ParseResult {
    return move |mut input: String| {
        for c in word.chars() {
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
    let (reamins, parsed_bool) = any_of(vec![_true, _false])(input).unwrap();
    if let ParseObj::Keyword(key) = parsed_bool {
        return Ok((reamins, ParseObj::Bool(key == "true")));
    } else {
        unreachable!()
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
fn test_parse_int() {
    assert_eq!(
        int("-1234AB".to_string()),
        ParseResult::Ok(("AB".to_string(), ParseObj::Int(-1234)))
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
fn test_parse_keyword() {
    assert_eq!(
        keyword("struct".to_string())("struct name".to_string()),
        ParseResult::Ok((" name".to_string(), ParseObj::Keyword("struct".to_string())))
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
