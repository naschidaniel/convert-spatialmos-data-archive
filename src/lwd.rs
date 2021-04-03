/* Converting archived lwd data into the new spatialMOS format */

use serde::Deserialize;
use std::fs;
use std::path::Path;
#[derive(Deserialize)]
struct ReadLwdRecord {
    date: String,
    name: String,
    lat: String,
    lon: String,
    alt: String,
    tp: String,
    wr: String,
    globalstrahlung_oben: String,
    wg: String,
    globalstrahlung_unten: String,
    boe: String,
    t: String,
    ldstat: String,
    rf: String,
    oft: String,
}

fn append_lwd_record(
    path: &str,
    writer: &mut csv::Writer<std::fs::File>,
) -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path(&path)?;
    for record in reader.deserialize() {
        let result: ReadLwdRecord = record.expect("could not be deserialize");
        writer.serialize((
            result.date,
            result.name,
            result.lat,
            result.lon,
            result.alt,
            result.ldstat,
            result.t,
            result.tp,
            result.rf,
            result.boe,
            result.wg,
            result.wr,
            result.oft,
            result.globalstrahlung_oben,
            result.globalstrahlung_unten,
        ))?;
    }
    let rm_file_path = Path::new(&path);

    fs::remove_file(rm_file_path)?;

    Ok(())
}

fn write_lwd(path: String, data_provider: &str, year: &str) -> Result<(), csv::Error> {
    let mut writer = csv::WriterBuilder::new().delimiter(b';').from_path(path)?;

    writer.write_record(&[
        "date",
        "name",
        "lat",
        "lon",
        "alt",
        "ldstat",
        "t",
        "tp",
        "rf",
        "boe",
        "wg",
        "wr",
        "oft",
        "globalstrahlung_oben",
        "globalstrahlung_unten",
    ])?;

    writer.write_record(&[
        "[UTC]",
        "[String]",
        "[Degree]",
        "[Degree]",
        "[m]",
        "[hPa]",
        "[Degree C]",
        "[Degree C]",
        "[Percent]",
        "[m/s]",
        "[m/s]",
        "[Degree]",
        "[Degree C]",
        "[W/m^2]",
        "[W/m^2]",
    ])?;

    for path in fs::read_dir("./data/")? {
        let entry = path?;
        let filename = entry.file_name().into_string().unwrap();
        if filename.contains(data_provider) && filename.contains(year) && filename.contains(".csv")
        {
            let file = entry.path().into_os_string().into_string().unwrap();
            append_lwd_record(&file, &mut writer)?;
        }
    }
    Ok(())
}

pub fn run(data_provider: &str, year: &str) {
    let path = format!("./{}_{}.csv", data_provider, year);
    println!("Converting data for {}", path);
    let handle_result = write_lwd(path, data_provider, year);
    match handle_result {
        Ok(()) => println!("Data conversion completed"),
        Err(error) => {
            println!("Something went wrong! {}", error);
        }
    };
}
