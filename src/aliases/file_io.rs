use super::AliasList;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub struct AliasFile {
    path: PathBuf,
    file: File,
    aliases: AliasList,
}
// TODO implement length for AliasList
impl AliasFile {
    pub fn new(path: PathBuf) -> Result<AliasFile, Box<dyn Error>> {
        // TODO write errors

        let mut file = File::open(&path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let aliases = AliasList::from(buffer.as_str());

        Ok(AliasFile {
            path,
            file,
            aliases,
        })
    }

    pub fn aliases(&mut self) -> &mut AliasList {
        &mut self.aliases
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
}
