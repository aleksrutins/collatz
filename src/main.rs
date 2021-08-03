use rayon::prelude::*;
use std::env;
// const MAX: u128 = 1000000; // 374607431768211455 - 1; - Chop off about half the digits for now for fewer overflows

fn main() {
    let args: Vec<String> = env::args().collect();
    let max: u128 = u128::from_str_radix(&args[1], 10).unwrap();
    if args.len() >= 3 && &args[2] == "one" {
        calc(max);
    } else {
        (1..max).into_par_iter().for_each(calc);
    }
}

fn calc(i: u128) {
    println!("Number: {}", i);
    let mut x = i;
    let mut steps = 0;
    loop {
        println!("{} -> {}", i, x);
        if x % 2 == 0 { // Even
            x = x / 2
        } else {
            x = (x * 3) + 1;
        }
        steps += 1;
        if x == 1 { println!("Proven for {} in {} steps", i, steps); break;}
    }
}