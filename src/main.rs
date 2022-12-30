mod task_cancelation;
mod task_cancelation2;
mod task_cancelation3;
mod task_cancelation4;
mod task_cancelation5;
mod task_cancelation6;
mod jigsaw;

fn main() {
    println!("Hello, world!");
    let arg = std::env::args().nth(1);
    if let Some(version) = arg{
        match version.as_str(){
            "1" => task_cancelation::task_cancelation_main(),
            "2" => task_cancelation2::task_cancelation_main(),
            "3" => task_cancelation3::task_cancelation_main(),
            "4" => task_cancelation4::task_cancelation_main(),
            "5" => task_cancelation5::task_cancelation_main(),
            "6" => task_cancelation6::task_cancelation_main(),
            _ => println!("Bad version"),
        }
    }else{
        println!("No version");
    }
}
