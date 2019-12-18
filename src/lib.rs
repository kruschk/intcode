mod instruction;
mod opcode;

pub mod machine;

use machine::Machine;
use std::io;

// This is a convenience function for day 2; it iterates over nouns and verbs
// (values at memory locations 1 and 2, respectively), until it finds the pair
// that results in a machine output (value at memory location 0) of MAGIC_NUM.
pub fn find_noun_and_verb(src: &str) -> Option<(isize, isize)> {
    const MAGIC_NUM: isize = 19_690_720;
    for noun in 0..100 {
        for verb in 0..100 {
            // Execute the program with this noun and verb pair.
            let mut machine = Machine::new(src);
            machine.write_mem(1, noun);
            machine.write_mem(2, verb);
            if let Ok(MAGIC_NUM) = machine.execute(io::stdin(), io::stdout()) {
                // Return if we found the correct pair.
                return Some((noun, verb));
            }
        }
    }
    None
}
