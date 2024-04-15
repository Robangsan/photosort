use std::{
    fs::{self, read_dir, DirBuilder, DirEntry, File},
    io::BufReader,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use anyhow::Context;

use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use exif::{In, Tag};

pub fn process_files(
    entries: Vec<DirEntry>,
    subfolder: bool,
    path: PathBuf,
    output: Option<PathBuf>,
    copy: bool,
) -> Result<(), anyhow::Error> {
    let exif_reader = exif::Reader::new();
    let pb =  indicatif::ProgressBar::new(entries.len() as u64);

    match copy {
        true => println!("Copying files..."),
        false => println!("Moving files..."),
    }

    for i in entries {
        let mut file_reader = BufReader::new(File::open(i.path())?);
        let date = match exif_reader.read_from_container(&mut file_reader) {
            Ok(e) => match e.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                Some(d) => parse_date_from_str(&d.display_value().to_string()),
                None => get_date_from_file(&i),
            },
            Err(_) => get_date_from_file(&i),
        };

        let new_path = match output {
            Some(ref o) => o,
            None => &path,
        };

        let photo_date = date?;

        let new_path = match subfolder {
            false => new_path
                .join(photo_date.year().to_string().as_str())
                .to_path_buf(),
            true => new_path
                .join(&photo_date.year().to_string().as_str())
                .join(photo_date.month().to_string()),
        };

        new_path
            .try_exists()
            .is_ok()
            .then(|| DirBuilder::new().recursive(true).create(&new_path));

        let new_path = &new_path.join(i.file_name());

        match copy {
            true => {
                if !new_path.exists() {
                    fs::copy(i.path(), &new_path).context("Error while copying file")?;
                }
                ()
            }
            false => fs::rename(i.path(), &new_path).context("Error while moving file")?,
        }

        pb.inc(1);
    };
    pb.finish();
    println!("Done!");
    Ok(())
}

pub fn gather_entries(
    path: &PathBuf,
    recursive: bool,
    exts: &Vec<String>,
) -> Result<Vec<DirEntry>, std::io::Error> {
    println!("Gathering files...");
    let mut entries = Vec::new();
    for f in read_dir(&path)? {
        let f = f?;
        if f.path().is_file() && is_photo(f.file_name().into_string().unwrap(), &exts) {
            entries.push(f);
        } else if recursive && f.path().is_dir() {
            entries.append(&mut gather_entries(&f.path(), recursive, &exts)?)
        }
    }
    println!("Found {} files to sort.", entries.len());
    Ok(entries)
}

fn parse_date_from_str(strdate: &str) -> Result<DateTime<Utc>, anyhow::Error> {
    Ok(NaiveDateTime::parse_from_str(strdate, "%Y-%m-%d %H:%M:%S")
        .context("Error while reading date")?
        .and_utc())
}

fn parse_date_from_duration(duration: &Duration) -> Result<DateTime<Utc>, anyhow::Error> {
    Ok(DateTime::from_timestamp(
        duration.as_secs()
            .try_into()
            .context("Error while reading date")?,
        0,
    )
    .unwrap())
}

fn is_photo(path: String, exts: &Vec<String>) -> bool {
    if let Some((_, e)) = path.rsplit_once('.') {
        exts.contains(&e.to_lowercase())
    } else {
        false
    }
}

fn get_date_from_file(file: &DirEntry) -> Result<DateTime<Utc>, anyhow::Error> {
    parse_date_from_duration(
        &file
            .path()
            .metadata()
            .context("Error while reading file metadata")?
            .created()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use chrono::{Datelike, Timelike};

    use crate::{parse_date_from_duration, parse_date_from_str};

    #[test]
    fn test_parse_date_from_str() {
        let date = parse_date_from_str("2024-04-14 22:33:44").expect("Should parse the date correctly");
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 4);
        assert_eq!(date.day(), 14);
        assert_eq!(date.hour(), 22);
        assert_eq!(date.minute(),33);
        assert_eq!(date.second(), 44);
    }

    #[test]
    #[should_panic(expected = "Should parse the date incorrectly")]
    fn test_parse_date_from_str_error() {
        parse_date_from_str("2024-04-14").expect("Should parse the date incorrectly");
    }

    #[test]
    fn test_parse_date_from_duration() {
        let date = parse_date_from_duration(&Duration::from_millis(1713134024000)).expect("Should parse the date correctly");
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 4);
        assert_eq!(date.day(), 14);
        assert_eq!(date.hour(), 22);
        assert_eq!(date.minute(),33);
        assert_eq!(date.second(), 44);

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let now_parsed = parse_date_from_duration(&now).expect("Should parse the date correctly");
        assert_eq!(now_parsed.timestamp() as u64, now.as_secs());        
    }
}
