use std::fs;
use std::path::{
    Path,
    PathBuf
};
pub fn copy_dir<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack_vector = Vec::new();
    stack_vector.push(PathBuf::from(from.as_ref()));
    let output = PathBuf::from(to.as_ref());
    let input = PathBuf::from(from.as_ref()).components().count();
    while let Some(this_path) = stack_vector.pop() {
        let source_buffer: PathBuf = this_path.components().skip(input).collect();
        let destination = if source_buffer.components().count() == 0 {
            output.clone()
        } else {
            output.join(&source_buffer)
        };
        if fs::metadata(&destination).is_err() {
            fs::create_dir_all(&destination)?;
        }
        for dir_entry in fs::read_dir(this_path)? {
            let dir_entry = dir_entry?;
            let init_path = dir_entry.path();
            if init_path.is_dir() {
                stack_vector.push(init_path);
            } else {
                match init_path.file_name() {
                    Some(filename) => {
                        let final_path = destination.join(filename);
                        fs::copy(&init_path, &final_path)?;
                    }
                    None => {}
                }
            }
        }
    }
    Ok(())
}