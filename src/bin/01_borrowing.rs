// borrowing
fn main() {
    let mut s1:String = String::from("Hello");
    let len:usize = calculate_length(&s1); // &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
    push_string(&mut s1);
    println!("The Length of {} is {}", s1, len);

    // some rules

    let mut word:String = String::from("Hello");
    let w1 = &mut word;
    w1.push_str("Word");
    println!("w1={}", w1);

    let w2:&mut String = &mut word; // DATA RACE OR RACE CONDITION // NO Sync // w1's work should complete before this
    w2.push_str("Code");
    println!("w2={}", w2);

    // println!("w1={}, w2={}", w1,w2); // THIS CAUSES AN ERROR
    // OR 
    // println!("w2={}", w1); // again calling w1 after w2 can cause error
    // Important mental model :

        // w1 = &mut word
        //     ↓
        // w1 has exclusive mutable access
        //     ↓
        // w1 is finished being used
        //     ↓
        // w1's borrow ends
        //     ↓
        // w2 = &mut word
        //     ↓
        // w2 now has exclusive mutable access

    // or even if try to read w1 after its exclusive mutable access is moved you cant because then that r1 would have the exclusive access
            // READ + READ       ✅
            // READ + WRITE      ❌
            // WRITE + READ      ❌
            // WRITE + WRITE     ❌
}

fn calculate_length(s:&String)->usize{ // because s does not own s1, when s goes out of scope nothing happens. It is basically address/pointer.
    return s.len(); // small nuance, you cannot change the value here, no mut operations.

}

// fn push_string(s:&String){
//     s.push_str("world"); // causes an error 
// }

// To Solve This:
fn push_string(s:&mut String){ // solved using &mut String // mutable reference
    s.push_str("world"); // now runs perfectly
}

