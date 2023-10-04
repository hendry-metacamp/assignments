
fn main() {

    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;




        let i = 3; // Lifetime for `i` starts. ────────────────┐
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        println!("borrow1: {}", borrow1); // ││
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        println!("borrow2: {}", borrow2); // ││


}
