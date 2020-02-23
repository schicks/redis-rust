use super::domain::{Command, Primitive};
use super::errors::{ApplicationError, Fallible};

pub fn parse_cmd(cmd: String) -> Result<Command, ApplicationError> {
    let mut args = cmd.split_whitespace();
    match args.next().fail_to("No command given")? {
        "echo" => args
            .next()
            .fail_to("Nothing to echo".into())
            .map(|echoed| Command::Echo(echoed.into())),
        "set" => Ok(Command::Set(
            args.next().fail_to("No key provided")?.into(),
            args.next()
                .fail_to("No value provided")
                .map(parse_data)?
                .map(|val| val.into())?,
        )),
        "get" => Ok(Command::Get(args.next().fail_to("No key provided")?.into())),
        "incr" => Ok(Command::Incr(
            args.next().fail_to("No key provided")?.into(),
        )),
        "sadd" => Ok(Command::Sadd(
            args.next().fail_to("No key provided")?.into(),
            args.next()
                .fail_to("No value provided")
                .map(parse_data)?
                .map(|val| val.into())?,
        )),
        unknown_command => Err(format!("No such command: {}", unknown_command).into()),
    }
}

pub fn parse_data(data: &str) -> Result<Primitive, ApplicationError> {
    if let (Some('"'), Some('"')) = (data.chars().nth(0), data.chars().nth_back(0)) {
        Ok(Primitive::String(data[1..data.len() - 1].into()))
    } else if let Ok(n) = data.parse::<i64>() {
        Ok(Primitive::Number(n))
    } else {
        Err(ApplicationError::Error("Unknown Type".into()))
    }
}

#[cfg(test)]
mod test {
    use super::super::domain::Primitive;
    use super::parse_data;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn quoted_strings_are_valid(s in "\".*\"") {
            assert_eq!(parse_data(&s)?, Primitive::String(s[1..s.len()-1].to_string()))
        }
    }

    proptest! {
        #[test]
        fn only_quoted_strings_are_valid(s in "[^\"0-9].*") {
            parse_data(&s).unwrap_err()
        }
    }

    proptest! {
        #[test]
        fn numbers_are_valid(n in -1000i64..1000) {
            parse_data(&format!("{}", n))?
        }
    }
}
