// #![allow(dead_code)]

// /*TODO
//     - for
//         - c syntax
//         - foreach
//         - while syntax
//     - interface
//     - operator expressions
// */
// #[derive(Clone, Debug, PartialEq)]
// pub enum ParseObj {
//     Char(char),
//     Uint(usize),
//     Int(isize),
//     Float(f64),
//     Str(String),
//     Keyword(String),
//     Ident(String),
//     Bool(bool),
//     List(Vec<ParseObj>),
//     Decl(String, Box<Option<ParseObj>>, Box<ParseObj>),
//     FnCall(String, Vec<ParseObj>),
//     Struct(Vec<(ParseObj, ParseObj)>),
//     Fn(Vec<(ParseObj, ParseObj)>, Box<ParseObj>, Box<ParseObj>),
//     Array(Box<Option<ParseObj>>, Box<ParseObj>),
//     Stmt(Box<ParseObj>),
//     Block(Vec<ParseObj>),
//     If(Box<ParseObj>, Box<ParseObj>),
//     ForC(Box<ParseObj>, Box<ParseObj>, Box<ParseObj>, Box<ParseObj>),
//     Empty,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum ParseErr {
//     // unexpected (expected, found, location)
//     Unexpected(String, String, u64),
//     Unknown(String),
// }

// impl std::fmt::Display for ParseErr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Unexpected(_, _, _) => f.write_fmt(format_args!("{:?}", self)),
//             Self::Unknown(msg) => f.write_fmt(format_args!("{}", msg)),
//             _ => unreachable!(),
//         }
//     }
// }

// impl std::error::Error for ParseErr {}

// type ParseResult = Result<(String, ParseObj), ParseErr>;

// fn whitespace() -> impl Fn(String) -> ParseResult {
//     return zero_or_more(any_whitespace());
// }

// #[test]
// fn test_parse_decl_bool() {
//     let decl_res = decl("a = false".to_string());
//     assert!(decl_res.is_ok());
//     let none: Box<Option<ParseObj>> = Box::new(None);
//     if let (_, ParseObj::Decl(name, none, be)) = decl_res.unwrap() {
//         assert_eq!(name, "a");
//         assert_eq!(be, Box::new(ParseObj::Bool(false)));
//     } else {
//         assert!(false);
//     }
// }
