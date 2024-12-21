use crate::types::{Literal, Op};
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res, recognize},
    sequence::preceded,
    IResult,
};

pub fn parse_integer(input: &str) -> IResult<&str, i64> {
    map_res(
        recognize(preceded(nom::combinator::opt(char('-')), digit1)),
        str::parse,
    )(input)
}

pub fn parse_float(input: &str) -> IResult<&str, f64> {
    map_res(
        recognize(preceded(
            nom::combinator::opt(char('-')),
            nom::sequence::tuple((digit1, char('.'), digit1)),
        )),
        str::parse,
    )(input)
}

fn parse_escaped_char(input: &str) -> IResult<&str, char> {
    preceded(char('🤫'), nom::character::complete::anychar)(input)
}

fn parse_unescaped_char(input: &str) -> IResult<&str, char> {
    nom::character::complete::none_of("🤫🫷")(input)
}

fn parse_text_fragment(input: &str) -> IResult<&str, char> {
    alt((parse_escaped_char, parse_unescaped_char))(input)
}

fn parse_text(input: &str) -> IResult<&str, String> {
    let build_string = nom::multi::fold_many0(parse_text_fragment, String::new, |mut str, c| {
        str.push(c);
        str
    });

    nom::sequence::delimited(char('🫸'), build_string, char('🫷'))(input)
}

fn parse_literal(input: &str) -> IResult<&str, Literal> {
    alt((
        map(parse_float, Literal::Float),
        map(parse_integer, Literal::Int),
        map(parse_text, Literal::Text),
    ))(input)
}

//TODO: allow multi-character identifiers delimited by spaces
fn parse_identifier(input: &str) -> IResult<&str, String> {
    // Match a single bigass unicode character - several blocks from the supplementary multilingual plane including most? emojis
    map(
        nom::character::complete::satisfy(|c| {
            let code = c as u32;
            (code >= 0x1F300 && code <= 0x1FFFF) || (code >= 0x25A0 && code <= 0x2BFF)
        }),
        |c| c.to_string(),
    )(input)
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    // First consume any whitespace
    let (input, _) = nom::character::complete::multispace0(input)?;

    // Then parse a single Op
    alt((
        map(parse_literal, Op::Literal),
        map(parse_identifier, Op::Identifier),
    ))(input)
}

pub fn parse_ops(input: &str) -> IResult<&str, Vec<Op>> {
    let (input, ops) = nom::multi::many0(parse_op)(input)?;
    let (input, _) = nom::character::complete::multispace0(input)?;
    if !input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    }
    Ok(("", ops))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_integer_parser() {
        assert_eq!(parse_integer("123"), Ok(("", 123)));
        assert_eq!(parse_integer("-456"), Ok(("", -456)));
        assert_eq!(parse_integer("789abc"), Ok(("abc", 789)));
        assert!(parse_integer("abc").is_err());
    }

    #[test]
    fn test_float_parser() {
        assert_eq!(parse_float("123.456"), Ok(("", 123.456)));
        assert_eq!(parse_float("-789.012"), Ok(("", -789.012)));
        assert_eq!(parse_float("3.14159abc"), Ok(("abc", 3.14159)));
        assert!(parse_float("123").is_err());
        assert!(parse_float(".456").is_err()); // TODO  make this not error
        assert!(parse_float("abc").is_err());
    }

    #[test]
    fn test_parse_escaped_char_parser() {
        assert_eq!(parse_escaped_char("🤫a"), Ok(("", 'a')));
        assert_eq!(parse_escaped_char("🤫🙂"), Ok(("", '🙂')));
        assert!(parse_escaped_char("🤫").is_err());
        assert!(parse_escaped_char("a").is_err());
        assert_eq!(parse_escaped_char("🤫aBC"), Ok(("BC", 'a')));
    }

    #[test]
    fn test_parse_unescaped_char_parser() {
        assert_eq!(parse_unescaped_char("a"), Ok(("", 'a')));
        assert_eq!(parse_unescaped_char("abc"), Ok(("bc", 'a')));
        assert!(parse_unescaped_char("🤫").is_err());
        assert!(parse_unescaped_char("🫷").is_err());
    }

    #[test]
    fn test_text_fragment_parser() {
        assert_eq!(parse_text_fragment("a"), Ok(("", 'a')));
        assert_eq!(parse_text_fragment("🙂"), Ok(("", '🙂')));
        assert_eq!(parse_text_fragment("🤫a"), Ok(("", 'a')));
        assert_eq!(parse_text_fragment("🤫🙂"), Ok(("", '🙂')));
        assert_eq!(parse_text_fragment("abc"), Ok(("bc", 'a')));
        assert!(parse_text_fragment("").is_err());
        assert!(parse_text_fragment("🤫").is_err());
        assert!(parse_text_fragment("🫷").is_err());
    }

    #[test]
    fn test_text_parser() {
        assert_eq!(parse_text("🫸hello 🌎🫷"), Ok(("", "hello 🌎".to_string())));
        assert_eq!(
            parse_text("🫸hello🫷world"),
            Ok(("world", "hello".to_string()))
        );
        assert_eq!(parse_text("🫸🙂🫷"), Ok(("", "🙂".to_string())));
        assert!(parse_text("hello").is_err());
        assert!(parse_text("🫸hello").is_err());
        assert!(parse_text("hello🫷").is_err());
    }

    #[test]
    fn test_literal_parser() {
        assert_eq!(parse_literal("123"), Ok(("", Literal::Int(123))));
        assert_eq!(parse_literal("-456"), Ok(("", Literal::Int(-456))));
        assert_eq!(parse_literal("123.456"), Ok(("", Literal::Float(123.456))));
        assert_eq!(
            parse_literal("-789.012"),
            Ok(("", Literal::Float(-789.012)))
        );
        assert_eq!(
            parse_literal("🫸hello🫷"),
            Ok(("", Literal::Text("hello".to_string())))
        );
        assert_eq!(
            parse_literal("🫸hello 🤫n world🫷"),
            Ok(("", Literal::Text("hello n world".to_string())))
        );
        assert_eq!(parse_literal("123abc"), Ok(("abc", Literal::Int(123))));
        assert!(parse_literal("abc").is_err());
    }

    #[test]
    fn test_parse_op() {
        assert_eq!(parse_op("123"), Ok(("", Op::Literal(Literal::Int(123)))));
        assert_eq!(
            parse_op("123.45"),
            Ok(("", Op::Literal(Literal::Float(123.45))))
        );
        assert_eq!(
            parse_op("🫸hello🫷"),
            Ok(("", Op::Literal(Literal::Text("hello".to_string()))))
        );
        assert_eq!(parse_op("🎈"), Ok(("", Op::Identifier("🎈".to_string()))));
        assert!(parse_op("abc").is_err());
    }

    #[test]
    fn test_parse_ops() {
        assert_eq!(parse_ops(""), Ok(("", vec![])));
        assert_eq!(
            parse_ops("123"),
            Ok(("", vec![Op::Literal(Literal::Int(123))]))
        );
        assert_eq!(
            parse_ops("123 456.789"),
            Ok((
                "",
                vec![
                    Op::Literal(Literal::Int(123)),
                    Op::Literal(Literal::Float(456.789))
                ]
            ))
        );
        assert_eq!(
            parse_ops("123 🎉 456.789"),
            Ok((
                "",
                vec![
                    Op::Literal(Literal::Int(123)),
                    Op::Identifier("🎉".to_string()),
                    Op::Literal(Literal::Float(456.789))
                ]
            ))
        );
        assert_eq!(
            parse_ops("🫸hello🫷 🎉"),
            Ok((
                "",
                vec![
                    Op::Literal(Literal::Text("hello".to_string())),
                    Op::Identifier("🎉".to_string())
                ]
            ))
        );
        assert!(parse_ops("123 abc").is_err());
        assert_eq!(
            parse_ops("123 "),
            Ok(("", vec![Op::Literal(Literal::Int(123))]))
        );
        assert_eq!(
            parse_ops("✖5➕-1 2🎉3"),
            Ok((
                "",
                vec![
                    Op::Identifier("✖".to_string()),
                    Op::Literal(Literal::Int(5)),
                    Op::Identifier("➕".to_string()),
                    Op::Literal(Literal::Int(-1)),
                    Op::Literal(Literal::Int(2)),
                    Op::Identifier("🎉".to_string()),
                    Op::Literal(Literal::Int(3)),
                ],
            )),
        );
    }
}
