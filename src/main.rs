use std::env;
use std::fs::{ read_to_string, write };

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        panic!("[ERROR] too few arguments");
    } else if args.len() > 2 {
        panic!("[ERROR too many arguments");
    }

    let contents = read_to_string(&args[1])
        .expect(format!("[ERROR] cannot read from {}", args[1]).as_str());

    let parsed_args = bcasm::parse_asm(contents)
        .expect("[ERROR] Parsing failed");

    let machine_code = bcasm::produce_machine_code(parsed_args)
        .expect("[ERROR] Machine code generation failed");

    let output: Vec<u8> = machine_code.iter()
        .map(|v| vec![((v & 0xff00) >> 8) as u8, (v & 0x00ff) as u8])
        .flatten()
        .collect();

    let output_file_name = format!("{}o", args[1].trim_end_matches("asm"));

    write(output_file_name, output);
}
