use std::io;

fn main() {

    loop {
        println!("\nPlease input your n");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: i64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The {n} term in the fibonacci sequence is {0}", calculate_nth_fib(n));
    }
}

fn calculate_nth_fib(n: i64) -> i64 {
    if n == 0 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    let mut currn: i64 = 2;

    while currn <= n {
        //println!("\nn:{n}\nlast_num:{last_number}\ncurr_number:{curr_number}\ncurr_n:{currn}");

        c = a + b;
        a = b;
        b = c;

        currn+=1;
    }

    c
}
