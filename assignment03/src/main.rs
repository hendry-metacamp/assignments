struct UserAccount{
    name:String,
    age:Option<u32>,
}

trait Balance{
    fn get_balance(&self)->u32{
        return 10;
    }
}


impl Balance for UserAccount{
    
}

fn increase_balance<T:Balance>( x:&T, amount:u32)->Result<u32,String>{
    println!("Increasing balance by {}",amount);
    if amount<=10{
        return Ok(x.get_balance()+amount);
    }else{
        return Err("Increase amount must be less than 10!".to_owned());
    }
 

}

fn main() {
    let account1=UserAccount{
        name:"John".to_owned(),
        age:None,
    };

    let account2=UserAccount{
        name:"Kelly".to_owned(),
        age:Some(34),
    };

    
    match increase_balance(&account1, 11){
        Ok(v)=>println!("UserAccount balance is {}",v),
        Err(e)=>println!("{}",e),
    };


    match increase_balance(&account1, 5){
        Ok(v)=>println!("UserAccount balance is {}",v),
        Err(e)=>println!("{}",e),
    };    

    //Check age of account 1
    if let Some(age) = account1.age{
        println!("Age is {}",age);
    }else{
        println!("Account  {} has no age",&account1.name);
    }

    //Check age of account 2
    if let Some(age) = account2.age{
        println!("Age is {}",age);
    }else{
        println!("Account  {} has no age",&account2.name);
    }



}
