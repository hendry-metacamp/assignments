
fn is_this_cash(input:String)-> Result<String,String>{
if input=="cash"{
    Ok("Yes this is cash".to_owned())
}else{
    Err("No, not cash".to_owned())
}
}

fn main() {
  let cash=Some("cash".to_owned());
  let credit=Some("credit".to_owned());

  if let Some(payment_type)=&cash{

    match is_this_cash(payment_type.to_string()){
        Ok(mess)=>println!("{}",mess),
        Err(aa)=>println!("{}",aa),
    }
  }

  println!("Cash is {:?}",cash);
}
