use chrono::NaiveDateTime;

use serde::Deserialize;
use std::fs;
use std::path::Path;

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
    path: &str,
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

fn write_zamg(path: String, state: &str) -> Result<(), csv::Error> {
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
        if filename.contains(state) && filename.contains(".csv") {
            let file = entry.path().into_os_string().into_string().unwrap();
            append_zamg_record(&file, &mut writer)?;
        }
    }
    Ok(())
}

pub fn run(data_provider: &str) {
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
        let path = format!("./{}_{}.csv", data_provider, state);
        println!("Converting data for {}", path);
        let handle_result = write_zamg(path, &state.to_string());
        match handle_result {
            Ok(()) => println!("Data conversion completed"),
            Err(error) => {
                println!("Something went wrong! {}", error);
                continue;
            }
        };
    }
}

pub fn date_to_spatialmos_date(timestamp: String) -> String {
    let dt = NaiveDateTime::parse_from_str(&*timestamp, "%Y-%m-%d %H:%M:%S").unwrap();
    format!("{}", NaiveDateTime::format(&dt, "%Y-%m-%d %H:00:00"))
}
