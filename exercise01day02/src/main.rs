fn main() {
    println!("Hello, world!");

    let vec1=vec![1,2,3,4];
    let vec2:Vec<i32>=Vec::new();

    let first=get_first_element(vec1);
    
    match first{
        Some(element)=>println!("First is {}",element),
        None=>println!("empty"),
    }

    let first=get_first_element(vec2);
    match first{
        Some(element)=>println!("First is {}",element),
        None=>println!("empty"),
    }
}


fn get_first_element<T>(vec:Vec<T>)->Option<T>{

let next=vec.into_iter().next();
return next;
}