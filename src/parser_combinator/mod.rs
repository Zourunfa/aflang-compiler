// #[derive(Debug, PartialEq, Eq)]
struct ParseErr {
    // struct ParseErr {
    msg: String, //     msg: String,
    loc: usize,  //     loc: usize,
} // }

impl ParseErr {
    // impl ParseErr {
    pub fn new(loc: usize, msg: String) -> Self {
        //     pub fn new(loc: usize, msg: String) -> Self {
        return Self { loc, msg }; //         return Self { loc, msg };
    } //     }
} // }

impl std::fmt::Display for ParseErr {
    // impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} at {}", self.msg, self.loc)) //         f.write_fmt(format_args!("{} at {}", self.msg, self.loc))
    } //     }
} // }
impl std::error::Error for ParseErr {} // impl std::error::Error for ParseErr {}

type ParseResult = Result<(String, String), ParseErr>; // type ParseResult = Result<(String, String), ParseErr>;

fn parse_char(c: char) -> impl Fn(String) -> ParseResult {
    // fn any_of<'a>(parsers: Vec<impl Fn(String) -> ParseResult>) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        //     return move |input: String| {
        if input.len() < 1 {
            //         for parser in parsers.iter() {
            return ParseResult::Err(ParseErr::new(0, "expected at least on char".to_string()));
            //             let res = parser(input.clone());
        } //             match res {
        if input.chars().nth(0).unwrap() == c.clone() {
            //                 Ok((remaining, parsed)) => return Ok((remaining, parsed)),
            return ParseResult::Ok((c.to_string(), input[1..].to_string())); //                 Err(err) => continue,
        } //             }
        return ParseResult::Err(ParseErr::new(
            //         }
            0, //         return Err(ParseErr::new(0, "combinator error".to_string()));
            format!("expected {} saw {}", c, input.chars().nth(0).unwrap()), //     };
        )); // }
    };
} // fn zero_or_more<'a>(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {

//     return move |mut input: String| {
fn any_of(parsers: Vec<impl Fn(String) -> ParseResult>) -> impl Fn(String) -> ParseResult {
    //         let mut result = String::new();
    return move |input: String| {
        //         while let Ok((remains, parsed)) = parser(input.clone()) {
        for parser in parsers.iter() {
            //             input = remains;
            let res = parser(input.clone()); //             result = format!("{}{}", result, parsed);
            match res {
                //         }
                Ok((parsed, remaining)) => return Ok((parsed, remaining)), //         return Ok((input.clone(), result));
                Err(err) => continue,                                      //     };
            } // }
        }
        return Err(ParseErr::new(0, "combinator error".to_string())); // fn one_or_more<'a>(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    }; //     return move |mut input: String| {
} //         let mut result = Vec::new();
fn zero_or_one(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {

    //         // we should first try to get one, if can't it's a parse error
} //         match parser(input.clone()) {

//             Ok((remains, parsed )) => {
fn zero_or_more(parser: impl Fn(String) -> ParseResult) -> impl Fn(String) -> ParseResult {
    //                 input = remains;
    return move |mut input: String| {
        //                 result.push(parsed);
        let mut result = Vec::new(); //             },
        while let Ok((parsed, remains)) = parser(input.clone()) {
            //             Err(err) => {
            input = remains; //                 return Err(err);
            result.push(parsed); //             }
        } //         }
        return Ok((result.concat(), input.clone())); //         while let Ok((remains, parsed)) = parser(input.clone()) {
    }; //             input = remains;
} //             result.push(parsed);

//         }
fn one_or_more(parser: impl Fn(String) -> ParseResult + Copy) -> impl Fn(String) -> ParseResult {
    //         return Ok((input.clone(), result));
    return move |mut input: String| {
        //     };
        let mut result = Vec::new(); // }

        // we should first try to get one, if can't it's a parse error	// fn parse_char<'a>(c: char) -> impl Fn(String) -> ParseResult {
        match parser(input.clone()) {
            //     return move |input: String| {
            Ok((parsed, remains)) => {
                //         if input.len() < 1 {
                input = remains; //             return ParseResult::Err(ParseErr::new(0, "expected at least on char".to_string()));
                result.push(parsed); //         }
            } //         if input.chars().nth(0).unwrap() == c.clone() {
            Err(err) => {
                //             return ParseResult::Ok((&c.to_string(), &input[1..].to_string()));
                return Err(err); //         }
            } //         return ParseResult::Err(ParseErr::new(
        } //             0,
          // other values are optional.	//             format!("expected {} saw {}", c, input.chars().nth(0).unwrap()),
        return zero_or_more(parser)(input); //         ));
    }; //     };
} // }

fn any_whitespace() -> impl Fn(String) -> ParseResult {
    // fn keyword<'a>(word: String) -> impl Fn(String) -> ParseResult {
    let sp = parse_char(' '); //    return move |mut input: String| {
    let tab = parse_char('\t'); //        let word_chars = word.chars();
    let newline = parse_char('\n'); //        for c in word_chars {
    return any_of(vec![sp, tab, newline]); //            match parse_char(c)(input) {
} //                Ok((remains, _)) => input = remains,

//                Err(err) => return Err(err),
fn whitespace() -> impl Fn(String) -> ParseResult {
    //            }
    return; //        }
} //        return Ok((word.clone(), input));
  //    }
  // }
  // fn any_whitespace<'a>() -> impl Fn(String) -> ParseResult<'a> {
  //     let sp = parse_char(' ');
  //     let tab = parse_char('\t');
  //     let newline = parse_char('\n');
  //     return any_of(vec![sp, tab, newline]);
  // }

// fn whitespace<'a>() -> impl Fn(String) -> ParseResult<'a> {
//     return zero_or_more(any_whitespace());
// }

// fn any_digits<'a>() -> impl (Fn(String) -> ParseResult<'a>) {
//     return any_of(vec![
//             parse_char('0'),
//             parse_char('1'),
//             parse_char('2'),
//             parse_char('3'),
//             parse_char('4'),
//             parse_char('5'),
//             parse_char('6'),
//             parse_char('7'),
//             parse_char('8'),
//             parse_char('9'),
//         ]);
// }

// fn digits<'a>() -> impl Fn(String) -> ParseResult<'a> {
//     return move |input: String| {
//         match one_or_more(any_digits())(input) {
//             Ok(()) => {},
//             Err(err) => return Err(err),

//         }
//     };
// }

// fn bool<'a>() -> impl Fn(String) -> ParseResult<'a> {
//     let _true = keyword("true".to_string());
//     let _false = keyword("false".to_string());
//     return any_of(vec![_true, _false]);
// }

// #[test]
// fn test_parse_single_digit() {
//     assert_eq!(any_digits()("1AB".to_string()), ParseResult::Ok((Expr::Int(1), "AB".to_string())));
// }

// #[test]
// fn test_parse_digits() {
//     assert_eq!(digits()("1234AB".to_string()), ParseResult::Ok((Expr::Int(1234), "AB".to_string())));
// }

// #[test]
// fn test_parse_keyword() {
//     assert_eq!(keyword("struct".to_string())("struct name".to_string()), ParseResult::Ok((Expr::Misc("struct".to_string()), " name".to_string())));
// }

// #[test]
// fn test_parse_bool() {
//     assert_eq!(bool()("truesomeshitaftertrue".to_string()), ParseResult::Ok((Expr::Bool(true), "someshitaftertrue".to_string())));
//     assert_eq!(bool()("falsesomeshitaftertrue".to_string()), ParseResult::Ok((Expr::Bool(false), "someshitaftertrue".to_string())));
// }
