fn main() {
    /* 
    let condition = true;

    let number = if condition { 5 } else { 5 };

    println!("{number}");


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break is like return (which can also be used) but it breaks the loop
        }
    };

    println!("{result}");

    

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;

    }

    println!("End count = {count}");

    */

    let a = [10,20,30,40,50];

    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() { //.rev() reverses the range
        println!("{number}");
    }
}
