mod fivonacci;
// mod ondo;
mod structuser;
fn main(){
    // ondo::ondo();
    let n = 1;
    let result: usize = fivonacci::fivonacci(n);
    println!("{}番目のフィボナッチ数は:{}",n, result);
    nijuukaihou();
    let mut s1 = String::from("Hello world");
    let length = calculate_length(&mut s1);
    {

        let r1 = &s1;
        println!("r1 :{}", r1);
    }
    {let r2 = &s1;
        println!("r2: {}", r2);
    
    }
    {

        let r3 = &mut s1;
        println!("r3: {}", r3);
    }

    

    println!("The length of'{}' is {}", s1, length);

    let reference = dangle();
    println!("Moveしました{}", reference);
    let s =String::from("Hello world");
    let _word = first_word(&s);
    slice();
    structuser::user1();

}

fn nijuukaihou(){
    let s1 = String::from("Hello");
    println!("{}, world!", s1);
    println!("s1 ptr :{:p}",s1.as_ptr());
    let s2 = s1;

    println!("{}, world!", s2);
    println!("s2 ptr : {:p}", s2.as_ptr());

    let x = 5;
    let y = x;

    println!("x: {}, y:{}", x,y);
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn dangle() -> String {
    let s = String::from("Hello");
    s
}

fn first_word(s: &str) -> &str {
    let bytest = s.as_bytes();

    for (i , &item) in bytest.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn slice(){
    {

        let numbers = [1,2,34,45];
        let slice = &numbers[1..4];
        let slice_len = slice.len();
        println!("スライス{:?}len:{}", slice,slice_len);
    }
    {
        let numbers = [10,20,30,40,50];
        let slice = &numbers[..];

        for &item in slice{
            println!("{}",item);
        }
    }
    
}