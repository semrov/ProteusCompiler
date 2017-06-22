pub mod lexanal;
pub mod synanal;
pub mod xml;
pub mod report;
pub mod abstree;



//use  lexanal::LexToken;

fn main() {
    let program_name = "p2.proteus".to_string();
   // println!("Current working directory: {:?}", std::env::current_dir().unwrap());

    //lexanal::run::run(program_name);
    synanal::run::run(program_name);
}
