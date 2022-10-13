use ferris_says::say;
use rand::Rng; // from the previous step
use std::io::{stdin, stdout, BufWriter};

fn say_things() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    // mut is required to make this mutable!
    let mut number: i64 = 999999999999999;

    println!("The number is {}", number);

    number = 23;

    println!("The number is {}", number);

    let another_number = 555u32;

    println!("The number is {}", another_number);

    let my_bool = false;

    println!("The bool is {}", my_bool);

    let my_char = 'a';

    println!("The char is {}", my_char);

    // Notice that ampersand
    let my_string = "stringy";

    // println is a macro ... what's that?
    println!("The string ref is {}", my_string);

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn main() {
    //say_things();

    let range = 0..10;
    let random_number = rand::thread_rng().gen_range(range);
    let my_name = "jimmy";

    //println!("The random number is {}, {}", random_number, my_name);

    println!("Enter a number, {}", my_name);

    loop {
        let mut guess = String::new();

        stdin().read_line(&mut guess);
        
        // unwrap is to fix the result type  to success 
        // you don't also need turbofish ::<u8>
        let guess_number: u8  = guess.trim().parse::<u8>().unwrap();

        // if guess_number > random_number {
        //     println!("{}, is too high!", guess_number);
        // } else if guess_number < random_number {
        //     println!("{}, is too low!", guess_number);
        // } else {
        //     println!("{} is correct!", guess_number);
        //     break;
        // }

        // cargo clippy says this is nicer
        // cargo build --release gives is releaseable build in target/release
        // this is a generic function - it wants to non-destructively take a reference to "other"
        // That is why it wants a reference (&)
        match guess_number.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("{}, is too low!", guess_number),
            std::cmp::Ordering::Equal => {
                println!("{}, is just right!", guess_number);
                break;
            }
            std::cmp::Ordering::Greater => println!("{}, is too high!", guess_number),

        }

        println!("{}", guess_number);
    }

}