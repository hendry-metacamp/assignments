
#[derive(Debug)]
struct User{
    name:String,
    balance:(f32,String),
}

impl  User{
    fn print_user_details(&self){
        println!("User Name: {}, Balance: {} ${}",self.name,self.balance.1, self.balance.0);
    }

    fn accrue_interest(user:&mut User,rate:f32){
        user.balance.0=user.balance.0*(1.0+rate/100.0);
        user.print_user_details();
    }
}

fn main() {
    println!("Metacamp Home Assignment 1");
    let mut user1=User{
        name:"Hendry".to_owned(),
        balance:(300000.0,"USD".to_owned()),
    };
 
    User::accrue_interest(&mut user1,3.3); 
    User::accrue_interest(&mut user1,3.4); 
    User::accrue_interest(&mut user1,3.5);
    User::accrue_interest(&mut user1,2.0);
    User::accrue_interest(&mut user1,2.6);
 
}
