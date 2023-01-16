use std::cell::RefCell;
use std::time::Duration;

use tokio::{runtime};
use tokio::select;
use tokio_util::sync::CancellationToken;

thread_local!{
    static CANCELATION_TOKEN: RefCell<Option<CancellationToken>> = RefCell::new(None);
}

//With asynchronous Rust, cancellation is performed by DROPPING a future
//Ready all
//https://tokio.rs/tokio/tutorial/select

//https://isocpp.org/wiki/faq/mixing-c-and-cpp

//https://docs.rs/tokio/1.21.2/tokio/runtime/struct.Builder.html

//https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html

pub fn main(){
    let basic_rt = runtime::Builder::new_current_thread()
    .enable_time()
    .build()
    .unwrap();
    
    let cancellation_token = CancellationToken::new();
    let cloned_token = cancellation_token.clone();

    CANCELATION_TOKEN.with(|token_refcell| {
        let _ =token_refcell.borrow_mut().insert(cancellation_token);
    });

    basic_rt.spawn(async move {
        // select the future that resolve first and cancel(=drop) the other
        select! {
            _ = cloned_token.cancelled() => {
                println!("Task 1 Cancelled!")
            }
            _ = worker_task() => {
                println!("Task 1 Completed!")
            }
        }
    });

    basic_rt.block_on(worker_task2());
    
    println!("Looks like we have a plan to survive JigSaw");

}

async fn worker_task(){
    println!("worker_task");
    tokio::time::sleep(Duration::from_secs(4)).await;
    
    CANCELATION_TOKEN.with(|token_refcell| {
        let cancellation_token =  token_refcell.borrow_mut().take().unwrap();
        cancellation_token.cancel();
    });
    
    tokio::task::yield_now().await; // You will miss me 
}

async fn worker_task2(){
    println!("worker_task2");
    let mut i = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        i = i+1;
        println!("tick {i}");
        if i==5{
            break;
        } 
    }
}
