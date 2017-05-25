use std::fmt;

fn main() {
    println!("{} days", 3+12);

    println!("{0} {1}, {1} {0}", "first", "second");

    println!("{nameditem1}, {nameditem2}", nameditem1="first", nameditem2="second");

    println!("do it in binary {:b}", 2);

    println!("{number:>width$}", number=5, width=10);

    println!("{number:>0padding$}", number=5, padding=10);

    //Arg checking: println!("{0} {1}", "Bond");
    println!("{0} {1}", "Bond", "James");

    /* It doesn't print complex types:
    #[allow(dead_code)]
    struct Structure(i32);

    println!("This {} struct won't print", Structure(3));
    */

    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    println!("This {:?} struct won't print", Structure(3).0);

    println!("{number:.3}", number=3.141592);
}
