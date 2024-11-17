use std::io;
use rand::Rng;



fn sim_coin () -> Vec<u32>{

    let mut flips = String::new();
    let mut weight = String::new();

    io::stdin()
        .read_line(&mut flips)
        .expect("Failed to read line, try again later!");

    let n: usize = flips.trim().parse().expect("Please enter a VALID number!");
    
    println!("What would you like the probability of a head to be?");

    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line, try again later!");

    let p: f32 = weight.trim().parse().expect("Please enter a VALID number!");

    let mut a = vec![0; n];

    let mut index = 0;

    while index < n{

        let rand = rand::thread_rng().gen_range(0.0..1.0);

        if rand < p{
            
            a[index] = 1;
        }
        else{
            a[index] = 0;
        }

        index += 1;

    }

    a

}

fn count_heads(a: Vec<u32>) -> u32{

    let mut count = 0;
    let mut index = 0;
    let size = a.len();

    while index < size  {
        
        if a[index] > 0{

            count += 1;

        }

        index += 1

    }

    count

}




fn main() {

    println!("Welcome to the coin flip simulator! Please specify how many coins you would like to flip?");    

    let flip_result = sim_coin();
    let head_count = count_heads(flip_result.clone());

    let mut heads = String::new();

    println!("How many heads would you like to count?");

    io::stdin()
        .read_line(&mut heads)
        .expect("Failed to read line, try again later!");

    let k: u32 = heads.trim().parse().expect("Please enter a VALID number!");

    println!("There are {head_count} heads.");
    println!("and here is the coin flip result: {:?}", flip_result);

}
