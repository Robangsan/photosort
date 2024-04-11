use std::{
    fs::{read_dir, rename, DirBuilder, DirEntry, File},
    io::BufReader,
    path::Path,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use exif::{In, Tag};

const FILTER_EXTS: [&str; 7] = ["jpg", "jpeg", "png", "tiff", "tif", "mp4", "avi"];

fn main() -> Result<(), std::io::Error> {
    let mut entries = Vec::new();

    for f in read_dir(".")? {
        let f = f?;
        if f.path().is_file() && is_photo(f.file_name().into_string().unwrap()) {
            entries.push(f);
        }
    }

    let exif_reader = exif::Reader::new();

    for i in entries {
        let mut file_reader = BufReader::new(File::open(i.path())?);
        let date = match exif_reader.read_from_container(&mut file_reader) {
            Ok(e) => {
                match e.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                    Some(d) => parse_date_from_str(&d.display_value().to_string()),
                    None => get_date_from_file(&i),
            }},
            Err(_) => get_date_from_file(&i),
        };
        let date_year = date.year().to_string();
        let path_year = Path::new(date_year.as_str());
        path_year
            .try_exists()
            .is_ok()
            .then(|| DirBuilder::new().create(path_year));

        rename(i.path(), path_year.join(i.file_name())).expect("Error al mover la foto");
    }

    Ok(())
}

fn parse_date_from_str(strdate: &str) -> DateTime<Utc> {
    NaiveDateTime::parse_from_str(strdate, "%Y-%m-%d %H:%M:%S")
        .expect("Error leyendo fecha de la foto")
        .and_utc()
}

fn parse_date_from_duration(duration: &Duration) -> DateTime<Utc> {
    DateTime::from_timestamp(
        duration
            .as_secs()
            .try_into()
            .expect("Error leyendo fecha de la foto"),
        0,
    )
    .unwrap()
}

fn is_photo(path: String) -> bool {
    if let Some((_, e)) = path.rsplit_once('.') {
        FILTER_EXTS.contains(&e.to_lowercase().as_str())
    } else {
        false
    }
}

fn get_date_from_file(file: &DirEntry) -> DateTime<Utc> {
    parse_date_from_duration(
        &file
            .path()
            .metadata()
            .expect("Error leyendo metadatos del fichero")
            .created()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
    )
}

