#[derive(Debug)]
struct A<T>(T);

fn main() {
    let a = A(0);
    println!("{:?}", a);
}
