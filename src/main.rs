use rayon::prelude::*;
use std::env::args;
use std::time::Instant;
use zeronet_cryptography::create;

fn main() {
    let domain: String = args().nth(1).unwrap();
    let chk_inv_str: Option<String> = args().nth(2);
    let mut is_satisified = false;
    let mut check_inverse = false;
    if let Some(cmd) = chk_inv_str {
        if cmd == "--try-inverse" {
            check_inverse = true;
        }
    }

    let time = Instant::now();
    let mut logged = false;
    //Need to Support Multi-threading
    let handle = std::thread::spawn(move || {
        while !is_satisified {
            if time.elapsed().as_secs() != 0 && time.elapsed().as_secs() % 10 == 0 {
                if !logged {
                    println!("{} secs elapsed from start", time.elapsed().as_secs());
                }
                logged = true;
            } else if time.elapsed().as_secs() % 5 == 0 {
                logged = false;
            }
            let (private, public) = create();
            let validate = || {
                let result = PATTERNS
                    .par_iter()
                    .map(|pattern| {
                        if pattern_validator(&(public.to_lowercase()), pattern, &domain, time) {
                            println!("elapsed time in secs {}", time.elapsed().as_secs());
                            println!("pattern : {}", pattern);
                            println!("public key : {}", public);
                            println!("private key : {}", private);
                            return true;
                        }
                        false
                    })
                    .reduce(
                        || false,
                        |pre, cur| {
                            if pre || cur {
                                true
                            } else {
                                false
                            }
                        },
                    );
                result
            };
            if check_passed(&(public.to_lowercase()), &domain) {
                is_satisified = validate();
            }
            if check_inverse {
                if check_passed(
                    &(public.chars().rev().collect::<String>().to_lowercase()),
                    &domain,
                ) {
                    is_satisified = validate();
                    if is_satisified {
                        println!("Inverse Pattern is satisfied");
                    }
                }
            }
        }
    });
    handle.join().unwrap();
}

fn check_passed(public: &str, domain: &str) -> bool {
    let mut d_arr: Vec<char> = vec![];
    domain.chars().for_each(|a| d_arr.push(a));

    let initial_check: Vec<char> = public
        .chars()
        .map(|c| {
            if d_arr.contains(&c) {
                return c;
            } else {
                return '*';
            }
        })
        .filter(|c| *c != '*')
        .collect();
    for a in domain.chars() {
        if !initial_check.contains(&a) {
            return false;
        }
    }
    if initial_check.len() < domain.len() {
        return false;
    }
    let mut cur_char = 0;
    for i in 0..initial_check.len() {
        if let Some(n) = domain.chars().nth(cur_char) {
            if n == initial_check[i] {
                cur_char += 1;
            }
        }
    }
    if cur_char < domain.len() {
        return false;
    }

    // println!("Address : {} , {:?} ", public, initial_check);
    true
}
//TODO: Before checking patterns, add more checkings.
//GPU Comoputation will improve performance for checking patterns.
fn pattern_validator(public: &str, pattern: &str, domain: &str, time: Instant) -> bool {
    let length = domain.len();
    let mut result = String::from("");
    match pattern {
        "E" => {
            for i in 1..public.len() {
                if i % 2 == 0 && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "O" => {
            for i in 1..public.len() {
                if i % 2 != 0 && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "P" => {
            if domain.len() > PRIMES.len() {
                return false;
            }
            for i in 2..public.len() {
                if PRIMES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "C" => {
            for i in 1..public.len() {
                if !PRIMES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "W" => {
            for i in 0..public.len() {
                if result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "N" => {
            for i in 1..public.len() {
                if result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "F" => {
            if domain.len() > FIBNACCI.len() {
                return false;
            }
            for i in 1..public.len() {
                if FIBNACCI.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "NF" => {
            for i in 1..public.len() {
                if !FIBNACCI.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "V" => {
            if domain.len() > VOWELS.len() {
                return false;
            }
            for i in 1..public.len() {
                if VOWELS.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "CO" => {
            for i in 1..public.len() {
                if !VOWELS.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "SQ" => {
            if domain.len() > SQUARES.len() {
                return false;
            }
            for i in 1..public.len() {
                if SQUARES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "NS" => {
            for i in 1..public.len() {
                if !SQUARES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "CU" => {
            if domain.len() > CUBES.len() {
                return false;
            }
            for i in 1..public.len() {
                if CUBES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "NC" => {
            for i in 1..public.len() {
                if !CUBES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "RE" => {
            if domain.len() > RECTANGULAR.len() {
                return false;
            }
            for i in 1..public.len() {
                if RECTANGULAR.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "NR" => {
            for i in 1..public.len() {
                if !RECTANGULAR.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "TR" => {
            if domain.len() > TRIANGULAR.len() {
                return false;
            }
            for i in 1..public.len() {
                if TRIANGULAR.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "NT" => {
            for i in 1..public.len() {
                if !TRIANGULAR.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "ZP" => {
            for i in 1..public.len() {
                if INTEGER_PRIMES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "!P" => {
            for i in 1..public.len() {
                if !INTEGER_PRIMES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "ZF" => {
            for i in 1..public.len() {
                if INTEGER_FIBNACCI.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "!F" => {
            for i in 1..public.len() {
                if !INTEGER_FIBNACCI.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "ZV" => {
            for i in 1..public.len() {
                if INTEGER_VOWELS.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "!V" => {
            for i in 1..public.len() {
                if !INTEGER_VOWELS.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "ZS" => {
            for i in 1..public.len() {
                if INTEGER_SQUARES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "!S" => {
            for i in 1..public.len() {
                if !INTEGER_SQUARES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "ZC" => {
            for i in 1..public.len() {
                if INTEGER_CUBES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "!C" => {
            for i in 1..public.len() {
                if !INTEGER_CUBES.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "ZR" => {
            for i in 1..public.len() {
                if INTEGER_RECTANGULAR.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "!R" => {
            for i in 1..public.len() {
                if !INTEGER_RECTANGULAR.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "ZT" => {
            for i in 1..public.len() {
                if INTEGER_TRAIANGULAR.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        "!T" => {
            for i in 1..public.len() {
                if !INTEGER_TRAIANGULAR.contains(&i) && result.len() < length {
                    let c = public.chars().nth(i).unwrap();
                    let d = &domain[0..result.len()];
                    if &result == d {
                        result.push(c);
                    } else {
                        return false;
                    }
                }
            }
        }
        _ => {
            if pattern.starts_with("A") {
                let mut fib: Vec<usize> = vec![1];
                let seq: usize = pattern[1..].parse().unwrap();
                let mut prev: usize = 1;
                for _ in 0..domain.len() {
                    prev = prev + seq;
                    fib.push(prev);
                }
                for i in 1..public.len() {
                    if fib.contains(&i) && result.len() < length {
                        let c = public.chars().nth(i).unwrap();
                        let d = &domain[0..result.len()];
                        if &result == d {
                            result.push(c);
                        } else {
                            return false;
                        }
                    }
                }
                if (result.len()) < domain.len() {
                    return false;
                }
            } else if pattern.starts_with("-") {
                let mut fib: Vec<usize> = vec![1];
                let seq: usize = pattern[1..].parse().unwrap();
                let mut prev: usize = 1;
                for _ in 0..domain.len() {
                    prev = prev + seq;
                    fib.push(prev);
                }
                for i in 1..public.len() {
                    if !fib.contains(&i) && result.len() < length {
                        let c = public.chars().nth(i).unwrap();
                        let d = &domain[0..result.len()];
                        if &result == d {
                            result.push(c);
                        } else {
                            return false;
                        }
                    }
                }
                if (result.len()) < domain.len() {
                    return false;
                }
            } else if pattern.starts_with("G") {
                let mut fib: Vec<usize> = vec![];
                let seq: usize = pattern[1..].parse().unwrap();
                let mut prev: usize = 1;
                for _ in 0..domain.len() {
                    prev = prev * seq;
                    fib.push(prev);
                }
                for i in 1..public.len() {
                    if fib.contains(&i) && result.len() < length {
                        let c = public.chars().nth(i).unwrap();
                        let d = &domain[0..result.len()];
                        if &result == d {
                            result.push(c);
                        } else {
                            return false;
                        }
                    }
                }
                if (result.len()) < domain.len() {
                    return false;
                }
            } else if pattern.starts_with("/") {
                let mut fib: Vec<usize> = vec![];
                let seq: usize = pattern[1..].parse().unwrap();
                let mut prev: usize = 1;
                for _ in 0..domain.len() {
                    prev = prev * seq;
                    fib.push(prev);
                }
                for i in 1..public.len() {
                    if !fib.contains(&i) && result.len() < length {
                        let c = public.chars().nth(i).unwrap();
                        let d = &domain[0..result.len()];
                        if &result == d {
                            result.push(c);
                        } else {
                            return false;
                        }
                    }
                }
                if (result.len()) < domain.len() {
                    return false;
                }
            }
        }
    }
    println!(
        "result : {} took : {} secs, pattern: {}",
        result.to_lowercase(),
        time.elapsed().as_secs(),
        pattern
    );
    if result.to_lowercase() == domain {
        return true;
    }
    false
}

//Add MORE patterns to the list
const PATTERNS: [&str; 56] = [
    "E", "O", "P", "C", "W", "N", "F", "NF", "V", "CO", "SQ", "NS", "CU", "NC", "RE", "NR", "TR",
    "NT", "A3", "-3", "A4", "-4", "A5", "-5", "A6", "-6", "A7", "-7", "A8", "-8", "A9", "-9",
    "A10", "-10", "/3", "/4", "/5", "/6", "/7", "/8", "/9", "/10", "ZP", "!P", "ZF", "!F", "ZV",
    "!V", "ZS", "!S", "ZC", "!C", "ZR", "!R", "ZT", "!T",
];

const PRIMES: [usize; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
const FIBNACCI: [usize; 7] = [1, 2, 3, 5, 8, 13, 21];
const VOWELS: [usize; 5] = [1, 5, 9, 15, 21];
const SQUARES: [usize; 5] = [1, 4, 9, 16, 25];
const CUBES: [usize; 3] = [1, 8, 27];
const RECTANGULAR: [usize; 15] = [2, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32];
const TRIANGULAR: [usize; 7] = [1, 3, 6, 10, 15, 21, 28];

// const INTEGERS: [isize; 34] = [
//     0_-16, 1_-15, 2_-14, 3_-13, 4_-12, 5_-11, 6_-10, 7_-9, 8_-8, 9_-7, 10_-6, 11_-5, 12_-4,
//     13_-3, 14_-2, 15_-1, 16_0, 17_1, 18_2, 19_3, 20_4, 21_5, 22_6, 23_7,
//     24_8, 25_9, 26_10, 27_11, 28_12, 29_13, 30_14, 31_15, 32_16, 33_17,
// ];
const INTEGER_PRIMES: [usize; 13] = [3, 5, 9, 11, 13, 14, 18, 19, 21, 23, 27, 29, 33];
const INTEGER_FIBNACCI: [usize; 12] = [3, 8, 11, 13, 14, 15, 17, 18, 19, 21, 24, 29];
const INTEGER_VOWELS: [usize; 8] = [1, 7, 11, 15, 17, 21, 25, 31];
const INTEGER_SQUARES: [usize; 7] = [7, 12, 15, 17, 20, 25, 32];
const INTEGER_CUBES: [usize; 4] = [8, 15, 17, 24];
const INTEGER_RECTANGULAR: [usize; 13] = [2, 4, 6, 8, 10, 14, 18, 22, 24, 26, 28, 30, 32];
const INTEGER_TRAIANGULAR: [usize; 10] = [1, 6, 10, 13, 15, 17, 19, 22, 26, 31];
