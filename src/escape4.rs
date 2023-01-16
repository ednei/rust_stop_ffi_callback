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
    
    println!("Pulling hair guy on Rust: Owww my god! we made it! we survived to JigSaw");
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
    panic!("Pulling hair guy on Rust: Oh my god! We all gonna die!!");
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