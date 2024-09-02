use std::io;

fn farenheight_to_celcius(temp: i32) -> i32 {
    ((temp as f32 -32.0) * (5.0 / 9.0)) as i32
}

fn celcius_to_farenheight(temp:i32) -> i32 {
    ( ( temp as f32 * ( 9.0 / 5.0 ) ) + 32.0 ) as i32
}

fn main() {
    loop {
        println!("\nInput the unit you'd like to convert to (F/C)");

        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        unit = unit.trim().to_lowercase();

        if unit == "f" {
            println!("\nPlease input a celcius temperature");

            let mut temperature = String::new();
    
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");
    
            let temperature = match temperature.trim().parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("Error! {e}");
                    continue;
                },
            };
    
            println!("{temperature} degrees in farenheight is {0} degrees!", celcius_to_farenheight(temperature));
        }else if unit == "c" {
            println!("\nPlease input a farenheight temperature");

            let mut temperature = String::new();
    
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");
    
            let temperature = match temperature.trim().parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("Error! {e}");
                    continue;
                },
            };
    
            println!("{temperature} degrees in celcius is {0} degrees!", farenheight_to_celcius(temperature));
        } else {
            println!("Invalid input!");
        }
    }
}
