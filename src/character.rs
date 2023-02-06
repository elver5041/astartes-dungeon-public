pub enum Gender {
    Male, //he him
    Female, //she her
    NonBinary, //they them
    GenderFluid //depends on gender app she-25they25he
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
    gender_app: i8, //masculinity -100..100
}

pub struct Attributes {
    pub str: u8,
    pub def: u8,
    pub agi: u8,
    pub mag: u8,
    pub cha: u8,
    pub lib: u8
}

pub struct Appearance {
    height: u16, //cm
    weight: u16, //cg
    age: u8, //y
    breasts: Option<char>,
    butt: u8,
    penis: Option<u8>,
    vagina: Option<u8>,
    mouth: u8,
    anus: u8,
    species: (Species, Option<Species>)
}

pub struct Character {
    pub name: String,
    pub gender: Gender,
    pub stats: Stats,
    pub attributes: Attributes,
    pub appearance: Appearance
}
pub const CHAR_ATTS: [&str; 18] = ["Name", "Gender", "Str", "Def", "Agi", "Mag", "Cha", "Lib", "Height", "Age", "Breasts", "Butt", "Penis", "Vagina", "Mouth", "Anus", "Species", "Subspecies"];

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
                corr: 0,
                gender_app: 100
            },
            attributes: Attributes{
                str: 1,
                def: 1,
                agi: 1,
                mag: 1,
                cha: 1,
                lib: 1
            },
            appearance: Appearance{
                height: 170,
                weight: 500,
                age: 20,
                breasts: None,
                butt: 5,
                penis: Some(15),
                vagina: None,
                mouth: 15,
                anus: 15,
                species: (Species::Human, None)
            }
        }
    }

    pub fn get_att_value(&self , txt: &str, ) -> String {
        match txt {
            "Name" => String::from(&self.name),
            "Gender" => match self.gender {
                Gender::Male => String::from("Male"), // you will be refered by he/him
                Gender::Female => String::from("Female"), // you will be refered by she/her
                Gender::NonBinary => String::from("NonBinary"), // you will be refered by they/them
                Gender::GenderFluid => String::from("GenderFluid") // how you will be refered by depends on appearence
            },
            "Str" => self.attributes.str.to_string(),
            "Def" => self.attributes.def.to_string(),
            "Agi" => self.attributes.agi.to_string(),
            "Mag" => self.attributes.mag.to_string(),
            "Cha" => self.attributes.cha.to_string(),
            "Lib" => self.attributes.lib.to_string(), 
            "Height" => self.appearance.height.to_string(), 
            "Age" => self.appearance.weight.to_string(), 
            "Breasts" => match self.appearance.breasts {
                None => String::from("No tits"),
                Some(s) => String::from(format!("{} cup",s))
            } 
            "Butt" => match self.appearance.butt {
                0 => String::from("Flat butt"),
                1 => String::from("Small butt"),
                2|3 => String::from("Normal butt"),
                4|5 => String::from("Big butt"),
                _ => String::from("Huge butt")
            }, 
            "Penis" => match self.appearance.penis {
                None => String::from("No penis"),
                Some(s) => match s {
                    0 => String::from("clitty"),
                    1..=9 => String::from("Tiny penis"),
                    10..=13 => String::from("Small penis"),
                    14..=16 => String::from("Normal penis"),
                    17..=19 => String::from("Big penis"),
                    _ => String::from("Huge penis")
                }
            }, 
            "Vagina" => match self.appearance.vagina {
                None => String::from("No vagina"),
                Some(s) => match s {
                    0 => String::from("clitty"),
                    1..=9 => String::from("Tiny vagina"),
                    10..=13 => String::from("Small vagina"),
                    14..=16 => String::from("Normal vagina"),
                    17..=19 => String::from("Deep vagina"),
                    _ => String::from("Spacious vagina")
                }
            }, 
            "Mouth" => match self.appearance.mouth {
                0 => String::from("closed"),
                1..=9 => String::from("Tiny vagina"),
                10..=13 => String::from("Small vagina"),
                14..=16 => String::from("Normal vagina"),
                17..=19 => String::from("Deep vagina"),
                _ => String::from("Spacious vagina")
            }, 
            "Anus" => match self.appearance.anus {
                0 => String::from("closed"),
                1..=9 => String::from("Tiny anus"),
                10..=13 => String::from("Small anus"),
                14..=16 => String::from("Normal anus"),
                17..=19 => String::from("Deep anus"),
                _ => String::from("Spacious anus")
            }, 
            "Species" => match self.appearance.species {
                (Species::Human, _) => String::from("Human"),
                (Species::Elf, _) => String::from("Elf"),
                (Species::Goblin, _) => String::from("Goblin"),
                (Species::Succubus, _) => String::from("Succubus"),
            }, 
            "Subspecies" => match self.appearance.species {
                (_, None) => String::from("Human"),
                (_, Some(Species::Human)) => String::from("Human"),
                (_, Some(Species::Elf)) => String::from("Elf"),
                (_, Some(Species::Goblin)) => String::from("Goblin"),
                (_, Some(Species::Succubus)) => String::from("Succubus")
            },
            _ => String::from("DOOF ERROR")
        }
    }
}