#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered outer loop");

        'inner: loop {
            println!("Entered inner loop");

            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
