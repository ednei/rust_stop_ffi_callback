mod escape4;
mod escape2;
mod escape3;
mod escape;
mod escape5;
mod jigsaw;

fn main() {
    let arg = std::env::args().nth(1);
    if let Some(version) = arg{
        match version.as_str(){
            "1" => escape::main(),
            "2" => escape2::main(),
            "3" => escape3::main(),
            "4" => escape4::main(),
            "5" => escape5::main(),
            _ => println!("Bad version"),
        }
    }else{
        println!("No version");
    }
}
