#[macro_use]
extern crate lazy_static;
use std::io;
use std::io::Write;
use std::process;
use std::time::Instant;
mod o1;

fn main() {
    let min_range: usize = get_input_from_user(String::from("min range  "));
    let max_range: usize = get_input_from_user(String::from("max range  "));
    let ant_threads: usize = get_input_from_user(String::from("ant threads"));
    let now = Instant::now();
    println!(
        "The primes between {} and {}, are:\n{:?}\nTime used: {:?}",
        min_range,
        max_range,
        o1::calculate_primes(min_range, max_range, ant_threads),
        now.elapsed()
    );
}

fn get_input_from_user(name: String) -> usize {
    let mut value_input = String::new();
    print!("Enter the {} >> ", name);
    io::stdout().flush().expect("could not flush");
    io::stdin()
        .read_line(&mut value_input)
        .expect("Error reading from STDIN");
    let trimmed_value = value_input.trim();
    match trimmed_value.parse::<usize>() {
        Ok(i) => return i,
        Err(..) => {
            println!("Error {} is not a valid input", trimmed_value);
            process::exit(0x0100);
        }
    };
}

#[cfg(test)]
mod dcode_tests {
    #[test]
    fn test_1() {
        let expected_result: Vec<usize> = [2, 3, 5, 7].to_vec();
        assert_eq!(expected_result, super::o1::calculate_primes(2, 10, 2));
    }
    #[test]
    fn test_2() {
        let expected_result: Vec<usize> = [5, 7, 11, 13, 17, 19, 23, 29].to_vec();
        assert_eq!(expected_result, super::o1::calculate_primes(5, 30, 5));
    }

    #[test]
    fn test_3() {
        let expected_result: Vec<usize> = [
            11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        ]
        .to_vec();
        assert_eq!(expected_result, super::o1::calculate_primes(10, 100, 10));
    }

    #[test]
    #[should_panic]
    fn test_error_max_range() {
        super::o1::calculate_primes(4, 2, 2);
    }
}
