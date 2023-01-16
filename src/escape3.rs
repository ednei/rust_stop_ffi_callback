use std::{time::Duration,cell::RefCell};

use tokio::{runtime, select};
use tokio_util::sync::CancellationToken;

use crate::jigsaw::Jigsaw;

thread_local!{
    static CANCELATION_TOKEN: RefCell<Option<CancellationToken>> = RefCell::new(None);
}

pub fn main(){
    println!("runner start");
    let basic_rt = runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    
    let cancellation_token = CancellationToken::new();
    let cloned_token = cancellation_token.clone();

    CANCELATION_TOKEN.with(|token_refcell| {
        let _ =token_refcell.borrow_mut().insert(cancellation_token);
    });

    basic_rt.spawn(worker_task2());
    
    basic_rt.block_on(async move {
        // Wait for either cancellation or a very long time
        select! {
            _ = cloned_token.cancelled() => {
                println!("Cancelled!")
            }
            _ = worker_task() => {
                println!("Not Cancelled!")
            }
        }
    });
    println!("survives");
}

async fn worker_task(){
    println!("worker_task1");
    tokio::time::sleep(Duration::from_secs(4)).await;
    println!("Enter JigSaw: ");
    let jigsaw = Jigsaw::new(handler);
    jigsaw.i_want_to_play_a_game();
}

extern "C" fn handler(){
    println!("Stupid guy on Rust: Well, let me just cancel this task!");
    CANCELATION_TOKEN.with(|token_refcell| {
        let cancellation_token =  token_refcell.borrow_mut().take().unwrap();
        cancellation_token.cancel();
    });
    //tokio::task::yield_now().await;
}

async fn worker_task2(){
    println!("worker_task2");
    let mut i = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        i = i+1;
        println!("tick {i}");
    }
}