mod option;
mod zip;
use crate::option::{Mode, Opt};
use structopt::StructOpt;

fn main() {
    let args = match Opt::from_args_safe(){
        Ok(a)=>a,
        Err(e)=>{
            eprintln!("{}", e);
            std::process::exit(-1);
        }
    };
    match args.mode {
        Mode::Zip => {
            if let Err(e)=zip::do_zip(&args.zip_path, &args.file){
                eprintln!("{}", e);
                std::process::exit(-2);
            }
        }
        Mode::Unzip => {
            if let Err(e)=zip::do_unzip(&args.zip_path, &args.dist_dir_path.unwrap()){
                eprintln!("Error: {}", e);
                std::process::exit(-3);
            }
        }
    }
}
