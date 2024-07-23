use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::io;

fn dec_to_bin(n: u32) -> String {
    format!("{:b}", n)
}

fn fastfib(n: u32) -> BigInt {
    let bin_of_n = dec_to_bin(n);
    let mut f: [BigInt; 2] = [BigInt::zero(), BigInt::one()];

    for b in bin_of_n.chars() {
        let f2i1: BigInt = &f[1] * &f[1] + &f[0] * &f[0];
        let f2i: BigInt = &f[0] * ((&f[1] << 1) - &f[0]);
        
        if b == '0' {
            f[0] = f2i;
            f[1] = f2i1;
        } else {
            f[0] = f2i1.clone();
            f[1] = f2i1 + f2i;
        }
    }

    f[0].clone()
}

fn explicit_fib (n_flt: f64) -> f64 {
    let phi: f64 = (1.0 + (5.0_f64).sqrt()) / 2.0;
    let psi:f64 =  (1.0 - (5.0_f64).sqrt()) / 2.0;
    return (1.0 / (5.0_f64).sqrt()) * ( phi.powf(n_flt) - psi.powf(n_flt));
}

fn gen_fibonacci (n: u32) -> u32 {
    if n <= 0 {
        return 0
    } else if n == 1 {
        return 1
} else { 
    return gen_fibonacci(n-1) + gen_fibonacci(n-2);
 }
} 

fn main() {
    println!("Type exit to quit program");

    loop {
        println!("Choose a method: (1) Naive recursion, (2) Binet's formula, or (3) Fast doubling");
        let mut method = String::new();
        io::stdin().read_line(&mut method).expect("Failed to read line");
        if method.trim() == "exit" {
            break;
        }

        let method: u32 = match method.trim().parse() {
            Ok(m) => m,
            Err(_) => continue,
        };

        println!("Type a positive integer");
        let mut n = String::new();    
        io::stdin() 
            .read_line(&mut n)
            .expect("Failed to read line"); 
        if n.trim() == "exit" {
            break;
        }

        let n: u32 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => continue,
    };
    
        match method {
            1 => { 
                if n >= 40{
                    println!("Please try an integer less than 40.")
                } else {
                let fib = gen_fibonacci(n);
                println!("The {} Fibonacci number (naive recursion) is {}.", n, fib);
                } 
            },
            2 => {
                if n >= 70 {
                    println!("Please try an integer less than 70.")
                } else {
                    let fib = explicit_fib(n as f64);
                    println!("The {} Fibonacci number (Binet's formula) is approx. {}.", n, fib);
                }
            },
            3 => {
                if n > 100_000 {
                    println!("Please try an integer less thann 100,000.")
                } else {
                let fib = fastfib(n);
                println!("The {} Fibonacci number (fast doubling) is {}.", n, fib);
                }
            },
            _ => println!("Invalid method choice"),
        }
   }
}
