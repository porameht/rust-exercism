pub struct Allergies {
    score: u32
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_u32 = *allergen as u32;
        self.score & allergen_u32 == allergen_u32
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut vec = vec![];
        let mut i = 1;
        while i <= 128 {
            if i & self.score == i {
                let allergen = match i {
                      1 => Allergen::Eggs,
                      2 => Allergen::Peanuts,
                      4 => Allergen::Shellfish,
                      8 => Allergen::Strawberries,
                     16 => Allergen::Tomatoes,
                     32 => Allergen::Chocolate,
                     64 => Allergen::Pollen,
                    128 => Allergen::Cats,
                    _ => {i = i * 2; continue}
                };
                vec.push(allergen);
            }
            i = i * 2;
        }
        vec
    }
}