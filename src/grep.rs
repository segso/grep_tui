use std::{fs, io, path::Path};

pub enum GrepError {
    FileNotFound,
    PathIsNotFile,
    FileSystemIssue(io::Error),
}

pub fn grep(file_contents: String, search: String) -> Vec<(u32, String)> {
    let search_lowercase = search.to_lowercase();
    let case_sensitive = search_lowercase != search;

    (1u32..)
        .zip(file_contents.lines())
        .filter(|(_, line)| {
            if case_sensitive {
                line.contains(&search)
            } else {
                line.to_lowercase().contains(&search_lowercase)
            }
        })
        .map(|(line_number, line)| (line_number, line.to_owned()))
        .collect()
}

pub fn grep_from_file(file: String, search: String) -> Result<Vec<(u32, String)>, GrepError> {
    let path = Path::new(&file);

    let path_exists = path
        .try_exists()
        .map_err(|err| GrepError::FileSystemIssue(err))?;

    if !path_exists {
        return Err(GrepError::FileNotFound);
    }

    if !path.is_file() {
        return Err(GrepError::PathIsNotFile);
    }

    let contents = fs::read_to_string(file).map_err(|err| GrepError::FileSystemIssue(err))?;

    Ok(grep(contents, search))
}
