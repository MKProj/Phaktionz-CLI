use mkproj_lib::phaktionz::*;
pub fn card_summon() -> [rules::Card; 2] {
    let striker = rules::Card {
        name: String::from("Striker"),
        description: String::from("Can Battle any Opponent's Summons, but not directly"),
    };

    let tech = rules::Card {
        name: String::from("Tech"),
        description: String::from("Can only Battle in the same column, and directly"),
    };
    let summons: [rules::Card; 2] = [striker, tech];
    summons
}
pub fn card_invocation() -> [rules::Card; 4] {
    let regular = rules::Card {
        name: String::from("Regular"),
        description: String::from("This type of invocation may only be cast on your turn"),
    };
    let counter = rules::Card {
        name: String::from("Counter"),
        description: String::from("This type of invocation may be cast on any turn"),
    };

    let weapon = rules::Card {
        name: String::from("Weapon"),
        description: String::from(
            "This type of invocation attaches itself to a Summon on the Battlefield",
        ),
    };

    let realm = rules::Card {
        name: String::from("Realm"),
        description: String::from(
            "This type of invocation remains on the battlefield, with continuous abilities",
        ),
    };
    let invocations: [rules::Card; 4] = [regular, counter, weapon, realm];
    invocations
}
