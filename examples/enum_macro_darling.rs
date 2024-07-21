use template::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    let d: Direction<u32> = DirectionUp { speed: 10 }.into();
    let left: Direction<u32> = 10.into();
    println!("{:?}", d);
    println!("{:?}", left);
}
