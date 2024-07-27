// Function to compute the greatest common divisor
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to compute the least common multiple
fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

// Function to compute the multiplicative order of b modulo m
fn multiplicative_order(b: u64, m: u64) -> u64 {
    let mut k = 1;
    let mut power = b % m;
    while power != 1 {
        power = (power * b) % m;
        k += 1;
    }
    k
}

// Function to compute the additive order of a in Z_n
fn additive_order(a: u64, n: u64) -> u64 {
    n / gcd(a, n)
}

// Function to compute the order of (a, b) in Z_n x Z_m^*
fn order(a: u64, n: u64, b: u64, m: u64) -> u64 {
    let add_order = additive_order(a, n);
    let mul_order = multiplicative_order(b, m);
    lcm(add_order, mul_order)
}

fn main() {
    let a = 6;
    let n = 8;
    let b = 7;
    let m = 18;

    let result = order(a, n, b, m);
    println!("The order of ({}, {}) in Z_{} x Z_{}^* is {}", a, b, n, m, result);
}








