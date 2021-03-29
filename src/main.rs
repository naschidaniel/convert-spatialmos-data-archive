/* A data converter for converting archived data into the new spatialMOS format */

use chrono::NaiveDateTime;
use core::panic;
use csv;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize)]
struct ReadZamgRecord {
    timestamp_download: String,
    station: String,
    alt: f32,
    t: f32,
    rf: f32,
    wg: f32,
    wr: String,
    wsg: f32,
    regen: f32,
    sonne: f32,
    ldred: f32,
}

fn append_zamg_record(
    path: &String,
    writer: &mut csv::Writer<std::fs::File>,
) -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path(&path)?;
    for record in reader.deserialize() {
        let result: ReadZamgRecord = record.expect("could not be deserialize");
        writer.serialize((
            date_to_spatialmos_date(result.timestamp_download),
            result.station,
            result.alt,
            result.t,
            result.rf,
            result.wg,
            result.wr,
            result.wsg,
            result.regen,
            result.sonne,
            result.ldred,
        ))?;
    }
    let rm_file_path = Path::new(&path);

    fs::remove_file(rm_file_path)?;

    Ok(())
}

fn date_to_spatialmos_date(timestamp: String) -> String {
    let dt = NaiveDateTime::parse_from_str(&*timestamp, "%Y-%m-%d %H:%M:%S").unwrap();
    format!("{}", NaiveDateTime::format(&dt, "%Y-%m-%d %H:00:00"))
}

fn write_zamg(path: String, state: &String, year: &String) -> Result<(), csv::Error> {
    let mut writer = csv::WriterBuilder::new().delimiter(b';').from_path(path)?;

    writer.write_record(&[
        "date", "name", "alt", "t", "rf", "wg", "wr", "boe", "regen", "sonne", "ldred",
    ])?;

    writer.write_record(&[
        "[UTC]",
        "[String]",
        "[m]",
        "[Degree C]",
        "[Percent]",
        "[String]",
        "[km/h]",
        "[km/h]",
        "[mm]",
        "[Percent]",
        "[hPa]",
    ])?;

    for path in fs::read_dir("./data/")? {
        let entry = path?;
        let filename = entry.file_name().into_string().unwrap();
        if filename.contains(state) && filename.contains(year) && filename.contains(".csv") {
            let file = entry.path().into_os_string().into_string().unwrap();
            append_zamg_record(&file, &mut writer)?;
        }
    }
    Ok(())
}

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
        let federal_state: [&str; 9] = [
            "burgenland",
            "kaernten",
            "niederoesterreich",
            "oberoesterreich",
            "salzburg",
            "steiermark",
            "tirol",
            "vorarlberg",
            "wien",
        ];

        for state in federal_state.iter() {
            let path = format!("./{}_{}_{}.csv", data_provider, state, year);
            println!("Converting data for {}", path);
            let handle_result = write_zamg(path, &state.to_string(), year);
            match handle_result {
                Ok(()) => println!("Data conversion completed"),
                Err(error) => {
                    println!("Something went wrong! {}", error);
                    continue;
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_to_spatialmos_date() {
        let rudi = date_to_spatialmos_date("2021-03-28 23:16:23".to_string());
        println!("{}", rudi);
        assert_eq!(
            "2021-03-28 23:00:00",
            date_to_spatialmos_date("2021-03-28 23:16:23".to_string())
        );
        assert_eq!(
            "2021-01-01 02:00:00",
            date_to_spatialmos_date("2021-01-01 02:03:01".to_string())
        );
    }
}
