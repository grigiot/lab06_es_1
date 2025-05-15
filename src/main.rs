use primes::find_primes;

mod primes;
fn main() {
    let v = find_primes(100,  4);
    println!("PRIME NUMBERS LIMIT 67 THREADS 5: {:?}", v);
}
