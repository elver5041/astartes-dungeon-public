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

pub enum Species {
    Human,
    Elf,
    Goblin,
    Succubus
}
pub struct Stats {
    lvl: u8,
    max_hp: u32,
    hp: u32,
    max_mp: u32,
    mp: u32,
    corr: u8,
}

pub struct Attributes {
    str: u8,
    def: u8,
    agi: u8,
    lib: u8
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
    anus: Option<Size>,
    species: (Species, Option<Species>)
}

pub struct Character {
    name: String,
    gender: Gender,
    stats: Stats,
    attributes: Attributes,
    appearance: Appearance
}

impl Character {
    pub fn new() -> Character {
        Character{
            name: String::from("Protagonist"),
            gender: Gender::Male,
            stats: Stats{
                lvl: 1,
                max_hp: 1,
                hp: 1,
                max_mp: 1,
                mp: 1,
                corr: 0
            },
            attributes: Attributes{
                str: 1,
                def: 1,
                agi: 1,
                lib: 1,
            },
            appearance: Appearance{
                height: 170,
                weight: 500,
                age: 20,
                gender_app: 100,
                breasts: None,
                butt: Some(Size::Small(5)),
                penis: Some(Size::Medium(15)),
                vagina: None,
                anus: Some(Size::Small(5)),
                species: (Species::Human, None)
            }
        }
    }
}

