use std::time::Duration;

use tokio::{runtime};

use crate::jigsaw::Jigsaw;

pub fn main(){
    println!("runner start");
    let basic_rt = runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();

    
    basic_rt.spawn(worker_task());

    basic_rt.block_on( worker_task2());

    println!("Owww we survived to JigSaw");
}

async fn worker_task(){
    println!("worker_task1");
    //let worker_task2 start 
    tokio::time::sleep(Duration::from_secs(4)).await;
    println!("Enter JigSaw: ");
    let jigsaw = Jigsaw::new(handler);
    jigsaw.i_want_to_play_a_game();
}

extern "C" fn handler(){
    println!("Stupid guy on Rust: Well, I just do the stupid thing JigSaw expect me to do!")
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