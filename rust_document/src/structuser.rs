struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn user1(){
    
    let user1 = User{
        username: String::from("user123"),
        email: String::from("user123@example.com"),
        sign_in_count: 32,
        active: true,
    };

    println!("user1 name:{}", user1.username);
}

