use psgen::{psgen_args::{create_args, run}};

fn main() {
    if let Err(e) = run(create_args()){
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
