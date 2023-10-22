use std::env;

mod args;

use args::Args;

fn main() {
    let schema_template = "l,p#,d*";
    let args_list = env::args().skip(1).collect::<Vec<String>>();

    let args = Args::build(schema_template, args_list);

    if let Ok(args) = args {
        println!("Value for argument 'l' is: {}", args.get_bool('l').unwrap());
        println!("Value for argument 'p' is: {}", args.get_int('p').unwrap());
        println!(
            "Value for argument 'd' is: {}",
            args.get_string('d').unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_schema_and_arguments() -> Result<(), String> {
        let args = Args::build(
            "l,p#,d*",
            vec![
                "-l".to_string(),
                "-p5050".to_string(),
                "-d/usr/bin".to_string(),
            ],
        )?;

        assert_eq!(args.get_int('p')?, 5050);
        assert_eq!(args.get_bool('l')?, true);
        assert_eq!(args.get_string('d')?, "/usr/bin".to_string());

        Ok(())
    }

    #[test]
    fn handle_invalid_schema() {
        let args = Args::build("KEKW", vec![]);

        assert!(args.is_err());

        let err = args.unwrap_err();

        assert_eq!("Wrong schema argument format: KEKW", err);
    }

    #[test]
    fn handles_unsupported_schema_argument() {
        let args = Args::build("e&", vec!["-e1234".to_string()]);

        assert!(args.is_err());

        let err = args.unwrap_err();

        assert_eq!("Unsupported schema argument type: e&", err);
    }

    #[test]
    fn handles_double_argument_in_schema() {
        let args = Args::build("p#,p*", vec!["-p8080".to_string()]);

        assert!(args.is_err());

        let err = args.unwrap_err();

        assert_eq!("Provided duplicate flag in schema: p", err);
    }

    #[test]
    fn handles_invalid_argument_format() {
        let args = Args::build("l", vec!["WUT".to_string()]);

        assert!(args.is_err());

        let err = args.unwrap_err();

        assert_eq!("Arguments should start with a '-': WUT".to_string(), err);
    }

    #[test]
    fn handles_argument_not_provided_in_schema() {
        let args = Args::build("p#", vec!["-s8080".to_string()]);

        assert!(args.is_err());

        let err = args.unwrap_err();

        assert_eq!("Argument not found in schema: -s8080".to_string(), err);
    }

    #[test]
    fn handles_wrong_argument_parsing() {
        let args = Args::build("p#", vec!["-pnot_int".to_string()]);

        assert!(args.is_err());

        let err = args.unwrap_err();

        assert_eq!(
            "Provided argument value 'not_int' is not of a type integer",
            err
        );
    }

    #[test]
    fn handles_wrong_argument_type() -> Result<(), String> {
        let args = Args::build("p*", vec!["-pnot_int".to_string()])?;

        let result = args.get_int('p');

        assert!(result.is_err());

        let err = result.unwrap_err();

        assert_eq!("Provided argument is not of a type integer: p", err);

        Ok(())
    }

    #[test]
    fn handled_absent_argument() -> Result<(), String> {
        let args = Args::build("p#", vec!["-p4000".to_string()])?;

        let result = args.get_bool('l');

        assert!(result.is_err());

        let err = result.unwrap_err();

        assert_eq!("No argument 'l' was provided in schema", err);
        Ok(())
    }
}
