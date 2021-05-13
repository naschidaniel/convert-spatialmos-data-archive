/* A data converter for converting archived data into the new spatialMOS format */

mod lwd;
mod util;
mod zamg;

use chrono::Local;
use core::panic;
use std::env;

fn main() {
    let start_time = Local::now().time();
    let args: Vec<String> = env::args().collect();
    let data_provider = &args[1];

    println!("Converting year {}", &args[1]);
    println!("Converting data_provider {}", data_provider);

    // Check Input arg data_provider
    let data_provider_available: [String; 2] = ["zamg".to_string(), "lwd".to_string()];
    if !data_provider_available.contains(data_provider) {
        panic!(
            "The data_provider {} can not be found in {:?}",
            data_provider, data_provider_available
        );
    }

    // Untar all archived files
    let handle_untar = util::untar_archive_files(data_provider);
    match handle_untar {
        Ok(_) => println!("All files have been unpacked"),
        Err(err) => panic!("Something went wrong! {}", err),
    }

    if data_provider == "zamg" {
        zamg::run(data_provider)
    } else if data_provider == "lwd" {
        lwd::run(data_provider)
    }
    let end_time = Local::now().time();
    let diff = end_time - start_time;
    println!("Total time in Seconds {}", diff.num_seconds());
}
