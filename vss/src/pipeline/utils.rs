use std::io::Cursor;
use std::path::Path;

#[cfg(not(target_os = "android"))]
pub fn load<P: AsRef<Path>>(path: P) -> Cursor<Vec<u8>> {
    use std::fs::File;
    use std::io::Read;

    let mut buf = Vec::new();
    let full_path = &Path::new("").join(&path);
    let mut file = match File::open(&full_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("Can`t open file '{}' ({})", full_path.display(), err);
        }
    };
    match file.read_to_end(&mut buf) {
        Ok(_) => Cursor::new(buf),
        Err(err) => {
            panic!("Can`t read file '{}' ({})", full_path.display(), err);
        }
    }
}

#[cfg(target_os = "android")]
pub fn load<P: AsRef<Path>>(path: P) -> Cursor<Vec<u8>> {
    use android_glue;

    let filename = path.as_ref().to_str().expect("Can`t convert Path to &str");
    match android_glue::load_asset(filename) {
        Ok(buf) => Cursor::new(buf),
        Err(_) => panic!("Can`t load asset '{}'", filename),
    }
}
