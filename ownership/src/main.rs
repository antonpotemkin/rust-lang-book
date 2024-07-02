fn main() {
    let str = String::from("some_string");
    println!("{str}");
    takes_ownership(str); // --- value moved here
    //println!("{str}"); // ^^^^^ value borrowed here after move
    let i = 32;
    println!("before copy {i}");
    make_copy(i);
    println!("after copy {i}");
    let second_str = String::from("second_string");
    println!("{second_str}");
    let second_str = takes_and_gives_back(second_str);
    println!("gives back {second_str}");

    let str_for_len = String::from("first");
    let len = calc_len_and_takes_ownership(str_for_len);
    println!("len is {len}");
    // println!("after calc len {str_for_len}"); // ^^^^^ value borrowed here after move

    let str_for_len = String::from("second");
    let len = calculate_length(&str_for_len);
    println!("len is {len}");
    println!("after calc len {str_for_len}");

    let mut string_for_changes = String::from("string for changes");
    {
        let mut_0 = &mut string_for_changes;
        change_mut_reference(mut_0);
        println!("{mut_0}")
    }
    let mut_1 = &mut string_for_changes;
    // let mut_2 = &mut string_for_changes; //second mutable borrow occurs here
    change_mut_reference(mut_1);
    // println!("{string_for_changes}"); //  ^^^^^^^^^^^^^^^^^^^^ immutable borrow occurs here
    println!("{mut_1}");

    let mut s = String::from("mut s");
    let r1 = & s;
    let r2 = & s;
    println!("r1 and r2 {r1} {r2}"); //variables r1 and r2 will not be used after this point
    let r3 = &mut s;
    println!("r3 {r3}");

    let message = String::from("The text message");
    println!("first world of {} is {}", &message, first_world(&message));
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership {some_string}");
}

fn make_copy(some_i: i32) {
    println!("make_copy {some_i}");
}

fn takes_and_gives_back(input_str: String) -> String {
    input_str
}

// some_string takes ownership
fn calc_len_and_takes_ownership(some_string: String) -> usize {
    some_string.len()
} // Here, s goes out of scope. It is dropped.

// some_string is reference to a String
fn calculate_length(some_string: &String) -> usize {
    some_string.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

fn change_reference(_: &String) {
    // some_string.push('a'); // ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

fn change_mut_reference(some_string: &mut String) {
    some_string.push('!'); // ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

//^ expected named lifetime parameter
// this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// fn dangle() -> &String {
//     let s = String::from("dangle");
//     &s
// }

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }

    }
    &s[..]
}
