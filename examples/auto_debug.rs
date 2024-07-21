use template::AutoDebug;

#[allow(unused)]
#[derive(AutoDebug)]
pub struct RespBulkString {
    inner: String,
    #[debug(skip)]
    nothing: (),
}

fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
    };
    println!("{:?}", s);
}
