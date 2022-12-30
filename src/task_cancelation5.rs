use std::{thread, time::Duration};

use tokio::{runtime};

use crate::jigsaw::Jigsaw;

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

    let _worker_task_handle = basic_rt.spawn(worker_task());
    
    let _worker_task_handle2 = basic_rt.spawn(worker_task2());

    basic_rt.block_on( async move {
        println!( "{:?}", _worker_task_handle.await);
    });

    println!("survives");
}

async fn worker_task(){
    println!("worker_task1");
    let jigsaw = Jigsaw::new(handler);
    jigsaw.i_want_to_play_a_game();
}


//https://doc.rust-lang.org/nomicon/unwinding.html

//https://stackoverflow.com/questions/66790155/what-is-the-recommended-way-to-propagate-panics-in-tokio-tasks

// TODO
//1) what happens if instead of panic we resume_unwind?
//https://doc.rust-lang.org/std/panic/fn.resume_unwind.html
extern "C" fn handler(){
    println!("Stupid guy on Rust: Well, I just do the stupid thing JigSaw expect me to do!")
}

async fn worker_task2(){
    println!("worker_task2");
    let mut i = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        //thread::sleep(Duration::from_secs(1)); 
        i = i+1;
        println!("tick {i}");
    }
    //let runner_thead_handle = thread::spawn(|| runner_main(2,queue_rx2));
}