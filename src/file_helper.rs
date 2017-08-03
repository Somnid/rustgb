use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn read_file_string<T: AsRef<Path>>(path: T) -> Result<String, String> {
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file|{
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_|contents)
        })
}

pub fn read_file_bytes<T: AsRef<Path>>(path: T) -> Result<Vec<u8>, String> {
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file|{
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_|contents)
        })
}

pub fn write_file_from_string<T: AsRef<Path>>(path: T, content: &str) -> Result<(), String>{
    File::create(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file|{
            file.write_all(content.as_bytes())
                .map_err(|err| err.to_string())
        })
}
