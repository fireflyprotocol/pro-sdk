use std::path::PathBuf;
use std::process::exit;
use std::{env, fmt, fs, io};

/// Name of the directory where OpenAPI YAML specs live.
const INPUT_DIR: &str = "resources";

#[derive(Debug)]
pub enum Error {
    /// A command line flag was not recognized.
    Flag(String),
    /// The input directory could not be found.
    Path,
    /// System interaction failed.
    Io(io::Error),
    /// An OpenAPI spec was expected, but not provided.
    Spec,
    /// A subcommand returned bad status.
    Status { command: &'static str },
}

impl Error {
    /// Returns true if this error was definitely the user's mistake.
    fn is_user(&self) -> bool {
        matches!(self, Error::Spec | Error::Flag(_))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Flag(flag) => write!(f, "{flag} is not recognized"),
            Error::Io(err) => write!(f, "{err}"),
            Error::Path => write!(f, "{INPUT_DIR} directory not found"),
            Error::Spec => write!(f, "expected SPEC, where 'resources/SPEC-api.yaml' exists"),
            Error::Status { command } => write!(f, "{command} returned bad status"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

type Result<T> = std::result::Result<T, Error>;

fn input_parent() -> Result<PathBuf> {
    let dir = env::current_dir()?;
    let mut dir = dir.as_path();
    while !dir.join(INPUT_DIR).is_dir() {
        dir = dir.parent().ok_or(Error::Path)?;
    }
    Ok(dir.into())
}

fn run(spec: &str) -> Result<()> {
    let path = format!("{INPUT_DIR}/{spec}-api.yaml");
    let loaded_spec = fs::read_to_string(&path)?;
    let new_spec = loaded_spec.replace("x-user-token", "Authorization");

    fs::File::create(path.clone())?;

    fs::write(&path, new_spec)?;
    Ok(())
}

fn main_imp() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        return Err(Error::Spec);
    }

    env::set_current_dir(input_parent()?)?;

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("usage: apised SPEC [account-data, auth, exchange, trade, websocket]")
            }
            flag if flag.starts_with('-') => return Err(Error::Flag(arg)),
            _ => run(&arg)?,
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = main_imp() {
        eprintln!("error: {err}");
        exit(if err.is_user() { 2 } else { 1 });
    }
}
