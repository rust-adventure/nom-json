use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take, take_until, take_while, take_while_m_n},
    character::{
        complete::{
            alpha0, alpha1, alphanumeric1, char, digit1, hex_digit1, multispace0, one_of, satisfy,
        },
        is_hex_digit,
    },
    combinator::{not, opt, peek, verify},
    error::{Error, ErrorKind, ParseError},
    multi::{many0, many1, separated_list0, separated_list1},
    number::complete::float,
    sequence::{delimited, pair, tuple},
    AsChar, IResult, InputTakeAtPosition, Parser,
};

pub fn json<'a>(input: &'a str) -> IResult<&'a str, Value<'a>> {
    println!(
        "\n\nPARSING
{}
vvv",
        &input
    );
    let (input, parsed_json) = value(input)?;
    Ok((input, parsed_json))
}

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Object { members: Vec<Member<'a>> },
    Array(Vec<Value<'a>>),
    String(&'a str),
    Number(f32),
    True,
    False,
    Null,
}
fn value(input: &str) -> IResult<&str, Value> {
    println!("> as value");
    alt((
        object,
        array,
        string,
        number,
        tag("true").map(|_| Value::True),
        tag("false").map(|_| Value::False),
        tag("null").map(|_| Value::Null),
    ))(input)
}

fn object(input: &str) -> IResult<&str, Value> {
    println!(">> as object");
    let (input, parsed_members) = delimited(tag("{"), members, tag("}"))(input)?;
    Ok((
        input,
        Value::Object {
            members: parsed_members,
        },
    ))
}
// // '{' ws '}'
// // '{' members '}'

fn members<'a>(input: &'a str) -> IResult<&'a str, Vec<Member<'a>>> {
    let (input, members) = separated_list0(char(','), member)(input)?;
    Ok((input, members))
}
// member
// member ',' members

#[derive(Debug, PartialEq)]
pub struct Member<'a> {
    pub key: &'a str,
    pub value: Value<'a>,
}
// TODO: can't leave it like this with the error
fn member<'a>(input: &'a str) -> IResult<&str, Member<'a>> {
    let (input, _) = ws(input)?;
    let (input, key) = string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, value) = element(input)?;
    if let Value::String(key) = key {
        Ok((input, Member { key: key, value }))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            "fa",
            ErrorKind::NoneOf,
        )))
    }
}

// }
// // ws string ws ':' element

fn array(input: &str) -> IResult<&str, Value> {
    println!(">> as array {}", &input);
    let (input, _) = char('[')(input)?;
    let (input, values) = opt(elements)(input)?;
    let (input, _) = char(']')(input)?;
    Ok((input, Value::Array(values.or(Some(vec![])).unwrap())))
}

fn elements<'a>(input: &'a str) -> IResult<&str, Vec<Value>> {
    let (input, elements) = separated_list0(char(','), element)(input)?;
    Ok((input, elements))
}
// // element
// // element ',' elements

fn element(input: &str) -> IResult<&str, Value> {
    let (input, _) = ws(input)?;
    let (input, val) = value(input)?;
    let (input, _) = ws(input)?;
    Ok((input, val))
}
// // ws value ws

fn string(input: &str) -> IResult<&str, Value> {
    println!(">> as string {}", &input);
    let (input, _) = char('"')(input)?;
    println!(">>> input1: {}", &input);
    let (input, s) = opt(characters)(input)?;
    println!(">>> input2: {} {:?}", &input, s);
    let (input, _) = char('"')(input)?;
    println!(">>> input3: {}", &input);
    Ok((
        input,
        Value::String(match s {
            Some(v) => v,
            None => "",
        }),
    ))
}

fn chars(i: &str) -> IResult<&str, char> {
    satisfy(|c| !c.is_ascii_control() && c != '"' && c != '\\')(i)
}

fn characters(input: &str) -> IResult<&str, &str> {
    verify(
        escaped(chars, '\\', one_of(r#""\/bfnrtu"#)),
        |maybe_hex: &str| {
            dbg!("maybe_hex_check");
            match validate_all_escaped_hex(maybe_hex) {
                Err(_) => {
                    dbg!("falsefalsefalse");
                    false
                }
                Ok((input, output)) => {
                    dbg!("truetruetrue", &output);
                    output.iter().all(|s| escaped_hex(s).is_ok())
                }
            }
        },
    )(input)
}
fn escaped_hex(arg_input: &str) -> IResult<&str, &str> {
    let (input, _u) = tag("\\u")(arg_input)?;
    let (_input, _hex) = verify(hex_digit1, |hex: &str| hex.len() == 4)(input)?;
    take(4 + _u.len())(arg_input)
}

fn validate_all_escaped_hex(arg_input: &str) -> IResult<&str, &str> {
    dbg!("any_escaped_hex");
    let res: IResult<&str, &str> = take_until("\\u")(arg_input);
    dbg!("any_escaped_hex_2");
    match res {
        Err(_) => {
            dbg!("any_escaped_hex err");
            let (_input, _u) = tag("\\u")(arg_input)?;
            take(4 + _u.len())(arg_input)
        }
        Ok((pls_input, _)) => {
            dbg!("any_escaped_hex ok");
            let (_input, _u) = tag("\\u")(pls_input)?;
            dbg!("any_escaped_hex ok 2");
            take(4 + _u.len())(pls_input)
        }
    }
}

fn number(input: &str) -> IResult<&str, Value> {
    let (input, (i, f, e)) = tuple((integer, opt(fraction), opt(exponent)))(input)?;
    dbg!(i, f, e);
    let num_str = format!(
        "{}{}{}{}{}",
        i,
        if f.is_some() { "." } else { "" },
        f.unwrap_or(""),
        if e.is_some() { "E" } else { "" },
        e.unwrap_or("")
    );
    // let (input, num) = float(input)?;
    println!("parsing {}", &num_str);
    let num = num_str.parse::<f32>().expect("expected a valid float");
    Ok((input, Value::Number(num)))
}

fn integer(arg_input: &str) -> IResult<&str, &str> {
    let (input, neg) = opt(char('-'))(arg_input)?;
    let (_input, nums) = verify(digit1, |s: &str| !(s.starts_with("0") && s.len() != 1))(input)?;
    let (input, output) = take(
        nums.len()
            + match neg {
                Some(_) => 1,
                None => 0,
            },
    )(arg_input)?;
    Ok((input, output))
}

fn fraction(input: &str) -> IResult<&str, &str> {
    let (input, _) = char('.')(input)?;
    let (input, digits) = digit1(input)?;
    Ok((input, digits))
}

fn exponent(input: &str) -> IResult<&str, &str> {
    // check for e
    let (post_e_input, _) = alt((char('E'), char('e')))(input)?;

    // check for num and sign
    let (input, sig) = sign(post_e_input)?;

    let (_input, digits) = digit1(input)?;

    // take the num if num and sign pass
    let (input, num) = take(
        digits.len()
            + match sig {
                Some(_) => 1,
                None => 0,
            },
    )(post_e_input)?;

    Ok((input, num))
}

fn sign(input: &str) -> IResult<&str, Option<char>> {
    opt(alt((char('+'), char('-'))))(input)
}

fn ws(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hex() -> Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let (input, value) = escaped_hex("\\u0000")?;
        assert_eq!(input, "");
        assert_eq!(value, "\\u0000");
        Ok(())
    }
}
