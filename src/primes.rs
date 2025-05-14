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