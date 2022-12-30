use std::{thread};

use tokio::runtime::{Handle, Runtime};
use tokio::{runtime};
use tokio::select;
use tokio_util::sync::CancellationToken;

pub fn task_cancelation_main(){
    let runner_thread_handle = thread::spawn(|| runner_main());
    runner_thread_handle.join().unwrap();
}

fn runner_main(){
    println!("runner start");
    let basic_rt:Runtime = runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    
        let cancellation_token = CancellationToken::new();
        let cloned_token = cancellation_token.clone();
        let rt_handle = basic_rt.handle();

    
        basic_rt.block_on(async move {
            // Wait for either cancellation or a very long time
            select! {
                _ = cloned_token.cancelled() => {
                    println!("Cancelled!")
                }
                _ = worker_task(rt_handle, cancellation_token) => {
                    println!("Not Cancelled!")
                }
            }
        });

    println!("survives");
}

async fn worker_task(rt_handle: &Handle, cancellation_token:CancellationToken){
    println!("worker_task1");
    //cancellation_token.cancel();
    //tokio::time::sleep(std::time::Duration::from_millis(3)).await;
    //thread::sleep(std::time::Duration::from_secs(3));
    handler(rt_handle,cancellation_token); 
}


// TODO
//2) brink back the runtime context,Call the cancellation token, sleep and await.
//https://docs.rs/tokio/1.21.2/tokio/runtime/struct.Runtime.html#method.enter

// Cannot await outside a async function, so its not possible using
// tokio sleep or yield
fn handler(rt_handle: &Handle, cancellation_token:CancellationToken){

    // Cannot block_on : panicked at 'Cannot start a runtime from within a runtime. 
    // This happens because a function (like `block_on`) attempted to block the 
    // current thread while the thread is being used to drive asynchronous tasks.'
    rt_handle.block_on(async move {
        cancellation_token.cancel();
        tokio::time::sleep(std::time::Duration::from_millis(3)).await;
    });
    panic!("die here");
}

