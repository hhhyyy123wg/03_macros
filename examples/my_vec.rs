fn main(){
    let v = my_vec![1,2,3];
    println!("{:?}", v);
    let v = my_vec!{1,2,3,};
    println!("{:?}", v);

    <[_]>::into_vec(Box::new([1,2,3,4]));
}

// my_vec! = my_vec!(1, 2, 3);
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr;$n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ($($x:expr),+$(,)?) => {
        <[_]>::into_vec(Box::new([$($x),+]))
    };
}