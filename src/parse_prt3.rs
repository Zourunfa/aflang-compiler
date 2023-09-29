fn whitespace() -> impl Fn(String) -> ParseResult {
    let sp = parse_char(' ');
    let tab = parse_char('\t');
    let newline = parse_char('\n');

    return any_of(vec![sp, tab, newline]);
}

fn any_of(parsers: Vec<impl Fn(String) -> ParseResult>) -> impl Fn(String) -> ParseResult {
    return move |input: String| {
        for parser in parsers.iter() {
            match parser(input.clone()) {
                Ok((remaining, parsed)) => return Ok((remaining, parsed)),
                Err(err) => continue,
            }
        }
    };
}

fn decl(mut input: String) -> ParserResult {
    // 去掉变量前面的空格
    let (remains, _) = whitespace()(input.clone())?;
}

#[test]
fn test_parse_decl_bool() {
    let decl_res = decl("a = false".to_string());

    assert!(decl_res.is_ok());

    let noen: Box<Option<ParseObj>> = Box::new(none);

    if let (_, ParseObj::Decl(name, none, be)) = decl_res.unwrap() {
        assert!(name, "a");
        assert_eq(be, Box::new(ParseObj::Bool(false)));
    } else {
        assert!(false)
    }
}
