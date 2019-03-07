use std::sync::Mutex;
use std::thread;

lazy_static! {
    static ref PRIMES: Mutex<Vec<Vec<usize>>> = Mutex::new(Vec::new());
}

pub fn calculate_primes(min_range: usize, max_range: usize, ant_threads: usize) -> Vec<usize> {
    if min_range >= max_range || ant_threads < 1 || min_range < 2 {
        panic!("ERROR! Invalid variables!");
    }
    let sum: usize = max_range - min_range;
    let ant_work_pr_threads: usize = sum / ant_threads;
    let last_work_thread: usize = sum - (ant_work_pr_threads * (ant_threads - 1));
    let thread_work_range_size: usize = ant_threads * 2;
    let mut thread_work_range: Vec<usize> = Vec::with_capacity(thread_work_range_size);
    init_primes(ant_threads);
    thread_work_range.push(min_range);
    let mut temp_value: usize = min_range + last_work_thread;
    thread_work_range.push(temp_value);
    for _i in 1..ant_threads {
        thread_work_range.push(temp_value);
        temp_value += ant_work_pr_threads;
        thread_work_range.push(temp_value);
    }
    let mut threads = Vec::new();
    let mut index = 0;
    for i in (0..thread_work_range_size).step_by(2) {
        let start = thread_work_range[i].clone();
        let end = thread_work_range[i + 1].clone();
        threads.push(thread::spawn(move || {
            get_primes_between_interval(index, start, end)
        }));
        index += 1;
    }
    for thread in threads {
        let _ = thread.join();
    }
    let prime_list_threads = PRIMES.lock().unwrap();
    let mut prime_list: Vec<usize> = Vec::new();
    for x in prime_list_threads.iter() {
        for &y in x.iter() {
            prime_list.push(y);
        }
    }
    return prime_list;
}

fn init_primes(ant_threads: usize) -> () {
    for _i in 0..ant_threads {
        PRIMES.lock().unwrap().push(Vec::new());
    }
}

fn get_primes_between_interval(index: usize, start: usize, end: usize) -> () {
    let mut vec: Vec<usize> = (start..end).collect();
    for p in 2..end {
        vec.retain(|&x| x <= p || x % p != 0);
    }
    PRIMES.lock().unwrap()[index] = vec;
}
