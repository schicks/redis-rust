mod domain;
mod errors;
mod incr;
mod parse;
mod sadd;
use domain::{Command, Data};
use errors::{ApplicationError, Fallible, Flattenable};
use parse::parse_cmd;
use std::collections::HashMap;
use std::io;

fn execute(
    store: &mut HashMap<String, Data>,
    command: Command,
) -> Result<String, ApplicationError> {
    match command {
        Command::Echo(echoed) => Ok(format!("{}", echoed)),
        Command::Set(key, val) => {
            let printed = format!("{}", val);
            store.insert(key, val);
            Ok(printed)
        }
        Command::Get(key) => Ok(format!(
            "{}",
            store
                .get(&key)
                .fail_to(&format!("No value at key {}", key))?
        )),
        Command::Incr(key) => incr::command(store, &key).map(|v| format!("{}", v)),
        Command::Sadd(key, value) => sadd::command(store, key, value).map(|v| format!("{}", v)),
    }
}

fn main() {
    let mut root_namespace: HashMap<String, Data> = HashMap::new();
    loop {
        let mut input = String::new();
        match io::stdin()
            .read_line(&mut input)
            .map(|_| parse_cmd(input))
            .flatten()
            .map(|command| execute(&mut root_namespace, command))
            .flatten()
        {
            Ok(output) => println!("{}", output),
            Err(error) => println!("error: {}", error),
        }
    }
}
