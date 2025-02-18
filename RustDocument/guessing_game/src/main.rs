use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

    // let mut v : Vec<i32> = Vec::new();
    let mut v = vec![1,2,3 ];
    v.push(4);
    v.push(6);

    println!("v:{:?}",v);
    println!("Let's Start Number Matching Game");

    let mut num = String::new();

    let answer_num = rand::thread_rng().gen_range(0..100);

    loop{

        num.clear();

        io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    
    println!("You guessed: {}", num);
    
    let num :u32= match num.trim().parse(){
        
        Ok(num) => num,
        Err(_)  => continue,
    
    };
    
    
    match num.cmp(&answer_num){
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => {println!("Nice You do it !"); 
        break;
        }
    }


    
};
println!("Anser * {}", answer_num);
}