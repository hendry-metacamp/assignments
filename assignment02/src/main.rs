#[derive(Debug, PartialEq, Eq)]
enum PaymentType{
    DigitalToken,
    Cash
}

struct Seller{
    payment_type:PaymentType,
    price:f32,
    balance:f32,
}

#[derive(Debug)]
struct Buyer{
    name:String,
    payment_type:PaymentType,
    balance:f32,
}
#[derive(Debug)]
struct BuyerGroup{
    members:Vec<Buyer>,
}

impl BuyerGroup{

    fn add_member(&mut self,buyer:Buyer){
        self.members.push(buyer);
    }

    fn find_buyer(&self, payment_type:PaymentType)-> i32{
        let mut index:i32=0;        
        for member in &self.members{
          if member.payment_type == payment_type{
            return index;
          }
          index+=1;
        }
        return -1;        
    }

    fn buy(&mut self, buyer_index:usize,seller:&mut Seller){
      
        let mut buyer=&mut self.members[buyer_index];
        println!(">>>>> Starting Buyer bal is {:?}",&buyer.balance);
        println!(">>>>> Starting Seller bal is {:?}",&seller.balance);
        loop{
            if buyer.balance < seller.price{
                break;
            }
            seller.balance = seller.balance + seller.price;
            buyer.balance = buyer.balance - seller.price;
            println!(">>>>> Seller bal is {:?}",&seller.balance);
            println!(">>>>> buyer bal is {:?}",&buyer.balance);        
        }

        println!(">>>>> Ending Buyer bal is {:?}",&buyer.balance);
        println!(">>>>> Ending Seller bal is {:?}",&seller.balance);


    }

    fn print_details(&self){
        println!("Group Info");
        println!("Num of Members: {}",self.members.len())
    }

}

fn main() {
    println!("Metacamp Home Assignment 2");

    //Creating 2 buyers
    let mut buyer1=Buyer{
        name:"John".to_owned(),
        payment_type:PaymentType::DigitalToken,
        balance:100.0,
    };

    let mut buyer2=Buyer{
        name:"Sally".to_owned(),
        payment_type:PaymentType::Cash,
        balance:100.0,
    };

    let mut buyer_group=BuyerGroup{
        members:vec![],
    };

    buyer_group.add_member(buyer1);
    buyer_group.add_member(buyer2);

 

    let buyer_index= buyer_group.find_buyer(PaymentType::Cash);
    println!("buyerr index {}", buyer_index);

    let mut seller=Seller{
        payment_type:PaymentType::Cash,
        price:13.0,
        balance:50.0,
    };

    println!(">>>> Buyer index {}",buyer_index);
    buyer_group.buy(buyer_index as usize,&mut seller);

    buyer_group.print_details();
}
