use std::{
    fs::{self, DirBuilder},
    path::{Path, PathBuf},
};

fn main() {
    // Gets an iterator of all entries in directory
    let dir_entries = fs::read_dir(".").expect("cant read dir");
    // Iterates over all entreies, and get all info and metadata about them.
    for entry in dir_entries {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let entry_metadata = entry.metadata().unwrap();
        let entry_file_type = entry.file_type().unwrap();
        let entry_filename = entry.file_name();

        // Prints all info about the entries
        println!(
            "Path is {:?}.\nMetadata is {:?}\n File_type is {:?}.\nEntry name is {:?}",
            entry_path, entry_metadata, entry_file_type, entry_filename
        );

        let new_path = Path::new("/usr/d1/d2/d3/bar.txt");
        println!("Path parent is: {:?}", new_path.parent());
        for component in new_path.components() {
            println!("Path component is: {:?}", component);
        }

        // Creates a directory using a path and DirBuilder
        let dir_struct = "tmp/dir1/dir2/dir3";
        DirBuilder::new()
            .recursive(true)
            .create(dir_struct)
            .unwrap();

        // Removes created directory
        fs::remove_dir_all(dir_struct).unwrap();

        // Creates directories and a file using PathBuf
        let mut f_path = PathBuf::new();
        f_path.push(r"/tmp");
        f_path.push("packt");
        f_path.push("rust");
        f_path.push("book");
        f_path.set_extension("rs");
        println!("Path constructed is {:?}", f_path);
    }
}
