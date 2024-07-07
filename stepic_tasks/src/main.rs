fn main() {
    println!("{}", read_num()[0])
}

fn read_num() -> Vec<i32> {
    let mut str = String::new();
    std::io::stdin()
        .read_line(&mut str)
        .unwrap();
    str.trim().split(" ")
        .map(|a| a.parse::<i32>().unwrap()).collect()
}


fn get_first_and_second() -> (f32, f32) {
    let mut str = String::new();
    std::io::stdin()
        .read_line(&mut str)
        .expect("");
    let i = first_num(&str);
    if str != "" {
        let first: f32 = *&str[0..i].trim().parse().expect("");
        let second: f32 = *&str[i + 1..].trim().parse().expect("");
        return (first, second)
    }
    return (0.0, 0.0);
}
fn first_num(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    0
}