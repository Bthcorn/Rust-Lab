use std::error::Error;
fn main() -> Result<() , Box<dyn Error>> {
    match points_record::get_args().and_then(points_record::run) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}