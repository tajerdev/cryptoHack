

/* Using the two primes p = 26513, q = 32321, find the integers u,v such that

p * u + q * v = gcd(p,q)

Enter whichever of u and v is the lower number as the flag. */


fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
       let temp = b;
      b = a % b;
      a = temp;
    }
    a
 }
 
 pub fn main() {
 
    let x = 66528;
    let y = 52920;
 
    let result = gcd(x, y);
 
    println!("{}", result);
 
 }