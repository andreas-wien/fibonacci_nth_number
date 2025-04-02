use std::env;
use std::io;
use std::num::ParseIntError;

fn fib(n: u128, memo: &mut Vec<u128>) -> u128 {
    match memo.get(n as usize) {
        Some(n) => n.to_owned(),
        None => {
            let r = if n == 0 {
                0
            } else if n == 1 {
                1
            } else {
                fib(n - 1, memo) + fib(n - 2, memo)
            };
            memo.push(r);
            r
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);

    let number = loop {
        let number: Result<u128, ParseIntError>;
        if args.len() > 0 {
            number = args.next().unwrap().parse();
        } else {
            let mut s = String::new();
            println!("Enter what number of the Fibonacci sequence should be displayed: ");
            io::stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            number = s.trim().parse();
        }

        match number {
            Ok(n) => {
                if n == 0 {
                    println!("Enter a number greater than zero.");
                    continue;
                } else if n > 187 {
                    println!("Enter a number lower than 188.");
                    continue;
                } else {
                    break n;
                }
            }
            Err(e) => println!("{e}: Enter a valid integer greater than zero."),
        }
    };

    let mut memo: Vec<u128> = vec![0, 1];
    println!(
        "The {number}. Fibonnaci number is: {0}\nn = {1}\nfib({1}) = {0}",
        fib(number - 1, &mut memo),
        number - 1
    );
}
