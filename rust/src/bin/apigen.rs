//! This is how Bluefin generates Rust code from the accompanying OpenAPI specifications.
//!
//! To install this command locally, change to the parent `rust` directory and run:
//!
//! ```sh
//! cargo install --path . --bin apigen
//! ```
//!
//! If you don't have `cargo`, start [here](https://rustup.rs/).

use std::path::PathBuf;
use std::process::{exit, Command};
use std::str::FromStr;
use std::{env, fmt, io};

/// Name of the directory where OpenAPI YAML specs live.
const INPUT_DIR: &str = "resources";

const USAGE: &str = "apigen { -l } { rust | python | ts }. Please ensure that npm and openapi-generator-cli are installed following the instructions at https://openapi-generator.tech/docs/installation";

#[derive(Debug)]
pub enum Error {
    /// A command line flag was not recognized.
    Flag(String),
    /// System interaction failed.
    Io(io::Error),
    /// The supplied value is not a supported target language.
    Lang(String),
    /// The `--lang` argument was expected, but not provided.
    NoLang,
    /// The input directory could not be found.
    Path,
    /// A subcommand returned bad status.
    Status { command: &'static str },
}

impl Error {
    /// Returns true if this error was definitely the user's mistake.
    fn is_user(&self) -> bool {
        matches!(self, Error::Flag(_))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Flag(flag) => write!(f, "{flag} is not recognized"),
            Error::Io(err) => write!(f, "{err}; is `openapi-generator` in your PATH?"),
            Error::Lang(s) => write!(f, "{s} is not a supported language"),
            Error::NoLang => write!(f, "expected --lang LANGUAGE"),
            Error::Path => write!(f, "{INPUT_DIR}: directory not found"),
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

/// The target language in which to generate code.
#[derive(Clone, Copy)]
enum Lang {
    Python,
    Rust,
    Typescript,
}

impl Lang {
    /// Returns a repo-relative path to the OpenAPI definition for this language.
    fn config(self) -> &'static str {
        match self {
            Lang::Python => "python/sdk/config.yaml",
            Lang::Rust => "rust/gen/config.yaml",
            Lang::Typescript => "ts/sdk/openapitools.json",
        }
    }

    /// Returns an OpenAPI generator name.
    fn generator(self) -> &'static str {
        match self {
            Lang::Python => "python",
            Lang::Rust => "rust",
            Lang::Typescript => "typescript-axios",
        }
    }

    /// Returns the target directory path where code should be generated.
    fn output(self) -> &'static str {
        match self {
            Lang::Python => "python/sdk/src",
            Lang::Rust => "rust/gen/bluefin_api",
            Lang::Typescript => "ts/sdk/src",
        }
    }
}

impl FromStr for Lang {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "py" | "python" => Ok(Lang::Python),
            "rs" | "rust" => Ok(Lang::Rust),
            "ts" | "typescript" | "typescript-axios" => Ok(Lang::Typescript),
            _ => Err(Error::Flag(s.to_owned())),
        }
    }
}

/// Invokes the OpenAPI code generator command for the named specification.  For example:
///
/// ```rs
/// generate("websocket-account")?;
/// ```
///
/// # Errors
///
/// Will return `Err` if the OpenAPI generator cannot be found, or if it returns bad status.
fn generate(lang: Lang) -> Result<()> {
    // Check if npm is installed
    let npm_check = Command::new("npm")
        .arg("--version")
        .output()
        .map_err(|_| Error::Status { command: "npm" })?;

    if !npm_check.status.success() {
        return Err(Error::Status { command: "npm" });
    }

    // Check if openapi-generator-cli is installed
    let openapi_check = Command::new("openapi-generator-cli")
        .arg("version")
        .output();

    if openapi_check.is_err() || !openapi_check.unwrap().status.success() {
        return Err(Error::Status {
            command: "openapi-generator-cli",
        });
    }

    // Set openapi-generator-cli version to 7.11.0
    let version_set = Command::new("openapi-generator-cli")
        .args(["version-manager", "set", "7.11.0"])
        .status()
        .map_err(|_| Error::Status {
            command: "openapi-generator-cli version-manager",
        })?;

    if !version_set.success() {
        return Err(Error::Status {
            command: "openapi-generator-cli version-manager",
        });
    }

    let command = "openapi-generator-cli";
    Command::new(command)
        .arg("generate")
        .args(["--input-spec", &format!("{INPUT_DIR}/bluefin-api.yaml")])
        .args(["--config", lang.config()])
        .args(["--generator-name", lang.generator()])
        .args(["--output", lang.output()])
        .status()?
        .success()
        .then_some(())
        .ok_or(Error::Status { command })
}

/// Returns the nearest ancestor of the current working directory containing a "resources" folder.
fn input_parent() -> Result<PathBuf> {
    let dir = env::current_dir()?;
    let mut dir = dir.as_path();
    while !dir.join(INPUT_DIR).is_dir() {
        dir = dir.parent().ok_or(Error::Path)?;
    }
    Ok(dir.into())
}

fn main_imp() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    // TODO(jeff): Validate args before invoking the generator.  They should be the prefixes of file
    //  names in the input ("resources") directory.
    if args.is_empty() {
        return Err(Error::NoLang);
    }

    // You may feel we're being lazy by cd-ing to the top of the repo merely so we can use relative
    // paths in the `generate` function.  You may even be right.  But maybe laziness isn't so bad.
    env::set_current_dir(input_parent()?)?;

    let mut lang = None;
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("usage: {USAGE}");
                return Ok(());
            }
            "-l" | "--lang" => lang = Some(args.next().ok_or(Error::Flag(arg))?.parse()?),
            _ => return Err(Error::Flag(arg)),
        }
    }

    generate(lang.ok_or(Error::NoLang)?)
}

fn main() {
    if let Err(err) = main_imp() {
        eprintln!("error: {err}");
        exit(if err.is_user() { 2 } else { 1 });
    }
}
