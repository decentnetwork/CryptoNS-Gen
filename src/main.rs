use rayon::prelude::*;
use secp256k1::{Secp256k1, Signing};
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod patterns;
mod address;
mod validators;

fn main() {
    let matches = args().get_matches();
    let domain: String = matches.value_of("domain").unwrap().to_string();
    let is_satisified = Arc::new(Mutex::new(false));
    let keys_generated = Arc::new(Mutex::new(0u64));
    let logged = Arc::new(Mutex::new(false));
    let mut check_inverse = false;
    let mut threads = 4;

    if matches.is_present("inverse") {
        check_inverse = true;
    }
    if matches.is_present("threads") {
        threads = matches.value_of("threads").unwrap().parse::<u64>().unwrap();
    }

    let time = Instant::now();
    let mut handles = vec![];
    let secp = Secp256k1::new();
    for _ in 0..threads {
        let is_satisified = is_satisified.clone();
        let logged = logged.clone();
        let keys_generated = keys_generated.clone();
        let domain = domain.clone();
        let secp = secp.clone();
        let handle = std::thread::spawn(move || {
            while !*is_satisified.lock().unwrap() {
                if time.elapsed().as_secs() != 0 && time.elapsed().as_secs() % 10 == 0 {
                    let mut logged = logged.lock().unwrap();
                    
                    if !*logged {
                        println!("{} secs elapsed from start, keys generated : {} ", time.elapsed().as_secs(), *keys_generated.lock().unwrap());
                    }
                    *logged = true;
                } else if time.elapsed().as_secs() % 5 == 0 {
                    let mut logged = logged.lock().unwrap();
                    *logged = false;
                }
                let (private, public) = create(&secp);
                let mut kg = keys_generated.lock().unwrap();
                *kg = *kg + 1;
                // keys_generated.store(keys_generated.load(Ordering::Acquire) + 1, Ordering::Release);
                let validate = || {
                    let result = patterns::PATTERNS
                        .par_iter()
                        .map(|pattern| {
                            if validators::pattern_validator(&(public.to_lowercase()), pattern, &domain, time) {
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
                if validators::check_passed(&(public.to_lowercase()), &domain) {
                    let mut is_satisified = is_satisified.lock().unwrap();
                    *is_satisified = validate();
                }
                if check_inverse {
                    if validators::check_passed(
                        &(public.chars().rev().collect::<String>().to_lowercase()),
                        &domain,
                    ) {
                        let mut is_satisified = is_satisified.lock().unwrap();
                        *is_satisified = validate();
                        if *is_satisified {
                            println!("Inverse Pattern is satisfied");
                        }
                    }
                }
            }
        }); 
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

fn create(secp: &Secp256k1<impl Signing>) -> (String, String) {
    let  btc_add = address::BitcoinAddress::new(secp, true, false);
    (btc_add.private_key.to_string(), btc_add.address.to_string())
}

pub fn args<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("domain")
                .index(1)
                .takes_value(true)
                .required(true)
                .help("Preferred Domain Name to Search"),
        )
        .arg(
            clap::Arg::with_name("threads")
                .short("t")
                .long("threads")
                .takes_value(true)
                .default_value("4")
                .help("Number of threads to be used"),
        )
        .arg(
            clap::Arg::with_name("inverse")
                .short("i")
                .long("inverse")
                .takes_value(true)
                .default_value("false")
                .help("Number of threads to be used"),
        )
        .arg(
            clap::Arg::with_name("case-sensitive")
                .short("c")
                .long("case-sensitive")
                .takes_value(false)
                .help("Use case sensitive comparison to match addresses"),
        )
}