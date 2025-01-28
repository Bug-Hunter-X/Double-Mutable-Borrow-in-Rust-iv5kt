fn main() {
    let mut x = 5;
    { // Scope the mutable borrow
        let y = &mut x; 
        *y += 1;
    }
    { // Another scope to avoid double borrowing
        let z = &mut x; 
        *z += 1; 
    }
    println!("x = {}", x);
}