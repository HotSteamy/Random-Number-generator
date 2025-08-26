use rand::Rng;
use std::io;
fn main() {
    
let (mut real_min, mut real_max): (i32 ,i32);

    loop {
        println!("Please input min number for your random number");
        let mut min = String::new();
        io::stdin().read_line(&mut min).expect("Invalid input");
        real_min = match min.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Invalid min number, try again.");  
                continue; 
            }
        };

        println!("Please input max number for your random number");
        let mut max = String::new();
        io::stdin().read_line(&mut max).expect("Invalid input");
        real_max = match max.trim().parse() {
            Ok(num) => num,  
            Err(_) => {
                println!("Invalid max number, try again.");  
                continue;
            }
        };
        

    
        if real_max < real_min {
                println!("invalid");
                continue;
            }
        
        else if real_max == real_min {
            println!("");
            println!("I dont see a point but anyway");
            break;
        }
        
        else {
            break;
        }
    }

    
    
    let random_number = rand::thread_rng().gen_range(real_min..=real_max);
    println!("");
    println!("");
    println!("");
    println!("Your random number is {random_number}");
    println!("");

    if random_number == real_min {
        println!("hey, your minimum value is equal to the randomly generated number :)")
    }

    else if random_number == real_max {
        println!("hey, your maximum value is equal to the randomly generated number :)")
    }
}
