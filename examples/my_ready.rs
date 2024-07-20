use std::{pin::Pin, task::{Context, Poll}};  

use futures::Future;  

#[tokio::main]
async fn main(){  
    // // 建立一个 waker 引用，该引用在整个程序中保持一致  
    // let waker = futures::task::noop_waker();  
    // let mut ctx = Context::from_waker(&waker);  

    // // 实例化 MyFuture  
    // let mut fut = MyFuture::new(42);  
    // // Pin 包裹 MyFuture  
    // let mut pinned_fut = Pin::new(&mut fut);  

    // // 首次调用 poll，应该返回 Poll::Pending  
    // let ret = pinned_fut.as_mut().poll(&mut ctx);  
    // println!("First poll: {:?}", ret);  // Expected: Poll::Pending  

    // // 再次调用 poll，应该返回 Poll::Ready  
    // let ret = pinned_fut.as_mut().poll(&mut ctx);  
    // let r = my_ready!(ret);  
    // println!("Second poll (ready value): {:?}", r);  // Expected: 42  

    let fut = MyFuture::new(42);
    let r = fut.await;
    println!("Awaited value: {:?}", r);
}  

// 定义一个自定义 Future 结构体  
struct MyFuture {  
    polled: bool,  
    value: usize,  
}  

impl MyFuture {  
    fn new(v: usize) -> Self {  
        MyFuture {  
            polled: false,  
            value: v,  
        }  
    }  
}  

impl Future for MyFuture {  
    type Output = usize;  

    // 当 Future 被 poll 时，第一个调用返回 Poll::Pending，第二次调用返回 Poll::Ready  
    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {  
        if self.polled {  
            Poll::Ready(self.value)  
        } else {  
            self.polled = true;  
            // wake up the waker
            ctx.waker().wake_by_ref();
            Poll::Pending  
        }  
    }  
}  

// 定义一个宏，用于解开 Poll::Ready 的结果  
#[macro_export]  
macro_rules! my_ready {  
    ($e:expr) => {  
        match $e {  
            std::task::Poll::Ready(v) => v,  
            std::task::Poll::Pending => panic!("Poll::Pending"),  
        }  
    };  
}