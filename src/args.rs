use std::collections::HashMap;

const INTEGER_CHAR_TYPE: char = '#';
const STRING_CHAR_TYPE: char = '*';

#[derive(Debug)]
enum Arg {
    Int(i32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Default)]
pub struct Args(HashMap<char, Arg>);

impl Args {
    pub fn build(schema_template: &str, args_list: Vec<String>) -> Result<Args, String> {
        let args = parse_schema(schema_template)?;
        let args = parse_args(args, args_list)?;

        Ok(args)
    }
    pub fn get_bool(&self, c: &char) -> Result<bool, String> {
        let entry = self.try_get_entry(c)?;

        if let Arg::Bool(value) = entry {
            Ok(*value)
        } else {
            Err(format!("Provided flag is not of a type bool: {}", c))
        }
    }
    pub fn get_int(&self, c: &char) -> Result<i32, String> {
        let entry = self.try_get_entry(c)?;

        if let Arg::Int(value) = entry {
            Ok(*value)
        } else {
            Err(format!("Provided flag is not of a type int: {}", c))
        }
    }
    pub fn get_string(&self, c: &char) -> Result<String, String> {
        let entry = self.try_get_entry(c)?;

        if let Arg::String(value) = entry {
            Ok(value.clone().to_string())
        } else {
            Err(format!("Provided flag is not of a type bool: {}", c))
        }
    }
    fn try_get_entry(&self, c: &char) -> Result<&Arg, String> {
        let entry = self.0.get(c);

        match entry {
            Some(value) => Ok(value),
            None => Err(format!("No flag -{} was provided", c)),
        }
    }
}

fn parse_args(mut args: Args, args_list: Vec<String>) -> Result<Args, String> {
    for arg in args_list {
        let flag = arg.chars().nth(1).unwrap();

        if let Some(arg_type) = args.0.get(&flag) {
            let flag_value: String = arg.chars().skip(2).collect();

            match arg_type {
                Arg::Int(_) => {
                    let parsed_value = flag_value.parse::<i32>().unwrap();
                    args.0
                        .entry(flag)
                        .and_modify(|v| *v = Arg::Int(parsed_value));
                }
                Arg::Bool(_) => {
                    args.0.entry(flag).and_modify(|v| *v = Arg::Bool(true));
                }
                Arg::String(_) => {
                    args.0
                        .entry(flag)
                        .and_modify(|v| *v = Arg::String(flag_value));
                }
            }
        }
    }
    Ok(args)
}

fn parse_schema(schema_template: &str) -> Result<Args, String> {
    let mut args = Args::default();
    let tokens = schema_template.split(",").collect::<Vec<&str>>();

    for token in tokens {
        let flag = token.chars().next().unwrap();
        if args.0.contains_key(&flag) {
            return Err(format!("Duplicate flag {}", flag));
        }

        // bool arg
        if token.len() == 1 {
            args.0.insert(flag, Arg::Bool(false));
            continue;
        }

        if token.len() == 2 {
            let mut chars = token.chars();
            let arg_char = chars.next().unwrap();
            let arg_type = chars.next().unwrap();

            if arg_type == INTEGER_CHAR_TYPE {
                args.0.insert(arg_char, Arg::Int(0));
                continue;
            }

            if arg_type == STRING_CHAR_TYPE {
                args.0.insert(arg_char, Arg::String("".to_string()));
                continue;
            }
        } else {
            return Err(format!("Wrong flag format: {}", token));
        }
    }

    Ok(args)
}
