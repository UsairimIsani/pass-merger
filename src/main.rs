use csv::Writer;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{
    collections::{BTreeSet, HashSet},
    io::BufWriter,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Bitwarden {
    encrypted: bool,
    folders: Vec<String>,
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    id: String,
    organizationId: Option<String>,
    folderId: Option<String>,
    #[serde(rename = "type")]
    t_type: u32,
    reprompt: u32,
    name: Option<String>,
    notes: Option<String>,
    favorite: bool,
    login: Login,
    collectionIds: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]

struct Login {
    #[serde(default)]
    uris: Vec<Uris>,
    username: Option<String>,
    password: Option<String>,
    totp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Uris {
    #[serde(rename = "match")]
    m_match: Option<String>,
    uri: String,
}

fn read_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Bitwarden> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    Ok(serde_json::from_reader(reader)?)
}

fn write_to_file<P: AsRef<Path>>(path: P, data: &[LastPass]) -> anyhow::Result<()> {
    // Open the file in read-only mode with buffer.
    let mut wtr = Writer::from_path(path)?;

    // Read the JSON contents of the file as an instance of `User`.
    data.iter().for_each(|s| {
        wtr.serialize(s).unwrap();
    });
    wtr.flush()?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct LastPass {
    url: String,
    username: Option<String>,
    password: Option<String>,
    totp: Option<String>,
    extra: Option<String>,
    name: Option<String>,
    grouping: String,
    fav: u32,
}

impl From<Item> for LastPass {
    fn from(t: Item) -> Self {
        let Item {
            name, notes, login, ..
        } = t;
        let Login {
            mut uris,
            username,
            password,
            totp,
        } = login;

        let url = if let Some(uri) = uris.pop() {
            uri.uri
        } else {
            String::new()
        };
        Self {
            url: url,
            extra: notes,
            totp,
            username,
            fav: 0,
            name,
            grouping: String::from("Merged"),
            password,
        }
    }
}
fn main() -> anyhow::Result<()> {
    let logins = read_from_file("bitwarden.json")?;
    let mut set = HashSet::new();

    let Bitwarden { items, .. } = logins;
    for login in items {
        set.insert(LastPass::from(login));
    }
    let items = set.into_iter().collect::<Vec<LastPass>>();

    write_to_file("merged.csv", &items)?;
    Ok(())
}
