
use std::sync::Mutex;
use std::{time::Duration, panic};

use tokio::{runtime};

use libc;

use crate::jigsaw::{Jigsaw};

use std::cell::RefCell;
use std::collections::HashMap;
use lazy_static::lazy_static;

extern {
	fn setjmp(env: *mut libc::c_void) -> libc::c_int;
	fn longjmp(env: *mut libc::c_void, val: libc::c_int);
}

type JmpEnv = [libc::c_int; 27];

//thread_local!(static JIGSAW_ESCAPE_RETURN_POINT: RefCell<HashMap<u8, JmpEnv>> = RefCell::new(HashMap::new()));

lazy_static! {
    static ref JIGSAW_ESCAPE_RETURN_POINT: Mutex<RefCell<HashMap<u8, JmpEnv>>> = Mutex::new(RefCell::new(HashMap::new()));
}


pub fn main(){
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
    // Actually the V8 code does something that makes unwind impossible, what is it?? 
    // Answser: V8 use jumps to change the  stack from Javascript to C
    // https://github.com/v8/v8/blob/main/src/builtins/x64/builtins-x64.cc#L4507 
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    let mut jigsaw_escape_return_point:JmpEnv  = [0; 27];
    let return_from_jump = unsafe {setjmp(
        std::mem::transmute(&mut jigsaw_escape_return_point))};
    
    if return_from_jump != 0 {
        println!("Smart guy: you failed jigsaw, I escaped");
        return;
    }
    
    match JIGSAW_ESCAPE_RETURN_POINT.lock() {
        Ok(map) => map.borrow_mut().insert(1, jigsaw_escape_return_point),
        Err(_) => panic!("Run Forrest"),
    };
    
    
    let jigsaw = Jigsaw::new(handler);
    jigsaw.i_want_to_play_a_game();
}

//https://doc.rust-lang.org/nomicon/unwinding.html

//https://stackoverflow.com/questions/66790155/what-is-the-recommended-way-to-propagate-panics-in-tokio-tasks

extern "C" fn handler(){
    //panic!("Stupid guy panics");

    let mut jigsaw_escape_return_point = match JIGSAW_ESCAPE_RETURN_POINT.lock() {
        Ok(map) => Some(map.borrow_mut().remove(&1).unwrap()),
        Err(_) => None,
    }.unwrap();
    
    unsafe{longjmp(std::mem::transmute(&mut jigsaw_escape_return_point), 1)};
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
}

