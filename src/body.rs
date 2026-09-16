use crate::de;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use std::fmt;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub enum BodyType {
    Star,
    Planet,
    PlanetaryRing,
    Moon, // TODO: Does this actually exist?
    StellarRing,
    Station,
    AsteroidCluster,

    // Special case for a body's parent being a barycenter
    Null,

    #[serde(untagged)]
    Unknown(String),
}

impl From<&str> for BodyType {
    fn from(name: &str) -> Self {
        serde_json::from_value(serde_json::Value::String(name.to_owned()))
            .unwrap_or_else(|_| Self::Unknown(name.to_owned()))
    }
}

impl fmt::Display for BodyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(name) => write!(f, "{}", name),
            named => write!(f, "{:?}", named),
        }
    }
}

/// What a body's atmosphere is mostly made of
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AtmosphereType {
    Ammonia,
    AmmoniaOxygen,
    AmmoniaRich,
    Argon,
    ArgonRich,
    CarbonDioxide,
    CarbonDioxideRich,
    EarthLike,
    Helium,
    MetallicVapour,
    Methane,
    MethaneRich,
    Neon,
    NeonRich,
    Nitrogen,
    Oxygen,
    SilicateVapour,
    SulphurDioxide,
    Water,
    WaterRich,
    None,

    #[serde(untagged)]
    Unknown(String),
}

impl From<&str> for AtmosphereType {
    fn from(name: &str) -> Self {
        serde_json::from_value(serde_json::Value::String(name.to_owned()))
            .unwrap_or_else(|_| Self::Unknown(name.to_owned()))
    }
}

impl fmt::Display for AtmosphereType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(name) => write!(f, "{}", name),
            named => write!(f, "{:?}", named),
        }
    }
}

/// What a body with a surface has, and a gas giant has none of
///
/// The first three are what a gas giant is scanned without, and so are what
/// decides whether there is a surface here at all. The rest are optional in
/// their own right: a body can have a surface and no volcanism. They are held
/// here rather than on [`Body`] to say where they can and cannot be expected,
/// since none of them means anything without somewhere to stand.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Surface {
    pub atmosphere_type: AtmosphereType,
    #[serde(rename = "SurfacePressure")]
    pub pressure: f32,
    pub composition: Composition,

    /// Whether a ship can be set down on it
    #[serde(default)]
    pub landable: bool,
    /// What the atmosphere is called, where [`Surface::atmosphere_type`] is
    /// what it is made of
    #[serde(default, deserialize_with = "de::empty_str_is_none")]
    pub atmosphere: Option<String>,
    #[serde(default, deserialize_with = "de::empty_str_is_none")]
    pub volcanism: Option<String>,
    #[serde(default, deserialize_with = "de::empty_str_is_none")]
    pub terraform_state: Option<String>,
    /// What can be picked up off it, which is nothing unless it is landable
    #[serde(default)]
    pub materials: Vec<Material>,
}

/// The path a thing takes around whatever it goes round
///
/// The first five arrive together or not at all: a scan of something that
/// orbits nothing carries none of them.
///
/// The last two are sent by the game and not by every uploader that passes its
/// scans on, so either may be absent from a scan carrying the rest. Without
/// them the path is known and where the thing stands along it is not.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Orbit {
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub orbital_inclination: f32,
    pub periapsis: f32,
    pub orbital_period: f32,
    #[serde(default)]
    pub ascending_node: Option<f32>,
    #[serde(default)]
    pub mean_anomaly: Option<f32>,
}

/// How a thing turns on its own axis
///
/// Nothing to do with an orbit, and the two are independent in both
/// directions: a system's primary star turns and goes round nothing, and a
/// barycenter goes round something and does not turn at all.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Spin {
    #[serde(rename = "RotationPeriod")]
    pub period: f32,
    #[serde(rename = "AxialTilt")]
    pub tilt: f32,
}

/// What is known about a thing rather than about the thing itself
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Discovery {
    #[serde(rename = "WasDiscovered")]
    pub discovered: bool,
    #[serde(rename = "WasMapped")]
    pub mapped: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Composition {
    pub ice: f32,
    pub rock: f32,
    pub metal: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Material {
    pub name: String,
    pub percent: f64,
}

pub struct Node {
    pub body_type: BodyType,
    pub body_id: i16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Body {
    #[serde(rename = "BodyID")]
    pub id: i16,
    #[serde(rename = "BodyName")]
    #[serde(alias = "Body")]
    pub name: String,
    #[serde(rename = "BodyType")]
    pub ty: Option<BodyType>,
    /// Distance from primary star in light seconds
    #[serde(rename = "DistanceFromArrivalLS")]
    #[serde(alias = "DistFromStarLS")]
    pub distance_from_arrival: Option<f32>,
    pub parents: Vec<Map<String, i16>>,

    pub planet_class: String, // TODO: e.g. "Rocky body"
    /// [`None`] where the scan does not report it, which a basic one does not
    pub tidal_lock: Option<bool>,
    /// Body masses in units of earth masses
    #[serde(rename = "MassEM")]
    pub mass: f32,
    pub radius: f32,
    /// Measured at the cloud tops where there is no surface, which is why
    /// these two are not part of [`Surface`] though the game names them for
    /// one
    #[serde(rename = "SurfaceGravity")]
    pub gravity: f32,
    /// [`None`] where the scan was a basic one, which does not report it
    #[serde(rename = "SurfaceTemperature")]
    pub temperature: Option<f32>,
    /// [`None`] for a body with no surface, which is to say a gas giant
    #[serde(flatten)]
    pub surface: Option<Surface>,
    #[serde(flatten)]
    pub orbit: Orbit,
    #[serde(flatten)]
    pub spin: Spin,

    // TODO: Ring info
    // pub reserve_level: Option<String>,
    #[serde(flatten)]
    pub discovery: Discovery,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Signal {
    // e.g. Alexandrite
    #[serde(rename = "Type")]
    pub ty: String,
    // #[serde(rename = "Type_Localised")]
    // pub ty_loc: String,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Star {
    #[serde(rename = "BodyName")]
    pub name: String,
    #[serde(rename = "BodyID")]
    pub id: i16,
    /// Empty for the primary, which is what everything else is measured from
    #[serde(default)]
    pub parents: Vec<Map<String, i16>>,

    pub absolute_magnitude: f32,
    #[serde(rename = "Age_MY")]
    pub age_my: i32,
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f32,
    // TODO: enum?
    pub luminosity: String,
    /// What class of star this is, which the game writes as `StarType`
    #[serde(rename = "StarType")]
    pub star_class: String,
    pub stellar_mass: f32,
    pub subclass: i16,

    /// [`None`] for the primary, which goes round nothing
    #[serde(flatten)]
    pub orbit: Option<Orbit>,
    #[serde(flatten)]
    pub spin: Spin,
    pub radius: f32,
    /// A star has no surface either, whatever the game calls this
    #[serde(rename = "SurfaceTemperature")]
    pub temperature: f32,

    #[serde(flatten)]
    pub discovery: Discovery,
    // "Rings": Array [
    //     Object {
    //         "InnerRad": Number(1168900000.0),
    //         "MassMT": Number(123920000000000.0),
    //         "Name": String("Spase IA-Y c17-73 A A Belt"),
    //         "OuterRad": Number(2304600000.0),
    //         "RingClass": String("eRingClass_Rocky"),
    //     },
    //     Object {
    //         "InnerRad": Number(4919500000.0),
    //         "MassMT": Number(7495800000000000.0),
    //         "Name": String("Spase IA-Y c17-73 A B Belt"),
    //         "OuterRad": Number(314850000000.0),
    //         "RingClass": String("eRingClass_MetalRich"),
    //     },
    // ],
}

/// How big a star is, where the game says so.
///
/// The game writes a size only for the classes it distinguishes one for —
/// `M_RedGiant`, `B_BlueWhiteSuperGiant` — and the bare letter otherwise.
/// So `Dwarf` means the main sequence, or that the game did not say.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Copy)]
pub enum StarSize {
    Dwarf,
    Giant,
    SuperGiant,
}

/// What class of star a body is: the game's `StarType`, as an enum.
///
/// A spectral letter with the size beside it, so the hierarchy the game
/// flattens into a token comes back out:
///
/// ```
/// use elite_journal::body::{StarClass, StarSize};
///
/// assert_eq!("M_RedSuperGiant".parse(), Ok(StarClass::M(StarSize::SuperGiant)));
/// assert_eq!("M".parse(), Ok(StarClass::M(StarSize::Dwarf)));
/// ```
///
/// The colour the game spells — `Red`, `BlueWhite` — is not carried: the
/// game pairs a colour only with its own letter, so the letter is the
/// colour and the size is what the token adds. The families off the main
/// sequence keep their own tokens, a white dwarf's `DAZ` and a
/// Wolf-Rayet's `WNC` being spectra rather than sizes.
///
/// [`StarClass::token`] is the one spelling of a class: `serde`,
/// [`Display`](fmt::Display) and [`FromStr`](std::str::FromStr) all go
/// through it. A pair the game has no token for — an O supergiant — writes
/// as the bare letter, which is what the game writes for one.
///
/// `Unknown` keeps a class this crate has no name for, so a journal line
/// carrying one still reads.
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum StarClass {
    // The spectral sequence, hottest first, each with the size the game
    // knows it at. The brown dwarfs carry one too, though the game spells
    // no giant among them.
    O(StarSize),
    B(StarSize),
    A(StarSize),
    F(StarSize),
    G(StarSize),
    K(StarSize),
    M(StarSize),
    L(StarSize),
    T(StarSize),
    Y(StarSize),

    // Pre-main-sequence.
    TTauri,
    HerbigAeBe,

    // Wolf-Rayet, by the lines in the spectrum rather than by size.
    W,
    WN,
    WNC,
    WC,
    WO,

    // The carbon stars.
    CS,
    C,
    CN,
    CJ,
    CH,
    CHd,

    // The S-types, whose tokens collide with the main sequence and are read
    // whole for that reason.
    MS,
    S,

    // The white dwarfs, by spectrum.
    D,
    DA,
    DAB,
    DAO,
    DAZ,
    DAV,
    DB,
    DBZ,
    DBV,
    DO,
    DOV,
    DQ,
    DC,
    DCV,
    DX,

    // The remnants, the proto-star, and the three the game reports where
    // there is no star to report.
    N,
    H,
    SupermassiveBlackHole,
    X,
    RoguePlanet,
    Nebula,
    StellarRemnantNebula,

    /// A class this crate has no name for, kept as it was written.
    Unknown(String),
}

impl StarClass {
    /// The game's own token for this class.
    ///
    /// What a journal writes and what anything keeping a class as text
    /// stores. The one place the spelling lives, so
    /// [`Display`](fmt::Display) and [`FromStr`](std::str::FromStr) cannot
    /// disagree with `serde`.
    pub fn token(&self) -> &str {
        use StarSize::{Dwarf, Giant, SuperGiant};
        match self {
            // The pairs the game spells, and the bare letter for every
            // other pair — which is what the game writes for one.
            StarClass::O(_) => "O",
            StarClass::B(SuperGiant) => "B_BlueWhiteSuperGiant",
            StarClass::B(_) => "B",
            StarClass::A(SuperGiant) => "A_BlueWhiteSuperGiant",
            StarClass::A(_) => "A",
            StarClass::F(SuperGiant) => "F_WhiteSuperGiant",
            StarClass::F(_) => "F",
            StarClass::G(SuperGiant) => "G_WhiteSuperGiant",
            StarClass::G(_) => "G",
            StarClass::K(Giant) => "K_OrangeGiant",
            StarClass::K(_) => "K",
            StarClass::M(Giant) => "M_RedGiant",
            StarClass::M(SuperGiant) => "M_RedSuperGiant",
            StarClass::M(Dwarf) => "M",
            StarClass::L(_) => "L",
            StarClass::T(_) => "T",
            StarClass::Y(_) => "Y",
            StarClass::TTauri => "TTS",
            StarClass::HerbigAeBe => "AeBe",
            StarClass::W => "W",
            StarClass::WN => "WN",
            StarClass::WNC => "WNC",
            StarClass::WC => "WC",
            StarClass::WO => "WO",
            StarClass::CS => "CS",
            StarClass::C => "C",
            StarClass::CN => "CN",
            StarClass::CJ => "CJ",
            StarClass::CH => "CH",
            StarClass::CHd => "CHd",
            StarClass::MS => "MS",
            StarClass::S => "S",
            StarClass::D => "D",
            StarClass::DA => "DA",
            StarClass::DAB => "DAB",
            StarClass::DAO => "DAO",
            StarClass::DAZ => "DAZ",
            StarClass::DAV => "DAV",
            StarClass::DB => "DB",
            StarClass::DBZ => "DBZ",
            StarClass::DBV => "DBV",
            StarClass::DO => "DO",
            StarClass::DOV => "DOV",
            StarClass::DQ => "DQ",
            StarClass::DC => "DC",
            StarClass::DCV => "DCV",
            StarClass::DX => "DX",
            StarClass::N => "N",
            StarClass::H => "H",
            StarClass::SupermassiveBlackHole => "SupermassiveBlackHole",
            StarClass::X => "X",
            StarClass::RoguePlanet => "RoguePlanet",
            StarClass::Nebula => "Nebula",
            StarClass::StellarRemnantNebula => "StellarRemnantNebula",
            StarClass::Unknown(it) => it,
        }
    }

    /// How big the game says this star is, or [`None`] for a family with no
    /// size to say: a white dwarf, a Wolf-Rayet, a remnant.
    pub fn size(&self) -> Option<StarSize> {
        match self {
            StarClass::O(size)
            | StarClass::B(size)
            | StarClass::A(size)
            | StarClass::F(size)
            | StarClass::G(size)
            | StarClass::K(size)
            | StarClass::M(size)
            | StarClass::L(size)
            | StarClass::T(size)
            | StarClass::Y(size) => Some(*size),
            _ => None,
        }
    }
}

impl fmt::Display for StarClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.token())
    }
}

impl Serialize for StarClass {
    /// As the game writes it, which is [`StarClass::token`].
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.token())
    }
}

impl<'de> Deserialize<'de> for StarClass {
    /// From the game's token, through [`FromStr`](std::str::FromStr), which
    /// cannot fail: a token this crate has no name for becomes
    /// [`StarClass::Unknown`].
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<StarClass, D::Error> {
        let token = String::deserialize(deserializer)?;
        Ok(token.parse().expect("parsing a class cannot fail"))
    }
}

impl std::str::FromStr for StarClass {
    type Err = std::convert::Infallible;

    /// A token back into a class, and anything unrecognised into
    /// [`StarClass::Unknown`] — which is why this cannot fail.
    ///
    /// Must stay in step with [`StarClass::token`], which
    /// `every_token_round_trips_through_its_class` checks.
    fn from_str(token: &str) -> Result<StarClass, Self::Err> {
        use StarSize::{Dwarf, Giant, SuperGiant};
        Ok(match token {
            "O" => StarClass::O(Dwarf),
            "B" => StarClass::B(Dwarf),
            "B_BlueWhiteSuperGiant" => StarClass::B(SuperGiant),
            "A" => StarClass::A(Dwarf),
            "A_BlueWhiteSuperGiant" => StarClass::A(SuperGiant),
            "F" => StarClass::F(Dwarf),
            "F_WhiteSuperGiant" => StarClass::F(SuperGiant),
            "G" => StarClass::G(Dwarf),
            "G_WhiteSuperGiant" => StarClass::G(SuperGiant),
            "K" => StarClass::K(Dwarf),
            "K_OrangeGiant" => StarClass::K(Giant),
            "M" => StarClass::M(Dwarf),
            "M_RedGiant" => StarClass::M(Giant),
            "M_RedSuperGiant" => StarClass::M(SuperGiant),
            "L" => StarClass::L(Dwarf),
            "T" => StarClass::T(Dwarf),
            "Y" => StarClass::Y(Dwarf),
            "TTS" => StarClass::TTauri,
            "AeBe" => StarClass::HerbigAeBe,
            "W" => StarClass::W,
            "WN" => StarClass::WN,
            "WNC" => StarClass::WNC,
            "WC" => StarClass::WC,
            "WO" => StarClass::WO,
            "CS" => StarClass::CS,
            "C" => StarClass::C,
            "CN" => StarClass::CN,
            "CJ" => StarClass::CJ,
            "CH" => StarClass::CH,
            "CHd" => StarClass::CHd,
            "MS" => StarClass::MS,
            "S" => StarClass::S,
            "D" => StarClass::D,
            "DA" => StarClass::DA,
            "DAB" => StarClass::DAB,
            "DAO" => StarClass::DAO,
            "DAZ" => StarClass::DAZ,
            "DAV" => StarClass::DAV,
            "DB" => StarClass::DB,
            "DBZ" => StarClass::DBZ,
            "DBV" => StarClass::DBV,
            "DO" => StarClass::DO,
            "DOV" => StarClass::DOV,
            "DQ" => StarClass::DQ,
            "DC" => StarClass::DC,
            "DCV" => StarClass::DCV,
            "DX" => StarClass::DX,
            "N" => StarClass::N,
            "H" => StarClass::H,
            "SupermassiveBlackHole" => StarClass::SupermassiveBlackHole,
            "X" => StarClass::X,
            "RoguePlanet" => StarClass::RoguePlanet,
            "Nebula" => StarClass::Nebula,
            "StellarRemnantNebula" => StarClass::StellarRemnantNebula,
            it => StarClass::Unknown(it.to_owned()),
        })
    }
}

/// Every token the game writes round-trips through the class it means, and
/// every class writes the token it came from.
///
/// [`StarClass::token`] and [`FromStr`](std::str::FromStr) are two tables;
/// nothing but this stops them drifting.
#[test]
fn every_token_round_trips_through_its_class() {
    use std::str::FromStr;

    for token in [
        "O",
        "B",
        "B_BlueWhiteSuperGiant",
        "A",
        "A_BlueWhiteSuperGiant",
        "F",
        "F_WhiteSuperGiant",
        "G",
        "G_WhiteSuperGiant",
        "K",
        "K_OrangeGiant",
        "M",
        "M_RedGiant",
        "M_RedSuperGiant",
        "L",
        "T",
        "Y",
        "TTS",
        "AeBe",
        "W",
        "WN",
        "WNC",
        "WC",
        "WO",
        "CS",
        "C",
        "CN",
        "CJ",
        "CH",
        "CHd",
        "MS",
        "S",
        "D",
        "DA",
        "DAB",
        "DAO",
        "DAZ",
        "DAV",
        "DB",
        "DBZ",
        "DBV",
        "DO",
        "DOV",
        "DQ",
        "DC",
        "DCV",
        "DX",
        "N",
        "H",
        "SupermassiveBlackHole",
        "X",
        "RoguePlanet",
        "Nebula",
        "StellarRemnantNebula",
    ] {
        let class = StarClass::from_str(token).unwrap();
        assert_eq!(class.token(), token, "{token} did not round trip");
        assert_eq!(
            serde_json::from_str::<StarClass>(&format!("\"{token}\"")).unwrap(),
            class,
            "{token} deserialized to something else",
        );
        assert_eq!(
            serde_json::to_string(&class).unwrap(),
            format!("\"{token}\""),
            "{token} serialized to something else",
        );
    }
}

/// The size is readable as a size, beside the letter.
#[test]
fn a_class_carries_its_size() {
    use std::str::FromStr;

    assert_eq!(
        StarClass::from_str("M_RedSuperGiant").unwrap(),
        StarClass::M(StarSize::SuperGiant),
    );
    assert_eq!(
        StarClass::from_str("M").unwrap(),
        StarClass::M(StarSize::Dwarf)
    );
    assert_eq!(
        StarClass::from_str("K_OrangeGiant").unwrap().size(),
        Some(StarSize::Giant),
    );
    // A family with no ladder to sit on says so rather than guessing.
    assert_eq!(StarClass::from_str("DAZ").unwrap().size(), None);
    assert_eq!(StarClass::from_str("H").unwrap().size(), None);

    // Every M is an M, whatever size it is, which is what matching on the
    // letter is for.
    for token in ["M", "M_RedGiant", "M_RedSuperGiant"] {
        assert!(matches!(StarClass::from_str(token).unwrap(), StarClass::M(_)));
    }
}

/// A pair the game has no token for writes as the bare letter, because that
/// is what the game writes for one.
#[test]
fn a_size_the_game_does_not_spell_writes_as_the_letter() {
    assert_eq!(StarClass::O(StarSize::SuperGiant).token(), "O");
    assert_eq!(StarClass::L(StarSize::Giant).token(), "L");
}

/// A class the game has added since this was written is kept rather than
/// refused: a body with a name nobody knows still has a place and a mass.
#[test]
fn an_unknown_star_class_is_kept() {
    let class: StarClass =
        serde_json::from_str(r#""Q_QuiteNew""#).expect("it parses");
    assert_eq!(class, StarClass::Unknown("Q_QuiteNew".to_owned()));
    assert_eq!(class.token(), "Q_QuiteNew");
}
