use std::{io,str::FromStr,fmt::Debug};
use rand::RngExt;


fn input<T: FromStr>()
    -> T
where 
    T::Err : Debug
    {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read input");
        buffer.trim().parse::<T>().expect("Couldn't parse input")
    }

fn main(){
    let mut rng = rand::rng();
    let number = rng.random_range(1..=100);
    let mut guess : i32;
    let mut count = 0;
    loop {
        count += 1;
        println!("Enter your guess between 1-100: ");
        guess = input();

        if guess > number{
            println!("The number is lower than {}",guess);
        }else if guess < number {
            println!("The number is greater than {}",guess);
        }else{
            println!("You guessed correctly in {} tries!!",count);
            break;
        }
    }
}
