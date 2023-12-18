use std::{
    fs::File,
    io::Read,
    process::ExitCode,
};

fn main()-> ExitCode {
    let config = point_record1::Config::get_args();

    let config = match config {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    let reader = match config.in_file {
        point_record1::Input::File(file) => Box::new(match File::open(&file) {
            Ok(file) => Box::new(file) as Box<dyn Read>,
            Err(err) => {
                eprintln!("{}: {err}", file.display());
                return ExitCode::FAILURE;
            }
        }),
        point_record1::Input::Stdin => Box::new(std::io::stdin()) as Box<dyn Read>
    };

    let writer = match File::create(&config.out_file) {
        Ok(writer) => writer,
        Err(err) => {
            eprintln!("{}: {err}", config.out_file.display());
            return ExitCode::FAILURE;
        }
    };

    let points = point_record1::load_points(reader);
    let tagged_point = point_record1::tag_points(&points);
    point_record1::save_points(writer, &points);
    return ExitCode::SUCCESS;
}
   