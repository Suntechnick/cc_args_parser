use std::collections::HashMap;

const INTEGER_CHAR_TYPE: char = '#';
const STRING_CHAR_TYPE: char = '*';

#[derive(Debug)]
enum Arg {
    Int(i32),
    Bool(bool),
    String(String),
}

impl Arg {
    fn set_value(&mut self, value: String) -> Result<(), String> {
        match self {
            Arg::Int(v) => match value.parse::<i32>() {
                Ok(parsed_value) => {
                    *v = parsed_value;
                }
                Err(_) => {
                    return Err(format!(
                        "Provided argument value '{}' is not of a type integer",
                        value
                    ));
                }
            },
            Arg::Bool(v) => {
                *v = true;
            }
            Arg::String(v) => {
                *v = value;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Args(HashMap<char, Arg>);

impl Args {
    pub fn build(schema_template: &str, args_list: Vec<String>) -> Result<Args, String> {
        let args = Args(HashMap::new());
        let args = parse_schema(args, schema_template)?;
        let args = parse_args(args, args_list)?;

        Ok(args)
    }
    pub fn get_bool(&self, c: char) -> Result<bool, String> {
        let entry = self.try_get_entry(c)?;

        if let Arg::Bool(value) = entry {
            Ok(*value)
        } else {
            Err(format!("Provided argument is not of a type boolean: {}", c))
        }
    }
    pub fn get_int(&self, c: char) -> Result<i32, String> {
        let entry = self.try_get_entry(c)?;

        if let Arg::Int(value) = entry {
            Ok(*value)
        } else {
            Err(format!("Provided argument is not of a type integer: {}", c))
        }
    }
    pub fn get_string(&self, c: char) -> Result<String, String> {
        let entry = self.try_get_entry(c)?;

        if let Arg::String(value) = entry {
            Ok(value.clone().to_string())
        } else {
            Err(format!("Provided argument is not of a type string: {}", c))
        }
    }
    fn try_get_entry(&self, c: char) -> Result<&Arg, String> {
        let entry = self.0.get(&c);

        match entry {
            Some(value) => Ok(value),
            None => Err(format!("No argument '{}' was provided in schema", c)),
        }
    }
}

fn parse_schema(mut args: Args, schema_template: &str) -> Result<Args, String> {
    let tokens = schema_template.split(",").collect::<Vec<&str>>();

    for token in tokens {
        let flag = token.chars().next().unwrap();
        if args.0.contains_key(&flag) {
            return Err(format!("Provided duplicate flag in schema: {}", flag));
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

            // argument didn't match any of supported types
            return Err(format!("Unsupported schema argument type: {}", token));
        } else {
            return Err(format!("Wrong schema argument format: {}", token));
        }
    }

    Ok(args)
}

fn parse_args(mut args: Args, args_list: Vec<String>) -> Result<Args, String> {
    for arg in args_list {
        let mut chars = arg.chars();

        // argument can't be an empty string so this unwrap is safe
        let first_char = chars.next().unwrap();
        if first_char != '-' {
            return Err(format!("Arguments should start with a '-': {}", arg));
        }

        let flag = chars.next();
        if !flag.is_some() {
            return Err(format!("Invalid argument format: {}", arg));
        }
        let flag = flag.unwrap();

        let flag_value: String = chars.collect();

        if let Some(arg) = args.0.get_mut(&flag) {
            arg.set_value(flag_value)?;
        } else {
            return Err(format!("Argument not found in schema: {}", arg));
        }
    }
    Ok(args)
}
