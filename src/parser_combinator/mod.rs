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
        }
    }
    pub fn new(msg: &str) -> Self {
        return Self { msg: String::from(msg) };
    }
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.msg))
    }
}
impl std::error::Error for ParseErr {}

type ParseResult = Result<(String, ParseObj), ParseErr>;





// 'a 生命周期的作用是指明输入闭包的输入参数 input 的生命周期，
// 并且要求返回的闭包的生命周期和输入闭包的生命周期相同。

fn parse_char<'a>(c: char) -> impl Fn(String) -> ParseResult {
  return move |input: String| {
      if input.len() < 1 {
          return ParseResult::Err(ParseErr::new("expected at least on char"));
      }
      if input.chars().nth(0).unwrap() == c.clone() {
          return ParseResult::Ok((input[1..].to_string(), ParseObj::Char(c)));
      }
      return ParseResult::Err(ParseErr::new(
          &format!("expected {} saw {}", c, input.chars().nth(0).unwrap()),
      ));
  };
}


/*
这段代码中使用input.clone()的目的是为了在每次迭代中对输入字符串进行复制，以保留原始的输入字符串。这样做是因为闭包中的input参数是通过移动(move)到闭包中获取所有权的，而在每次迭代中需要对它进行多次使用。

Rust中，闭包会捕获其环境中的变量，这些变量在闭包内部使用时需要遵循所有权规则。在这种情况下，由于闭包参数是通过移动获取所有权的，所以闭包在每次迭代时会消耗该所有权，并且无法再次使用原始的input值。
*/
fn any_of<'a>(parsers: Vec<impl Fn(String) -> ParseResult>) -> impl Fn(String) -> ParseResult {
  return move |input: String| {
      for parser in parsers.iter() {
          let res = parser(input.clone());
          match res {
              Ok((remaining, parsed)) => return Ok((remaining, parsed)),
              Err(err) => continue,
          }
      }
      return Err(ParseErr::new("any_of err"));
  };
}

fn digit(input:String) ->ParseResult{
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






#[test]
fn test_parse_single_digit(){
  // 在Rust中，双引号（"）用于表示字符串字面值，而单引号（'）用于表示字符字面值。
  assert_eq!(digit("1AB".to_string()),ParseResult::Ok(("AB".to_string(),ParseObj::Char('1'))));
  
}
