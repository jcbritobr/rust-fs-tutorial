use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

const FILE: &str = "sample.txt";

fn main() {
    {
        println!("Creating file...");
        let _file = File::create(FILE).unwrap();
    }

    let file_metadata = fs::metadata(FILE).unwrap();
    println!(
        "len: {}, last acessed: {:?}, modified {:?}, created {:?}",
        file_metadata.len(),
        file_metadata.accessed(),
        file_metadata.modified(),
        file_metadata.created()
    );

    println!(
        "Is file {}, is dir {}",
        file_metadata.is_file(),
        file_metadata.is_dir()
    );

    println!(
        "Permissions  of file are: {:?}",
        file_metadata.permissions()
    );

    read_file(FILE).unwrap();
    println!("Writing file ...");
    write_file_append(FILE).unwrap();
    fs::write(FILE, "writin someting").unwrap();
    let data = read_to_buffer(FILE).unwrap();
    println!("file content: {}", data);
    println!("Reading file ...");
    read_file(FILE).unwrap();
    println!("Copying files ...");
    copy_files(FILE, "new_file.txt").unwrap();
    println!("Renaming file ...");
    rename_file(FILE).unwrap();
}

fn rename_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::rename(path, "renamed_file.txt")?;
    Ok(())
}

fn write_file_append(filename: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(filename)?;

    file.write_all(b"this is a simple test")?;

    Ok(())
}

fn read_file(filename: &str) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("read: {}", buffer);
    Ok(())
}

fn copy_files<S: AsRef<Path>>(file_a: S, file_b: S) -> io::Result<()> {
    fs::copy(file_a, file_b)?;
    Ok(())
}

fn read_to_buffer<P: AsRef<Path>>(filename: P) -> io::Result<String> {
    let buffer = fs::read(filename)?;
    Ok(String::from_utf8(buffer).unwrap())
}
