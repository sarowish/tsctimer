use anyhow::{bail, Result};
use std::path::PathBuf;

use crate::app::{Session, Solve};

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

fn get_history_dir() -> Result<PathBuf> {
    let path = match dirs::data_local_dir() {
        Some(path) => path.join(PACKAGE_NAME),
        None => bail!("Couldn't find local data directory"),
    };

    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    Ok(path)
}

pub fn get_sessions_list() -> Result<Vec<PathBuf>> {
    let mut entries = Vec::new();

    for entry in std::fs::read_dir(get_history_dir()?)? {
        let path = entry?.path();
        entries.push(path);
    }

    Ok(entries)
}

pub fn get_session_history_file(file_name: &str) -> Result<PathBuf> {
    Ok(get_history_dir()?.join(file_name))
}

pub fn read_history(path: PathBuf) -> Result<Session> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    let mut session = Session::default();

    for result in rdr.records() {
        let record = result?;

        let solve = Solve::from_history_file(
            record[0].parse().unwrap(),
            record[1].parse().unwrap(),
            &record[2],
            record[3].parse().unwrap(),
        );

        session.solves.push(solve);
    }

    Ok(session)
}

pub fn add_to_history(path: PathBuf, solve: &Solve) -> Result<()> {
    let file = std::fs::OpenOptions::new().append(true).open(path)?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    wtr.write_record(&[
        solve.time.time.to_string(),
        (solve.time.penalty as u8).to_string(),
        solve.scramble.to_string(),
        solve.date.to_string(),
    ])?;

    wtr.flush()?;

    Ok(())
}

pub fn update_history(path: PathBuf, solves: &[Solve]) -> Result<()> {
    let out_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path.with_extension("tmp"))?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(out_file);

    for solve in solves {
        wtr.write_record(&[
            solve.time.time.to_string(),
            (solve.time.penalty as u8).to_string(),
            solve.scramble.to_string(),
            solve.date.to_string(),
        ])?;
    }

    wtr.flush()?;

    std::fs::rename(path.with_extension("tmp"), path)?;

    Ok(())
}
