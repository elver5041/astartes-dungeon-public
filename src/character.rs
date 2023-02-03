pub struct Stats {
    str: u8,
    def: u8,
    agi: u8,
    lib: u8,
    corr: u8,
}

pub enum Size {
    Nonexistent(u8),
    Small(u8),
    Medium(u8),
    Big(u8),
    Huge(u8)
}

pub struct Appearance {
    height: u16,
    weight: u16,
    age: u8,
    gender_app: i8,
    breasts: Option<Size>,
    butt: Option<Size>,
    penis: Option<Size>,
    vagina: Option<Size>,
    anus: Option<Size>
}

pub enum Species {
    Human,
    Elf,
    Goblin
}

pub struct Character {
    max_hp: u32,
    hp: u32,
    max_mp: u32,
    mp: u32,
    stats: Stats,
    appearance: Appearance,
    species: Species,
}