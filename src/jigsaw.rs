use std::{ptr::NonNull};


pub type EscapeCallback = extern "C" fn();

#[link(name = "rusty_jigsaw")]
#[link(name = "jigsaw")]
extern "C" {
    fn jigsaw__new(callback:EscapeCallback) -> *mut JigsawOpaque;
    fn jigsaw__i_want_to_play_a_game(this: *mut JigsawOpaque);
}


//https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
#[repr(C)]
#[derive(Debug)]
pub struct JigsawOpaque([u8; 0]);

pub struct Jigsaw{
    ptr_jigsaw: NonNull<JigsawOpaque>,
}

impl Jigsaw{
    pub fn new(escape_callback: EscapeCallback) -> Jigsaw{
        let ptr_jigsaw = unsafe { jigsaw__new(escape_callback) }; 
        Jigsaw { ptr_jigsaw: NonNull::new(ptr_jigsaw).unwrap() }
    }

    pub fn i_want_to_play_a_game(&self){
        unsafe { jigsaw__i_want_to_play_a_game(self.ptr_jigsaw.as_ptr()) }; 
    }
}