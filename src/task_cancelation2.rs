use std::{thread, time::Duration};

use tokio::{runtime, task::JoinHandle};

pub fn task_cancelation_main(){
    let runner_thread_handle = thread::spawn(|| runner_main());
    runner_thread_handle.join().unwrap();
}

fn runner_main(){
    println!("runner start");
    let basic_rt = runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    
    //TODO we dont want to drop the Task handle, we want to drop the future itself

    let worker_task_handle = basic_rt.spawn(worker_task());
   
    basic_rt.block_on(abort_task(worker_task_handle));
}

async fn worker_task(){
    println!("worker_task");
    let mut i = 0;
    loop {
        //tokio::time::sleep(Duration::from_secs(1)).await; 
        thread::sleep(Duration::from_secs(1)); 
        i = i+1;
        println!("tick {i}");
    }
    //let runner_thead_handle = thread::spawn(|| runner_main(2,queue_rx2));
}

async fn abort_task(worker_task_handle: JoinHandle<()>){
    println!("abort_task");
    tokio::time::sleep(Duration::from_secs(5)).await;
    //drop(worker_task_handle);
    worker_task_handle.abort(); 
    let result = worker_task_handle.await;
    println!("{result:?}");
}

