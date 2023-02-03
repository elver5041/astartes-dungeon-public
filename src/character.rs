pub struct Stats {
    str: u8,
    def: u8,
    agi: u8,
    lib: u8,
    corr: u8
}

pub enum Size {
    Nonexistent,
    Small(u8), //0-10
    Medium(u8),//10-20
    Big(u8),   //20-30
    Huge(u8)   //30..
}

pub enum Gender {
    Male,
    Female,
    NonBinary
}
pub struct Appearance {
    height: u16,
    weight: u16,
    age: u8,
    gender: Gender,
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
    Goblin,
    Succubus
}

pub struct Character {
    name: String,
    max_hp: u32,
    hp: u32,
    max_mp: u32,
    mp: u32,
    stats: Stats,
    appearance: Appearance,
    species: Species,
    hybrid: Option<Species>
}

impl Character {
    pub fn new() -> Character {
        Character{
            name: String::from("Protagonist"),
            max_hp: 1,
            hp: 1,
            max_mp: 1,
            mp: 1,
            stats: Stats{
                str: 1,
                def: 1,
                agi: 1,
                lib: 1,
                corr: 0,
            },
            appearance: Appearance{
                height: 170,
                weight: 500,
                age: 20,
                gender: Gender::Male,
                gender_app: 100,
                breasts: None,
                butt: Some(Size::Small(5)),
                penis: Some(Size::Medium(15)),
                vagina: None,
                anus: Some(Size::Small(5)),
            },
            species: Species::Human,
            hybrid: None,
        }
    }
}