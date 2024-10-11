use detox::{detox_full, parse_args};

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    let parsed_args = parse_args(args);
    if let Err(code) = parsed_args {
        std::process::exit(code);
    }
    let (options, paths_to_check) = parsed_args.unwrap();
    let result_code = detox_full(&options, paths_to_check);
    std::process::exit(result_code);
}
