use std::env;
use std::process;

fn check(num: i64) -> bool {
    if num == 2 || num ==3 {
        return true
    }
    if num <= 1 || num % 2 == 0 || num % 3 == 0 {
        return false
    }
    let mut i: i64 = 5;
    let stop: i64 = (f64::powf(num as f64, 0.5) as i64) + 1;
    
    while i <= stop 
    {
        if num % i ==0 || num % (i + 2) ==0 {
            return false;
        }
        i += 6;
    }
    return true
}

fn printhelp() {
    println!("A simple program to calculate the first n prime numbers, or to\n\
            check if number(s) is/are prime(s).\n\
            Usage:\n\
            find_prime <arg> num\n\
            \n\
            Options:\n\
            -n <num>  Calculates the first n prime number.\n\
            -c <num1 num2 num3 ...>  Checks if num is a prime number or not.\n\
            -h Prints this message.")
}

fn nprimes(_args: Vec<String>) {
    let mut count: i64 = 0;
    let mut nprime: i64 = 0;
    let mut _args: Vec<String> = env::args().collect();
    if _args.len() > 3 {
        println!("Too many numbers! Provide only one number. Exiting.");
        process::exit(0x0100);
    }
    for arg in _args.iter_mut() {
        if count > 1 {
            let _maxprime: i64 = match arg.parse::<i64>() {
                Ok(_maxprime) => _maxprime,
                Err(e) => {
                    println!("Error: {e}");
                    continue;
                }
            };
            println!("The first {arg} prime numbers:");
            loop { 
                let result: bool = check(count);
                    if result == true {
                        println!("{count}");
                        nprime += 1;
                    }
                count +=1;
                if nprime == _maxprime {
                    break;
                    }
                }
            }
            count += 1
        }
}
        

fn check_primes(_args: Vec<String>) {
    let mut _args: Vec<String> = env::args().collect();
    _args.remove(0);
    _args.remove(0);
    for arg in _args.iter_mut() {
        
        let _tnum: i64 = match arg.parse::<i64>() {
            Ok(_tnum) => _tnum,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };
        println!("Check if {arg} is a prime number:");
        
        let result: bool = check(_tnum);
            if result == true {
                println!("{_tnum} \x1b[1;32mIS\x1b[0m a prime!");
            }
            else {
                println!("{_tnum} is \x1b[1;31m\x1b[5mNOT\x1b[0m a prime!");
            }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let farg: &String = &args[1];
        match farg.as_str() {
            "-n" => nprimes(args),
            "-c" => check_primes(args),
            "-h" => printhelp(),
            _ => println!("Wrong argument was given. Try: -h for help."),
        }
    } else {
        println!("No argument was given. Try: -h for help.");
    }
}

