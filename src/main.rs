fn main(){
    print!("hei");
}


/*

/*
    let mut threads = Vec::new();
        for i in (0..ant_threads).step_by(2) {
        threads.push(thread::spawn(move || {
            get_primes_between_interval(v[i].parse::<u64>,v[i+1]);
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
    */
}
    //round::ceil(i^2 * (max_range-min_range)/(ant_threads^2)
    //DividingValue[i=1..ant_threads-1] = min_range + Round(i^2 * (max_range-min_range)/(ant_threads^2))
/*
    let mut threads = Vec::new();

    for i in 0..10 {
        threads.push(thread::spawn(move || {
            println!("Output from thread {}", i);
        }));
    }
    for thread in threads {
        let _ = thread.join();
    }
    */

    print!("Enter the max range >> ");
    io::stdout().flush().expect("could not flush");
    io::stdin().read_line(&mut max_range).expect("Error reading from STDIN");

    print!("Enter ant threads >> ");
    io::stdout().flush().expect("could not flush");
    io::stdin().read_line(&mut ant_threads).expect("Error reading from STDIN");


    print!("max: {}, min: {}, threads: {}", min_range, max_range, ant_threads);

    let start: u64 = 40;
    let end: u64 = 70;

    let list = get_primes_between_interval(min_range.parse().unwrap(), end);
    println!("Collected (0..10) into: {:?}", list);
}

fn get_primes_between_interval(start: u64 ,end: u64) -> Vec<u64> {
    let mut vec: Vec<_> = (start..end).collect();
    for p in 2..end {
        vec.retain(|&x| x <= p || x % p != 0);
    }
    vec
}
    /*
    let mut prime=

    for(int prime = min_range; prime <=max_range; prime++){
        for(int i=2;i<=prime; i++){
            if(prime%i==0){
                break;
            }
        }
        if(prime == i){
            cout << prime << endl;
        }
    }
    /*
    let mut threads = Vec::new();

    for i in 0..10 {
        threads.push(thread::spawn(move || {
            println!("Output from thread {}", i);
        }));
    }
    for thread in threads {
        let _ = thread.join();
    }
    */
}

*/
*/