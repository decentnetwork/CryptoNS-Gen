use std::time::Instant;

use crate::patterns;

pub fn check_passed(public: &str, domain: &str) -> bool {
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
pub fn pattern_validator(public: &str, pattern: &str, domain: &str, time: Instant) -> bool {
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
            if domain.len() > patterns::PRIMES.len() {
                return false;
            }
            for i in 2..public.len() {
                if patterns::PRIMES.contains(&i) && result.len() < length {
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
                if !patterns::PRIMES.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::FIBNACCI.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::FIBNACCI.contains(&i) && result.len() < length {
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
                if !patterns::FIBNACCI.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::VOWELS.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::VOWELS.contains(&i) && result.len() < length {
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
                if !patterns::VOWELS.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::SQUARES.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::SQUARES.contains(&i) && result.len() < length {
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
                if !patterns::SQUARES.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::CUBES.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::CUBES.contains(&i) && result.len() < length {
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
                if !patterns::CUBES.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::RECTANGULAR.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::RECTANGULAR.contains(&i) && result.len() < length {
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
                if !patterns::RECTANGULAR.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::TRIANGULAR.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::TRIANGULAR.contains(&i) && result.len() < length {
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
                if !patterns::TRIANGULAR.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::INTEGER_PRIMES.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::INTEGER_PRIMES.contains(&i) && result.len() < length {
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
                if !patterns::INTEGER_PRIMES.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::INTEGER_FIBNACCI.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::INTEGER_FIBNACCI.contains(&i) && result.len() < length {
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
                if !patterns::INTEGER_FIBNACCI.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::INTEGER_VOWELS.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::INTEGER_VOWELS.contains(&i) && result.len() < length {
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
                if !patterns::INTEGER_VOWELS.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::INTEGER_SQUARES.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::INTEGER_SQUARES.contains(&i) && result.len() < length {
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
                if !patterns::INTEGER_SQUARES.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::INTEGER_CUBES.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::INTEGER_CUBES.contains(&i) && result.len() < length {
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
                if !patterns::INTEGER_CUBES.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::INTEGER_RECTANGULAR.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::INTEGER_RECTANGULAR.contains(&i) && result.len() < length {
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
                if !patterns::INTEGER_RECTANGULAR.contains(&i) && result.len() < length {
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
            if domain.len() > patterns::INTEGER_TRAIANGULAR.len() {
                return false;
            }
            for i in 1..public.len() {
                if patterns::INTEGER_TRAIANGULAR.contains(&i) && result.len() < length {
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
                if !patterns::INTEGER_TRAIANGULAR.contains(&i) && result.len() < length {
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
