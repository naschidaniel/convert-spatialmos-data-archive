/* A data converter for converting archived data into the new spatialMOS format */

mod zamg;

use chrono::Local;
use core::panic;
use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

fn untar_archive_files(data_provider: &String, year: &String) -> Result<(), Error> {
    let archive_path = Path::new("./data/archive/");
    for path in fs::read_dir(archive_path)? {
        let entry = path?;
        let filename = entry.file_name().into_string().unwrap();
        if filename.contains(data_provider)
            && filename.contains(year)
            && filename.contains("tar.gz")
        {
            println!("The file {} is unpacked", filename);
            let tar_file = archive_path.join(filename);
            Command::new("tar")
                .arg("-zxvf")
                .arg(tar_file)
                .output()
                .expect("failed to execute process");
        }
    }

    for path in fs::read_dir("./data/")? {
        let entry = path?;
        let filename = entry.file_name().into_string().unwrap();
        if filename.contains(".html.tmp") {
            fs::remove_file(entry.path())?;
        };
    }

    Ok(())
}

fn main() {
    let start_time = Local::now().time();
    let args: Vec<String> = env::args().collect();
    let data_provider = &args[1];
    let year = &args[2];

    println!("Converting year {}", &args[1]);
    println!("Converting data_provider {}", data_provider);

    // Check Input arg data_provider
    let data_provider_available: [String; 1] = ["zamg".to_string()];
    if !data_provider_available.contains(data_provider) {
        panic!(
            "The data_provider {} can not be found in {:?}",
            data_provider, data_provider_available
        );
    }

    // Check Input arg year
    let year_available: [String; 2] = ["2020".to_string(), "2021".to_string()];
    if !year_available.contains(year) {
        panic!("The year {} can not be found in {:?}", year, year_available);
    }

    let handle_untar = untar_archive_files(data_provider, year);
    match handle_untar {
        Ok(_) => println!("All files have been unpacked"),
        Err(err) => panic!("Something went wrong! {}", err),
    }

    if data_provider == "zamg" {
        zamg::run(data_provider, year)
    }
    let end_time = Local::now().time();
    let diff = end_time - start_time;
    println!("Total time in Seconds {}", diff.num_seconds());
}
