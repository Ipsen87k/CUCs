use mp4decoder::mp4decoder_args::{create_args, run};

fn main() {
    if let Err(e) = create_args().and_then(run)  {
        eprintln!("{}",e);
        std::process::exit(1)
    }
}
