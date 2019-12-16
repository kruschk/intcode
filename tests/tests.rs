use intcode;
use std::path::Path;

#[test]
fn day2_part1_tests() {
    let inputs = [
        "input/day2-part1-test0.txt",
        "input/day2-part1-test1.txt",
        "input/day2-part1-test2.txt",
        "input/day2-part1-test3.txt",
        "input/day2-part1-test4.txt",
    ];
    let outputs = [
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        vec![2, 0, 0, 0, 99],
        vec![2, 3, 0, 6, 99],
        vec![2, 4, 4, 5, 99, 9801],
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
    ];
        assert_eq!(inputs.len(), outputs.len());
        for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
            let mut src = intcode::parse_source(Path::new(input))
                .unwrap();
            intcode::execute_intcode_program(&mut src);
            assert_eq!(*output, src);
        }
}

#[test]
fn day2_part1_input() {
    let mut src = intcode::parse_source(Path::new(
        "input/day2-part1-input.txt")).unwrap();
    assert_eq!(10566835, intcode::execute_intcode_program(&mut src));
}

#[test]
fn day2_part2_input() {
    let src = intcode::parse_source(Path::new(
        "input/day2-part2-input.txt")).unwrap();
    if let Some((noun, verb)) = intcode::find_noun_and_verb(&src) {
        assert_eq!(2347, 100*noun + verb);
    } else {
        panic!();
    }
}

#[test]
fn day5_part1_tests() {
    let inputs = [
        "input/day5-part1-test0.txt",
        "input/day5-part1-test1.txt",
    ];
    let outputs = [
        vec![1002, 4, 3, 4, 99],
        vec![1101, 100, -1, 4, 99],
    ];
        assert_eq!(inputs.len(), outputs.len());
        for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
            let mut src = intcode::parse_source(Path::new(input))
                .unwrap();
            intcode::execute_intcode_program(&mut src);
            assert_eq!(*output, src);
        }
}
