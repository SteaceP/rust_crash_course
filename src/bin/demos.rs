fn main() {
    let mut i = 600;
    loop {
        println!("{:?}", i);
        i = i - 1;
        if i == 0 {
            break;
        }
    }
    println!("Done");
}
