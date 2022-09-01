// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0: Vec<i32> = Vec::new();

    // Solution 1
    //let vec_tmp = vec0.clone();
    //let mut vec1 = fill_vec(vec_tmp);

    // Solution 2
    //let mut vec1 = fill_vec(vec0.clone());

    // Solution 3
    let mut vec1: Vec<i32> = Vec::new();
    fill_vec_ref(&mut vec1);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_ref(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
