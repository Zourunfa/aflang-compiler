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

fn whitespace() -> impl Fn(String) -> ParseResult {
    return zero_or_more(any_whitespace());
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

/**
 * box是一个智能指针，用于在堆上分配内存，并持有其数据
 * 她允许你在编译时知道数据的大小，并且拥有所有权，而不需要
 * 手动管理内存的分配和释放
 * 
 * 
 * fn main() {
    // 在堆上分配一个整数，并将所有权移交给 box_pointer
    let box_pointer: Box<i32> = Box::new(42);

    // 使用 * 运算符来获取指针所指向的数据
    println!("Value: {}", *box_pointer);
    
    // 在这个代码块的末尾，box_pointer 将被释放，从而释放堆上的内存
}
 */
