#[derive(serde::Serialize, Debug)]
pub struct Weapon {
    pub name: String,
    pub weapon_type: Type,
    pub def: Defense,
    pub stability: u8,
}

#[derive(serde::Serialize, Debug)]
pub enum Type {
    Common(WeaponModifier),
    Special(OffensiveStats),
}

#[derive(serde::Serialize, Debug, Default, Eq, PartialEq)]
pub struct OffensiveStats {
    pub dmg: Damage,
    pub scaling: Scaling,
}

#[derive(serde::Serialize, Debug, Default, Eq, PartialEq)]
pub struct Damage {
    pub physical: u16,
    pub magic: u16,
    pub fire: u16,
    pub lightning: u16,
}

#[derive(serde::Serialize, Debug, Default, Eq, PartialEq)]
pub struct Scaling {
    pub str: u8,
    pub dex: u8,
    pub int: u8,
    pub faith: u8,
}

#[derive(serde::Serialize, Debug, Eq, PartialEq)]
pub struct Defense {
    pub physical: u8,
    pub magic: u8,
    pub fire: u8,
    pub light: u8,
}

#[derive(serde::Serialize, Debug, strum_macros::EnumString)]
pub enum WeaponModifier {
    Normal(OffensiveStats),
    #[strum(serialize = "Crystal", serialize = "Crys.")]
    Crystal(OffensiveStats),
    #[strum(serialize = "Lightning", serialize = "Ltng.")]
    Lightning(OffensiveStats),
    Raw(OffensiveStats),
    Magic(OffensiveStats),
    #[strum(serialize = "Enchanted", serialize = "Ench.")]
    Enchanted(OffensiveStats),
    #[strum(serialize = "Divine", serialize = "Div.")]
    Divine(OffensiveStats),
    #[strum(serialize = "Occult", serialize = "Occ.")]
    Occult(OffensiveStats),
    Fire(OffensiveStats),
    Chaos(OffensiveStats),
}
