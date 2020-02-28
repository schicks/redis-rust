use super::domain::{Command, Primitive};
use super::errors::{ApplicationError, Fallible};
use std::collections::LinkedList;

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
                .map(parse_primitive)??
                .into(),
        )),
        "get" => Ok(Command::Get(args.next().fail_to("No key provided")?.into())),
        "incr" => Ok(Command::Incr(
            args.next().fail_to("No key provided")?.into(),
        )),
        "sadd" => {
            let key = args.next().fail_to("No key provided")?;
            let values: Vec<Primitive> = args
                .map(parse_primitive)
                .fold(Ok(LinkedList::new()), |acc, item| match (acc, item) {
                    (Ok(mut acc), Ok(item)) => {
                        acc.push_front(item);
                        Ok(acc)
                    }
                    (Err(acc), _) => Err(acc),
                    (_, Err(item)) => Err(item),
                })?
                .into_iter()
                .collect();
            Ok(Command::Sadd(key.into(), values))
        }
        "scard" => Ok(Command::Scard(
            args.next().fail_to("No key provided")?.into(),
        )),
        "sismember" => Ok(Command::Sismember(
            args.next().fail_to("No key provided")?.into(),
            args.next()
                .fail_to("No value provided")
                .map(parse_primitive)??
                .into(),
        )),
        "sdiffstore" => Ok(Command::SdiffStore(
            args.next().fail_to("No destination provided")?.into(),
            args.next().fail_to("No base key provided")?.into(),
            args.map(String::from).collect(),
        )),
        "sinterstore" => Ok(Command::SinterStore(
            args.next().fail_to("No destination provided")?.into(),
            args.map(String::from).collect(),
        )),
        "sunion" => Ok(Command::Sunion(args.map(String::from).collect())),
        "sunionstore" => Ok(Command::SunionStore(
            args.next().fail_to("No destination provided")?.into(),
            args.map(String::from).collect(),
        )),
        unknown_command => Err(format!("No such command: {}", unknown_command).into()),
    }
}

fn parse_primitive(data: &str) -> Result<Primitive, ApplicationError> {
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
    use super::parse_primitive;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn quoted_strings_are_valid(s in "\".*\"") {
            assert_eq!(parse_primitive(&s)?, Primitive::String(s[1..s.len()-1].to_string()))
        }
    }

    proptest! {
        #[test]
        fn only_quoted_strings_are_valid(s in "[^\"0-9].*") {
            parse_primitive(&s).unwrap_err()
        }
    }

    proptest! {
        #[test]
        fn numbers_are_valid(n in -1000i64..1000) {
            parse_primitive(&format!("{}", n))?
        }
    }
}
