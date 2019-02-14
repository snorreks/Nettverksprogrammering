#[macro_use]
extern crate lazy_static;
use std::thread;
use std::io;
use std::io::Write;
use std::sync::Mutex;
use std::process;
use std::time::{Instant};

lazy_static! {
    static ref PRIMES:Mutex<Vec<Vec<usize>>> = Mutex::new(Vec::new());
}
fn main() {
    let min_range: usize = get_input_from_user(String::from("min range"));
    let max_range: usize = get_input_from_user(String::from("max range"));
    if min_range>max_range{
        println!("ERROR! min range must be higher than max range!");
        process::exit(0x0100);
    }
    let ant_threads: usize = get_input_from_user(String::from("ant threads"));
    if ant_threads> (max_range-min_range) || ant_threads < 1 {
        println!("ERROR! Invalid ant threads range!");
        process::exit(0x0100);
    }
    let now = Instant::now();
    println!("The primes between {} and {}, are:\n{:?}\nTime used: {:?}",min_range, max_range, calculate_primes(min_range, max_range, ant_threads),now.elapsed());
}

fn calculate_primes(min_range: usize, max_range: usize, ant_threads: usize)->Vec<usize>{
    if min_range>max_range{
        panic!("shieet!")
    }
    let sum: usize = max_range - min_range;
    let ant_work_pr_threads: usize = sum / ant_threads;
    let last_work_thread: usize = sum - (ant_work_pr_threads * (ant_threads - 1));
    let thread_work_range_size: usize = ant_threads *2;
    let mut thread_work_range: Vec<usize> = Vec::with_capacity(thread_work_range_size);
    init_primes(ant_threads);
    thread_work_range.push(min_range);
    let mut temp_value: usize = min_range + last_work_thread;
    thread_work_range.push(temp_value);
    println!("last_work_thread: {}, ant_work_pr_threads: {}, temp_value: {}",last_work_thread,ant_work_pr_threads, temp_value);
    for _i in 1..ant_threads {
        thread_work_range.push(temp_value);
        temp_value += ant_work_pr_threads;
        thread_work_range.push(temp_value);
    }
    let mut threads = Vec::new();
    let mut index = 0;
    for i in (0..thread_work_range_size).step_by(2) {
        let start = thread_work_range[i].clone();
        let end = thread_work_range[i+1].clone();
        println!("start: {}, end: {}, index: {}",start,end, index);
        threads.push(thread::spawn(move || {get_primes_between_interval(index, start, end)}));
        index += 1;
    }
    for thread in threads {
        let _ = thread.join();
    }
    let prime_list_threads =PRIMES.lock().unwrap();

    let mut prime_list: Vec<usize> = Vec::new();

    for x in prime_list_threads.iter() {
        for &y in x.iter() {
            prime_list.push(y);
        }
    }
    return prime_list;
}

fn get_input_from_user(name:String)->usize{
    let mut value_input = String::new();
    print!("Enter the {} >> ",name);
    io::stdout().flush().expect("could not flush");
    io::stdin().read_line(&mut value_input).expect("Error reading from STDIN");
    let trimmed_value = value_input.trim();
    match trimmed_value.parse::<usize>() {
        Ok(i) => return i,
        Err(..) => {
            println!("Error {} is not a valid input", trimmed_value);
            process::exit(0x0100);
            },
    };
}

fn init_primes(ant_threads: usize)->() {
    for _i in 0..ant_threads {
        PRIMES.lock().unwrap().push(Vec::new());
    }
}

fn get_primes_between_interval(index:usize,start: usize ,end: usize) -> () {
    let mut vec: Vec<usize> = (start..end).collect();
    for p in 2..end {
        vec.retain(|&x| x <= p || x % p != 0);
    }
    PRIMES.lock().unwrap()[index] = vec;
}

#[cfg(test)]
mod dcode_tests{
    #[test]
    fn test_1(){
        let expected_result: Vec<usize> = [2,3,5,7].to_vec();
        assert_eq!(expected_result, super::calculate_primes(2, 10, 2));
    }
    #[test]
    fn test_2(){
        let expected_result: Vec<usize> = [5, 7, 11, 13, 17, 19, 23, 29].to_vec();
        assert_eq!(expected_result, super::calculate_primes(5, 30, 5));
    }

    #[test]
    fn test_3(){
        let expected_result: Vec<usize> = [11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97].to_vec();
        assert_eq!(expected_result, super::calculate_primes(10, 100, 10));
    }

    #[test]
    #[should_panic]
    fn test_error_max_range(){
        super::calculate_primes(4, 2, 2);
    }
}