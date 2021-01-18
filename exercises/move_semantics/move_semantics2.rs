// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let mut vec0 = Vec::new();

    // let mut vec1 = fill_vec(vec0);

    // method 1. clone the data
    // let mut vec1 = fill_vec(vec0.clone());

    // method 2. borrow arg & clone or to_vec
    // let mut vec1 = fill_vec(&vec0);

    // method 3. mutably borrow and modify directly
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // vec1.push(88);

    // method 3. mutably borrow and modify directly
    vec0.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // method 3. mutably borrow and modify directly
    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

// fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {    

// method 2. borrow arg & clone or to_vec
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {    

// method 3. mutably borrow and modify directly
fn fill_vec(vec: &mut Vec<i32>) {
    // let mut vec = vec;

    // method 2. borrow arg & clone or to_vec
    // let mut vec = vec.to_vec()

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // vec
}
