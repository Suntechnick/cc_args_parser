use std::env;

mod args;

use args::Args;

fn main() {
    let schema_template = "l,p#,d*";
    let args_list = env::args().skip(1).collect::<Vec<String>>();

    let args = Args::build(schema_template, args_list).unwrap();

    println!("{}", args.get_bool(&'l').unwrap());
    println!("{}", args.get_int(&'p').unwrap());
    println!("{}", args.get_string(&'d').unwrap());

    match args.get_bool(&'x') {
        Ok(_) => (),
        Err(message) => println!("{message}"),
    }
}
