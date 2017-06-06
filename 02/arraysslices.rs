use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("the first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    //fixed size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    //init all elements to same value
    let ys: [i32; 500] = [0; 500];

    //indexing starts at 0
    println!("first element of array: {}", xs[0]);
    println!("second element of array: {}", xs[1]);

    println!("array xs size: {}", xs.len());
    println!("array ys size: {}", ys.len());

    //arrays are stack allocated
    println!("array xs occupies {} bytes", mem::size_of_val(&xs));
    println!("array ys occupies {} bytes", mem::size_of_val(&ys));

    //arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    //slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    //out of bound indexing yields panic
    println!("{}", xs[5]);
}
