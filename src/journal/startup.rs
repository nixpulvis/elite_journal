use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Fileheader {
    pub part: u8,
    pub language: String,
    pub gameversion: String,
    pub build: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NewCommander {
    #[serde(flatten)]
    pub commander: Commander,
    pub package: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Commander {
    #[serde(alias = "Commander")]
    pub name: String,
    #[serde(rename = "FID")]
    pub fid: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LoadGame {
    #[serde(flatten)]
    pub commander: Option<Commander>,
    #[serde(flatten)]
    pub ship: Option<Ship>,
    pub horizons: bool,
    pub game_mode: Option<GameMode>,
    pub credits: u64,
    pub loan: u64,
}

#[derive(Deserialize, Debug)]
pub enum GameMode {
    Open,
    Group,
    Solo,
}

#[derive(Deserialize, Debug)]
pub enum Vessel {
    #[serde(rename = "SRV")]
    Srv,
    Ship,
}

#[derive(Deserialize, Debug)]
pub struct Ship {
    #[serde(rename = "Ship")]
    pub model: String,
    #[serde(rename = "ShipID")]
    pub id: u64,
    #[serde(rename = "ShipName")]
    pub name: String,
    #[serde(rename = "ShipIdent")]
    pub ident: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Cargos {
    pub vessel: Vessel,
    #[serde(default)]
    pub inventory: Vec<Cargo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Cargo {
    pub name: String,
    pub count: u64,
    pub stolen: u64,
    pub mission_id: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Materials {
    pub raw: Vec<Material>,
    pub manufactured: Vec<Material>,
    pub encoded: Vec<Material>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Material {
    pub name: String,
    pub count: u64,
}
