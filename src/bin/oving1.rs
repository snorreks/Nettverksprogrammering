#[macro_use]
extern crate lazy_static;

use std::thread;
use std::io;
use std::io::Write;
use std::sync::Mutex;
use std::collections::HashMap;

fn main() {
    let min_range: usize = get_input_from_user(String::from("min range"));
    let max_range: usize = get_input_from_user(String::from("max range"));
    let ant_threads: usize = get_input_from_user(String::from("ant threads"));
    println!("The min range is: {}, The max range is: {}, The ant threads is {}", min_range, max_range, ant_threads);
    let sum = max_range - min_range;
    let ant_work_pr_threads = sum / ant_threads;
    let last_work_thread = sum - (ant_work_pr_threads * (ant_threads - 1));

    let mut thread_work_range: Vec<usize> = Vec::new();
    lazy_static! {
        static ref PRIMES:<Vec<Vec<usize>>> = {
            
        for _i in 0..ant_threads {
            PRIMES.lock().unwrap().push(Vec::new());
        }
            Mutex::new(Vec::with_capacity(ant_threads));


        static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
        // Since it's mutable and shared, use mutext.
    }

    init_primes(ant_threads);

    thread_work_range.push(min_range);
    thread_work_range.push(last_work_thread);
    let mut temp_value: usize = min_range + (last_work_thread);
    for _i in 1..ant_threads {
        thread_work_range.push(temp_value);
        temp_value += ant_work_pr_threads;
        thread_work_range.push(temp_value);
    }

    let mut threads = Vec::new();

    for i in (0..ant_threads).step_by(2) {
        let index = i.clone();
        let start = thread_work_range[i].clone();
        let end = thread_work_range[i+1].clone();
        threads.push(thread::spawn(move || {get_primes_between_interval(index, start, end)
        }));
    }
    for thread in threads {
        let _ = thread.join();
    }
    let te =PRIMES.lock().unwrap().len();
    for x in 0..te.size() {
    println!("{:?}", x);
}
}
fn get_input_from_user(name:String)->usize{
    let mut value_input = String::new();
    let mut value: usize = 0;
    print!("Enter the {} >> ",name);
    io::stdout().flush().expect("could not flush");
    io::stdin().read_line(&mut value_input).expect("Error reading from STDIN");
    let trimmed_value = value_input.trim();
    match trimmed_value.parse::<usize>() {
        Ok(i) => value = i,
        Err(..) => println!("Error {} is not a valid input", trimmed_value),
    };
    value
}

/*
fn init_primes(ant_threads: usize)->() {
        for _i in 0..ant_threads {
            PRIMES.lock().unwrap().push(Vec::new());
        }
}

*/

fn get_primes_between_interval(index:usize,start: usize ,end: usize) -> () {
    let mut vec: Vec<usize> = (start..end).collect();
    for p in 2..end {
        vec.retain(|&x| x <= p || x % p != 0);
    }
    PRIMES.lock().unwrap()[index] = vec;
}