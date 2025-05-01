use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Добро пожаловать в  игру 'Guess the number!'");
    let secret_number = rand::rng().random_range(0..=100);
    
    loop {
        println!("Введите число: ");
        let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Ой, ой, похоже произошла ошибка...😭😭😭");
        
        let guess_number: i32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Ваше число: {guess_number}");
        
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Ожидамое число больше!"),
            Ordering::Greater => println!("Ожидамое число меньше!"),
            Ordering::Equal => {
                println!("Вы угадали число!");
                break;
            },
        }        
    }

    println!("\nНажмите Enter, чтобы выйти...");
    let mut exit = String::new();
    io::stdin().read_line(&mut exit).unwrap();
}