enum Transactions{
    withdraw,
    deposit,
    transfer,
}


mod transactions;

use transactions::*;

fn main() {
    let tt=Transactions::withdraw;

    match tt{
        Transactions::withdraw=> withdraw(),
        Transactions::deposit=>withdraw(),
        Transactions::transfer=>deposit(), 
    }
}
