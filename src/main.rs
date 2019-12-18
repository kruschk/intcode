use intcode::machine::Machine;
use std::{
    env,
    io,
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
    let mut machine = Machine::new_from_file(Path::new(&src_filename))
        .expect("Error creating a machine.");
    machine.execute(io::stdin(), io::stdout()).expect("IO error occurred.");
    machine.execute(io::stdin(), io::stdout()).expect("IO error occurred.");
}
