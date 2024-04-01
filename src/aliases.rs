#[derive(Debug)]
pub struct Alias {
    pub shortcut: String,
    pub command: String,
}

#[derive(Debug)]
pub struct AliasList {
    pub aliases: Vec<Alias>,
}

impl AliasList {
    pub fn new() -> AliasList {
        AliasList { aliases: vec![] }
    }

    pub fn add_alias(&mut self, alias: Alias) -> Option<()> {
        if self.aliases.iter().any(|a| a.shortcut == alias.shortcut) {
            return None;
        }
        self.aliases.push(alias);

        Some(())
    }
}
