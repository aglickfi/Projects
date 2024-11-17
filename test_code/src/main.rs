use std::io;
use rand::Rng;

fn main (){

    let mut flips = String::new();
    //let mut heads = String::new();
    //let mut weight = String::new();

    io::stdin()
        .read_line(&mut flips)
        .expect("Failed to read line, try again later!");

    let n: usize = flips.trim().parse().expect("Please enter a VALID number!");
    
    //println!("How many heads would you like to count?");

    //io::std()
        //.read_line(&mut heads)
        //.expect("Failed to read line, try again later!");

    //let k: u32 = heads.trim().parse().expect("Please enter a VALID number!");

    //println!("What would you like the probability of a head to be?");

    //io::std()
        //.read_line(&mut weight)
        //.expect("Failed to read line, try again later!");

    //let p: f32 = weight.trim().parse().expect("Please enter a VALID number!");

    let mut a = vec![0; n];

    //let index = 0;

    //while index < n{

        //let rand = rand::thread_rng().gen_range(1..=1000);

        //if rand < p*1000.0{
            
            //a[index] = 1;
        //}
        //else{
            //a[index] = 0;
        //}

        //index += 1;

    //}

    println!("{:?}", a);

}