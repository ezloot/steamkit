use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use crate::{Entry, Error, Result, Vpk};

pub struct VpkFile {
    pub path: PathBuf,
    pub vpk: Vpk,
    pub archive_name: String,
}

impl VpkFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = PathBuf::from(path.as_ref());
        let file_name = path
            .file_stem()
            .ok_or(Error::InvalidName)?
            .to_string_lossy();

        // verify file name is valid
        let (archive_name, suffix) = file_name.rsplit_once("_").ok_or(Error::InvalidName)?;
        if suffix != "dir" {
            return Err(Error::InvalidName);
        }

        // open file from path
        let mut file = File::open(&path)?;

        // get buffer from file
        let len = file.metadata()?.len();
        let mut buf = Vec::with_capacity(len as usize);
        file.read_to_end(&mut buf)?;

        // try parse buffer as vpk
        let vpk = Vpk::parse(&buf)?;
        let archive_name = archive_name.to_owned();

        Ok(Self {
            path,
            vpk,
            archive_name,
        })
    }

    pub fn read(&self, path: &str) -> Result<Option<Vec<u8>>> {
        match self.vpk.lookup(path) {
            Some(entry) => {
                if entry.dir_entry.file_length == 0 {
                    return Ok(Some(entry.dir_entry.preload.clone()));
                }

                let mut buf = vec![0; entry.dir_entry.file_length as usize];

                if entry.dir_entry.archive_index == 0x7fff {
                    let mut file = File::open(&self.path)?;
                    let offset =
                        self.vpk.header.data_offset() + entry.dir_entry.archive_offset as u64;

                    file.seek(SeekFrom::Start(offset))?;
                    file.take(entry.dir_entry.file_length as u64)
                        .read(&mut buf)?;
                } else {
                    let mut path = self.path.clone();
                    path.pop();
                    path.push(format!(
                        "{}_{:0>3}.vpk",
                        self.archive_name, entry.dir_entry.archive_index
                    ));

                    let mut file = File::open(path)?;
                    let offset = entry.dir_entry.archive_offset as u64;

                    file.seek(SeekFrom::Start(offset))?;
                    file.take(entry.dir_entry.file_length as u64)
                        .read(&mut buf)?;
                }

                Ok(Some(buf))
            }
            None => Ok(None),
        }
    }

    pub fn lookup(&self, path: &str) -> Option<&Entry> {
        self.vpk.tree.get(path)
    }
}

#[cfg(test)]
mod tests {
    use crate::VpkFile;

    #[test]
    fn read_csgo() {
        let result = VpkFile::open("assets/csgo_dir.vpk");
        assert!(result.is_ok(), "failed to parse CS:GO pack");
    }

    #[test]
    fn read_portal2() {
        let result = VpkFile::open("assets/portal2_dir.vpk");
        assert!(result.is_ok(), "failed to parse CS:GO pack");
    }
}
