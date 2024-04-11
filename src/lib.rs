use std::{
    fs::{read_dir, rename, DirBuilder, DirEntry, File},
    io::BufReader,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use exif::{In, Tag};

pub fn process_files(entries: Vec<DirEntry>, subfolder: bool) -> Result<(), std::io::Error> {
    let exif_reader = exif::Reader::new();
    Ok(for i in entries {
        let mut file_reader = BufReader::new(File::open(i.path())?);
        let date = match exif_reader.read_from_container(&mut file_reader) {
            Ok(e) => match e.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                Some(d) => parse_date_from_str(&d.display_value().to_string()),
                None => get_date_from_file(&i),
            },
            Err(_) => get_date_from_file(&i),
        };

        let new_path = match subfolder {
            false => Path::new(date.year().to_string().as_str()).to_path_buf(),
            true => Path::new(date.year().to_string().as_str()).join(date.month().to_string()),
        };

        new_path
            .try_exists()
            .is_ok()
            .then(|| DirBuilder::new().recursive(true).create(&new_path));

        rename(i.path(), &new_path.join(i.file_name())).expect("Error al mover la foto");
    })
}

pub fn gather_entries(path: PathBuf, recursive: bool, exts: &Vec<String>) -> Result<Vec<DirEntry>, std::io::Error> {
    let mut entries = Vec::new();
    for f in read_dir(path)? {
        let f = f?;
        if f.path().is_file() && is_photo(f.file_name().into_string().unwrap(), &exts) {
            entries.push(f);
        } else if recursive && f.path().is_dir() {
            entries.append(&mut gather_entries(f.path(), recursive, &exts)?)            
        }
    }
    Ok(entries)
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

fn is_photo(path: String, exts: &Vec<String>) -> bool {
    if let Some((_, e)) = path.rsplit_once('.') {
        exts.contains(&e.to_lowercase())
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
