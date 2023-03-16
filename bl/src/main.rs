use anyhow::Result;
use std::{
    env,
    fmt, fs,
    io::{Write},
    ops::Deref,
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct Revision(String);

impl Revision {
    fn as_split_path(&self) -> PathBuf {
        let mut split = PathBuf::new();
        let mut s = 0;
        let mut e = 2;
        for _ in 0..4 {
            split.push(&self.0[s..e]);
            s += 2;
            e += 2;
        }
        split
    }
}

impl From<usize> for Revision {
    fn from(v: usize) -> Self {
        Self(format!("{v:08X}"))
    }
}

impl fmt::Display for Revision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Revision {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for Revision
where
    T: ?Sized,
    <Revision as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

#[derive(Debug)]
struct SrcFile {
    path: PathBuf,
}

impl SrcFile {
    const EXTENSION: &str = "csv";

    fn new(revision: &Revision) -> Self {
        Self {
            path: revision
                .as_split_path()
                .join(Path::new(revision))
                .with_extension(Self::EXTENSION),
        }
    }

    fn create(&self) -> Result<()> {
        fs::create_dir_all(self.path.parent().unwrap())?;
        fs::File::create(&self.path)?.write_all(b"Hello, world!")?;
        Ok(())
    }
}

#[derive(Debug)]
struct DiffFiles {
    paths: Vec<PathBuf>,
}

impl DiffFiles {
    const EXTENSION: &str = "csv";
    const MAX: usize = 30;

    fn new(latest_revision: &Revision) -> Self {
        let mut paths = Vec::new();
        let mut cnt = 0;
        let mut revision_diff = usize::from_str_radix(latest_revision, 16).unwrap() - 1;
        let dir = latest_revision.as_split_path();

        loop {
            if cnt == Self::MAX || revision_diff == 0 {
                break;
            }
            paths.push(
                dir.join(Path::new(&format!(
                    "{latest_revision}_{}",
                    Revision::from(revision_diff)
                )))
                .with_extension(Self::EXTENSION),
            );
            cnt += 1;
            revision_diff -= 1;
        }

        Self { paths }
    }

    fn create(&self) -> Result<()> {
        self.paths.iter().try_for_each(|path| {
            fs::create_dir_all(path.parent().unwrap())?;
            fs::File::create(path)?;
            Ok(())
        })
    }
}

fn main() {
    let revision = env::args()
        .nth(1)
        .map(|arg| Revision::from(usize::from_str_radix(arg.as_str(), 16).unwrap()))
        .unwrap();

    SrcFile::new(&revision).create().unwrap();
    DiffFiles::new(&revision).create().unwrap();
}
