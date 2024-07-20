use template::EnumFrom;


#[allow(unused)]
#[derive(Debug,EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32
}

fn main() {
    let d :Direction = DirectionUp { speed: 10 }.into();
    let left :Direction = 10.into();
    println!("{:?}", d);
    println!("{:?}", left);
}