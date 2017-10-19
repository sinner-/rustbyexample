use List::*;

enum List {
    //Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    //Nil: A node that signifies the end of the ll
    Nil,
}

//Methods can be attached to an enum
impl List {
    //Create an empty list
    fn new() -> List {
        //`Nil` has type `List`
        Nil
    }

    //Consume a list and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        //`Cons` also has type `List`
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        //`self` has to be matched, because the behaviour of this method
        //depends on the variant of `self`
        //`self` has type `&List`, and `*self` has type `List`, matching on a
        //concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            //Can't take ownership of the tail because `self` is borrowed;
            //instead take a reference to the tail
            Cons(_, ref tail) => 1+ tail.len(),
            //Base case: empty list has len of 0
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                //`format!` is similar to `print!`, but returns a heap
                //allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }

}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has len: {}", list.len());
    println!("{}", list.stringify());
}
