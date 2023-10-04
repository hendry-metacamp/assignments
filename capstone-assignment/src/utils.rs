use std::collections::HashMap;

use crate::transaction;
use crate::location;



pub fn investments_by_continents(transactions:&Vec<transaction::Transaction>){
 
    let mut map:HashMap<String,f64> = HashMap::new();

    for transaction in transactions{
       // let continent= format!("{:?}", &transaction.continent);
        let continent= location::Continent_as_string(&transaction.continent);
        
        match map.get(&continent){
            Some(v)=> map.insert(continent,(v+transaction.amount)),
            None =>  map.insert(continent,transaction.amount)
        };         
    }

    println!("Investments by Continents: {:?}",map);
}


 pub fn list_by_continent(transactions:&[transaction::Transaction], continent: &location::Continent){
    println!("-------Listing by Continent {} -----------", location::Continent_as_string(&continent));
    for t in transactions.iter().filter(|trans| &trans.continent == continent){
        println!("{:?}",t);
    }

 }