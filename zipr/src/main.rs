mod zipr_args;
fn main() {
    if let Err(e) = zipr_args::args().and_then(zipr_args::run){
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
