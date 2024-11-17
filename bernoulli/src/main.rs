use std::io;
use rand::Rng;



fn sim_coin (n: usize, p: f32) -> Vec<u32>{

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

fn fact(num: u32) -> u32 {

    if num == 0{
        1
    }
    else{
        num*fact(num-1)
    }

}

//fn float_power(float: f32, power: u32) -> f32 {

    //let mut result = float;
    //let mut power_index = power;

    //if power_index == 0 {
        //1.0;
    //}
    //else{
        //result = result*float;
        //float_power(result, power_index);
    //}

    //result

//}

fn bern_dist(prob: f32, flips: u32, heads: u32) -> f32{

    let float_heads: f32 = heads as f32;
    let float_flips: f32 = flips as f32;

    let theo_heads_number: u32 = fact(flips)/(fact(flips - heads)*fact(heads));
    let ftheo_heads_number: f32 = theo_heads_number as f32;



    let exp_prob = ftheo_heads_number*(prob.powf(float_heads))*((1.0-prob).powf(float_flips-float_heads));

    exp_prob

}




fn main() {

    println!("Welcome to the coin flip simulator! Please specify how many coins you would like to flip?"); 

    let mut flips = String::new();
    let mut weight = String::new();
    let mut heads = String::new();

    io::stdin()
        .read_line(&mut flips)
        .expect("Failed to read line, try again later!");

    let n: usize = flips.trim().parse().expect("Please enter a VALID number!");
    
    println!("What would you like the probability of a head to be?");

    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line, try again later!");

    let p: f32 = weight.trim().parse().expect("Please enter a VALID number!");

    println!("How many heads would you like to count?");

    io::stdin()
        .read_line(&mut heads)
        .expect("Failed to read line, try again later!");

    let k: u32 = heads.trim().parse().expect("Please enter a VALID number!");

    println!("Finally, how many simulations would you like to run?");

    let mut sim_in = String::new();

    io::stdin()
        .read_line(&mut sim_in)
        .expect("Error, try again later.");

    let sim_num: usize = sim_in.trim().parse().expect("Please enter a valid number!");

    let mut head_array: Vec<u32> = vec![0; sim_num];
    let mut sim_index: usize = 0;

    while sim_index < sim_num{

        let flip_result = sim_coin(n,p);
        let head_count = count_heads(flip_result.clone());

        head_array[sim_index] = head_count;
        sim_index += 1;

    }

    let mut head_count = 0;
    let mut count_index = 0;

    while count_index<sim_num{

        if head_array[count_index] == k {

            head_count += 1;

        }

        count_index += 1;

    }

    let u32n = n as u32;
    let fhead_count = head_count as f32;
    let fsim_num = sim_num as f32;

    let theo_prob = bern_dist(p,u32n,k);
    let obs_prob = fhead_count/(fsim_num);

    println!("The theoretical probability of flipping {k} heads in {n} flips with a head appearing with probability {p} is: {theo_prob}");

    println!("The observed proabability is {obs_prob}");

}
