use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    vec,
};

use crate::errors::StatsError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SrcStats {
    pub number_of_files: u32,
    pub number_of_lines: u32,
    pub loc: u32,
    pub comments: u32,
    pub blanks: u32,
}

pub fn get_summary_src_stats(in_dir: &Path) -> Result<SrcStats, StatsError> {
    let mut total_lines = 0u32;
    let mut total_loc = 0u32;
    let mut total_comments = 0u32;
    let mut total_blanks = 0u32;

    let mut dir_entries: Vec<PathBuf> = vec![in_dir.to_path_buf()];
    let mut file_entries: Vec<DirEntry> = vec![];
    while let Some(entry) = dir_entries.pop() {
        for inner_entry in fs::read_dir(&entry)? {
            if let Ok(entry) = inner_entry {
                if entry.path().is_dir() {
                    dir_entries.push(entry.path());
                } else {
                    if entry.path().extension() == Some(OsStr::new("rs")) {
                        file_entries.push(entry);
                    }
                }
            }
        }
    }
    let file_count = file_entries.len();
    for entry in file_entries {
        let stat = get_src_stats_for_file(&entry.path())?;
        total_lines += stat.number_of_lines;
        total_loc += stat.loc;
        total_blanks += stat.blanks;
        total_comments += stat.comments;
    }

    Ok(SrcStats {
        number_of_files: u32::try_from(file_count)?,
        number_of_lines: total_lines,
        blanks: total_blanks,
        comments: total_comments,
        loc: total_loc,
    })
}

pub fn get_src_stats_for_file(file_name: &Path) -> Result<SrcStats, StatsError> {
    let file_contents = fs::read_to_string(file_name)?;
    let mut loc = 0u32;
    let mut blanks = 0u32;
    let mut comments = 0u32;

    for line in file_contents.lines() {
        if line.len() == 0 {
            blanks += 1;
        } else if line.trim_start().starts_with("//") {
            comments += 1;
        } else {
            loc += 1;
        }
    }
    let source_stats = SrcStats {
        number_of_files: 1,
        number_of_lines: u32::try_from(file_contents.lines().count())?,
        blanks,
        comments,
        loc,
    };

    Ok(source_stats)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::statistics::get_summary_src_stats;

    use super::{get_src_stats_for_file, SrcStats};

    #[test]
    fn test_get_summary_source_stats() {
        let expected = SrcStats {
            loc: 10,
            comments: 4,
            blanks: 4,
            number_of_lines: 18,
            number_of_files: 2,
        };
        let path = Path::new("resources");
        let stats = get_summary_src_stats(path).expect("Cant read files");
        assert_eq!(expected, stats);
    }

    #[test]
    fn test_get_source_stats_for_file() {
        let expected = SrcStats {
            loc: 5,
            comments: 2,
            blanks: 2,
            number_of_lines: 9,
            number_of_files: 1,
        };
        let path = Path::new("resources/sample1.rs");
        let stats = get_src_stats_for_file(path).expect("Cant read file");
        assert_eq!(expected, stats);
    }
}
