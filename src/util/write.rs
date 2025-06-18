use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Writes the byte stream to a file after detecting the file type.
/// Returns the path to the written file.
pub fn save_to_file(
    bytes: &[u8],
    base_name: &str,
    directory: &str,
) -> Result<PathBuf, std::io::Error> {
    let file_type = crate::util::filetype::get_sig(bytes);
    let extension = file_type.map(|t| t.as_str()).unwrap_or("bin"); // fallback if type is unknown

    let mut path = PathBuf::from(directory);
    path.push(format!("{}.{}", base_name, extension));

    let mut file = File::create(&path)?;
    file.write_all(bytes)?;
    file.flush()?;

    Ok(path)
}
