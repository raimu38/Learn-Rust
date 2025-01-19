use rand::seq::SliceRandom;
use rand::thread_rng;

fn main(){
    let mut numbers = vec![0,1,2,3,4,5,6,7,8,9];
    println!("before: {:?}",numbers);
    let mut rng = thread_rng();
    numbers.shuffle(&mut rng);
    println!("after: {:?}", numbers);
}