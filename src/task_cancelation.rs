use std::thread;

use tokio::{runtime, task};
use tokio::select;
use tokio_util::sync::CancellationToken;

//With asynchronous Rust, cancellation is performed by DROPPING a future
//Ready all
//https://tokio.rs/tokio/tutorial/select

//https://isocpp.org/wiki/faq/mixing-c-and-cpp

//https://docs.rs/tokio/1.21.2/tokio/runtime/struct.Builder.html

//https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html

pub fn task_cancelation_main(){
    let (_queue_tx, queue_rx) = async_channel::unbounded::<u64>();

    //thread::spawn(|| server_main(queue_tx));

    //let queue_rx1 = queue_rx.clone();
    //thread::spawn(|| runner_main(1,queue_rx1));

    let queue_rx2 = queue_rx.clone();
    thread::spawn(|| runner_main(2,queue_rx2))
        .join()
        .expect("The receiver thread has panicked");
}

fn runner_main(id:u8, queue_rx: async_channel::Receiver<u64>){
    let mut basic_rt = runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    let local = task::LocalSet::new();

    local.block_on(&mut basic_rt, runner_main_task_1(id,queue_rx))
    
    /*let arg = std::env::args().nth(1);
    if let Some(version) = arg{
        match version.as_str(){
            "1" => local.block_on(&mut basic_rt, runner_main_task_1(id,queue_rx)),
            "2" => local.block_on(&mut basic_rt, runner_main_task_2(id,queue_rx)),
            "3" => local.block_on(&mut basic_rt, runner_main_task_3(id,queue_rx)),
            _ => {},
        }
    }*/
}

async fn runner_main_task_1(_runner_id:u8, _queue_rx: async_channel::Receiver<u64>){
    let token = CancellationToken::new();
    let cloned_token = token.clone();

    let join_handle = tokio::spawn(async move {
        // Wait for either cancellation or a very long time
        select! {
            _ = cloned_token.cancelled() => {
                println!("Cancelled!")
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(9999)) => {
                println!("Not Cancelled!")
            }
        }
    });

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        token.cancel();
    });

    let result = join_handle.await;
    println!("{result:?}");
}
