use std::time::{self, Instant};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep, JoinHandle};

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn find_primes(limit: u64, n_threads: u64) -> Vec<u64> {
    
    let mut jh_vec: Vec<JoinHandle<Vec<u64>>> = vec![];

    let counter: Arc<Mutex<u64>> = Arc::new(Mutex::new(2));
    for _i in 0..=n_threads {
        let cnt: Arc<Mutex<u64>> = counter.clone();

        jh_vec.push(std::thread::spawn(move ||{
            let start = time::Instant::now();
            let mut res_vec: Vec<u64> = vec![];
            // let mut first_loop = true;
            println!("THREAD {:?} started at {:?}", thread::current().id(), start);
            loop{
                let mut p: std::sync::MutexGuard<'_, u64> = cnt.lock().unwrap();
                
                let n: u64 = *p;
                *p += 1;
                drop(p);
                // if first_loop{
                //     sleep(time::Duration::from_secs(7));
                //     first_loop = false;
                // }

                if n>limit{
                    break;
                }
                if is_prime(n) {
                    res_vec.push(n);
                }    
            }
            println!("THREAD {:?} -> res_vec: {:?} -> ended after: {:?} ", std::thread::current().id(), res_vec, Instant::now()-start);
            res_vec
        }));
    }
    
    let result = jh_vec.into_iter().map(|jh| {
        jh.join().unwrap()
    }).fold(Vec::new(), |mut acc: Vec<u64>, mut e|{
        acc.append(&mut e);
        return acc;
    });

    result
}

#[cfg(test)]
mod test{
    use crate::primes::is_prime;


    #[test]
    fn test_is_prime(){
        assert!(is_prime(2));
        assert!(!is_prime(78))
    }

}