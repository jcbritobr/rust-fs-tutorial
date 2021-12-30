use std::{fs, path::Path};

use crate::errors::StatsError;

pub struct SrcStats {
    pub number_of_files: u32,
    pub loc: u32,
    pub comments: u32,
    pub blanks: u32,
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
        number_of_files: u32::try_from(file_contents.lines().count())?,
        blanks,
        comments,
        loc,
    };

    Ok(source_stats)
}
