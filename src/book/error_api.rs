use std::{error::Error, fmt::Display, fs, io::Read, panic};

use tracing::info;

pub fn error_api() {
    let result = panic::catch_unwind(|| "No problem here!");
    info!("1.{:?}", result);

    let result = panic::catch_unwind(|| {
        let _v = vec![1, 2, 3];
        //v[100] = 4;
    });
    info!("2.{:?}", result);

    // fs::write("config.dat", "hello world").unwrap()
    let username = read_username("config.dat");
    info!("3.{:?}", username);
}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username_file = fs::File::open(path)?;
    let mut username = String::with_capacity(100);

    username_file.read_to_string(&mut username)?;

    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(path.to_string()));
    }

    Ok(username)
}

#[derive(Debug)]
enum ReadUsernameError {
    Io(std::io::Error),
    EmptyUsername(String),
}

impl Error for ReadUsernameError {}
impl Display for ReadUsernameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadUsernameError::Io(e) => write!(f, "IO error: {}", e),
            ReadUsernameError::EmptyUsername(s) => write!(f, "Found no username in {}", s),
        }
    }
}

impl From<std::io::Error> for ReadUsernameError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
