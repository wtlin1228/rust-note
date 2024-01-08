#[derive(Debug)]
enum Size {
    Small,
    Large,
}

#[derive(Debug)]
enum Color {
    White,
    Black,
}

struct Cat {
    name: String,
    id: Option<usize>,
    size: Size,
}

impl Cat {
    fn new(name: String, size: Size) -> Self {
        Self {
            name,
            id: None,
            size,
        }
    }
}

impl std::fmt::Debug for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cat")
            .field("name", &self.name)
            .field("is_shielded", &self.id.is_some())
            .field("size", &self.size)
            .finish()
    }
}

struct Dog {
    name: String,
    id: Option<usize>,
    color: Color,
}

impl Dog {
    fn new(name: String, color: Color) -> Self {
        Self {
            name,
            id: None,
            color,
        }
    }
}

impl std::fmt::Debug for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dog")
            .field("name", &self.name)
            .field("is_shielded", &self.id.is_some())
            .field("color", &self.color)
            .finish()
    }
}

trait GetInAnimalShelter {
    fn tag(&mut self, id: usize);
}

impl GetInAnimalShelter for Cat {
    fn tag(&mut self, id: usize) {
        self.id = Some(id);
    }
}

impl GetInAnimalShelter for Dog {
    fn tag(&mut self, id: usize) {
        self.id = Some(id);
    }
}

struct AnimalShelter {
    next_id: usize,
}

impl AnimalShelter {
    fn new() -> Self {
        Self { next_id: 0 }
    }

    fn shield_static_dispatch(&mut self, animal: &mut impl GetInAnimalShelter) {
        self.next_id += 1;
        animal.tag(self.next_id);
    }

    fn shield_dynamic_dispatch(&mut self, animal: &mut dyn GetInAnimalShelter) {
        self.next_id += 1;
        animal.tag(self.next_id);
    }
}

fn main() {
    let mut cat_butter = Cat::new("Butter".to_string(), Size::Small);
    let mut cat_peanut = Cat::new("Peanut".to_string(), Size::Large);
    let mut dog_chocolate = Dog::new("Choco".to_string(), Color::Black);
    let mut dog_marshmallow = Dog::new("Marsho".to_string(), Color::White);

    let mut shelter = AnimalShelter::new();
    shelter.shield_static_dispatch(&mut cat_butter);
    shelter.shield_static_dispatch(&mut cat_peanut);
    shelter.shield_dynamic_dispatch(&mut dog_chocolate);
    shelter.shield_dynamic_dispatch(&mut dog_marshmallow);

    println!("{:?}", cat_butter);
    println!("{:?}", cat_peanut);
    println!("{:?}", dog_chocolate);
    println!("{:?}", dog_marshmallow);
}
