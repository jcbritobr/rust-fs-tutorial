use std::fs;
use std::os::unix::fs as fsunix;

fn main() {
    fs::hard_link("renamed_file.txt", "h_link_renamed_file.txt").unwrap();
    fsunix::symlink("renamed_file.txt", "s_link_renamed_file.txt")
        .expect("Cannot create symbolic link");
    let sym_path = fs::read_link("s_link_renamed_file.txt").expect("Cannot read link");
    println!("Link is {:?}", sym_path);
}
