use std::io::{Write, Read, Seek, SeekFrom};
use std::{thread, time::Duration};
use std::process::Command;

use tempfile::tempdir;

fn sleep_three_sec() {
    thread::sleep(Duration::from_secs(3));
}

fn main() -> anyhow::Result<()> {
    // write
    let mut tmpfile = tempfile::tempfile().unwrap();
    write!(tmpfile, "Hello World!").unwrap();

    // seek to start
    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    // read
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("Hello World!", buf);

    // Create a directory inside of `std::env::temp_dir()`.
    let temp_dir = tempdir()?;
    println!("{:?}", temp_dir);

    let exit_status = Command::new("touch").arg("tmp").current_dir(&temp_dir).status()?;
    sleep_three_sec();
    assert!(exit_status.success());

    Ok(())
}
