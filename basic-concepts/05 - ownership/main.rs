fn read(y: bool) {
    if y {
        println!("y is true");
    }
}

fn main() {
   // read(x);
   // let x: bool = true;
    let fruit1 = String::from("Apple");
    let fruit2 = fruit1;
    println!("{fruit2}");
    println!("{fruit1}");
}


/*
 *
    Each value in Rust has an owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.

    -- borrow


    fn main() {
	    let mut x = vec![1, 2, 3];
	    modify_vec(&mut x); // mutable borrow of x
	    print_vec(&x); // immutable borrow of x
	}

	fn modify_vec(v: &mut Vec<i32>) {
	    v.push(4);
	}
	fn print_vec(v: &Vec<i32>) {
	    for i in v:
		println!("{}", i);
	    }
	}


    -- lifetime

    fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1` ends. ─────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
    }   
 * */
