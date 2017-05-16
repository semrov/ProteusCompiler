pub mod lexanal;
pub mod xml;
pub mod report;



//use  lexanal::Token;

fn main() {
    let program_name = "p1.proteus".to_string();
   // println!("Current working directory: {:?}", std::env::current_dir().unwrap());

    lexanal::run::run(program_name);
}
