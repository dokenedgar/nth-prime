pub fn nth(n: u32) -> u32 {
    let primes: Vec<u32> = get_primes(n);
    primes[n as usize]
}

fn get_primes(end: u32) -> Vec<u32> {
    let mut counter = 2;
    let mut current_number = 2;
    let mut primes: Vec<u32> = Vec::new();
    loop {
        if primes.len() <= end as usize {
            loop {
                if counter != current_number {
                    if current_number % counter == 0 {
                        break;
                    } else {
                        counter += 1;
                    }
                } else {
                    primes.push(current_number);
                    break;
                }
            }
            current_number += 1;
            counter = 2;
        } else {
            break;
        }
    }
    primes
}
