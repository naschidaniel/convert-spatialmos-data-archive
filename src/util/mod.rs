use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

pub fn untar_archive_files(data_provider: &str, year: &str) -> Result<(), Error> {
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

    if Path::new("./orig").exists() {
        fs::remove_dir_all("./orig")?;
    }

    Ok(())
}
