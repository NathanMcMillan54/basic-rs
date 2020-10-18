fn main() {
    print!("i = 10 x = 7 j = 4.9 \n");
    let mut i = 10;
    let mut x = 7;
    let mut j = 4.9;

    i = i * x;
    print!("i * x = {}", i);
    i = i / j;
    print!("i / j = {}", i);
}