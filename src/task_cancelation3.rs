use std::{time::Duration};

use tokio::{runtime};

pub fn task_cancelation_main(){
    runner_main();
}

fn runner_main(){
    println!("runner start");
    let basic_rt = runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();

    basic_rt.spawn(worker_task());
    
    let worker_task_handle2 = basic_rt.spawn(worker_task2());

    basic_rt.block_on( async move {
        println!( "{:?}", worker_task_handle2.await);
    });

    println!("survives");
}

async fn worker_task(){
    println!("worker_task1");
    tokio::time::sleep(Duration::from_secs(3)).await; 
    handler(); 
}

//https://doc.rust-lang.org/nomicon/unwinding.html

//https://stackoverflow.com/questions/66790155/what-is-the-recommended-way-to-propagate-panics-in-tokio-tasks

fn handler(){
    panic!("die here");
}

async fn worker_task2(){
    println!("worker_task2");
    let mut i = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        //thread::sleep(Duration::from_secs(1)); 
        i = i+1;
        println!("tick {i}");
        if i==5{
            break;
        } 
    }
    //let runner_thead_handle = thread::spawn(|| runner_main(2,queue_rx2));
}

