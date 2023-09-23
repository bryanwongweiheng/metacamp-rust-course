struct Griffin {
    name: String,
    magic_power: u32,
}

struct Unicorn {
    name: String,
    magic_power: u32,
}

enum Creature {
    Griffin(Griffin),
    Unicorn(Unicorn),
}

impl Creature {
    fn magic_power(&self) -> u32 {
        match self {
            Creature::Griffin(griffin) => griffin.magic_power,
            Creature::Unicorn(unicorn) => unicorn.magic_power,
        }
    }
    fn name(&self) -> String {
        match self {
            Creature::Griffin(griffin) => griffin.name[..].to_owned(),
            Creature::Unicorn(unicorn) => unicorn.name[..].to_owned(),
        }
    }
    fn clone(&self) -> Creature {
        match self {
            Creature::Griffin(griffin) => Creature::Griffin(Griffin{name: self.name(), magic_power: self.magic_power()}),
            Creature::Unicorn(unicorn) => Creature::Unicorn(Unicorn{name: self.name(), magic_power: self.magic_power()}),
        }
    }
}

fn compare_magic<'a>(c2: &'a Creature, c1: &'a Creature) -> &'a Creature {
    if c1.magic_power() >= c2.magic_power() {
        c1
    } else {
        c2
    }
}

fn creature_box(creature: &Creature) -> Box<Creature> {
    Box::new(creature.clone())
}

fn main() {
    let unicorn = Unicorn{name: "u".to_owned(), magic_power: 3};
    let griffin = Griffin{name: "g".to_owned(), magic_power: 2};
    let unicorn_creature = Creature::Unicorn(unicorn);
    let griffin_creature = Creature::Griffin(griffin);
    let stronger_creature = compare_magic(&unicorn_creature, &griffin_creature);
    let boxed_creature = creature_box(stronger_creature);
    println!("{}, {}", boxed_creature.name(), boxed_creature.magic_power())
}
