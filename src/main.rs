use intcode;
use std::{
    env,
    path::Path,
    process,
};

fn main() {
    let mut arguments = env::args();
    let src_filename = match arguments.nth(1) {
        Some(s) => s,
        None => {
            println!("usage: intcode SOURCE_FILENAME");
            process::exit(1);
        }
    };
    let mut src = intcode::parse_source(Path::new(
        &src_filename)).unwrap();
    intcode::execute_intcode_program(&mut src);
}
