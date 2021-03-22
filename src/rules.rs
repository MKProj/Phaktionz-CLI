pub struct Card {
    pub name: String,
    pub description: String,
}

pub fn types(summmons: [Card; 2], invocations: [Card; 4]) {
    println!("Summmons: ");
    let mut i = 0;
    while i < 2 {
        println!("\t{}: {}", summmons[i].name, summmons[i].description);
        i += 1;
    }
    i = 0;
    println!("Invocations");
    while i < 4 {
        println!("\t{}: {}", invocations[i].name, invocations[i].description);
        i += 1;
    }
}

pub fn game() {
    let mech: [String; 5] = [
        String::from("• When a Summon battles it becomes disabled (turned sideways)"),
        String::from("• To place a Tier 2 or higher summon, you must demote Tiers total to the Summon’s Tier.\n\t– For example, a Tier 2 may be placed by demoting a Tier 2 or 2 Tier 1s."),
        String::from("• At the start of the game, after the turn order is chosen, both players may mulligan any cards in their hand once."),
        String::from("• If a card’s ability were to break one of these rules, the card’s ability takes precedence."),
        String::from( "• When battling, a Player takes DMG equal to the difference between the Summons.\n\t– If a Summon that battles has less DMG than the opposing, no DMG is dealt.\n\t– If a Summon that battles has more DMG than the opposing, the Opponent takes the difference, and the Summon is demoted, except if it’s Tier 3+.")
    ];
    let mut i = 0;
    while i < 5 {
        println!("\t{}", mech[i]);
        i += 1;
    }
}

pub fn terms() {
    let keywords:[String; 11] = [
        String::from("Summons: Units that battle in the battlefield"),
        String::from("Invocations: Sorcery that may be cast to gain benefit"),
        String::from("Abled: The position in which a unit may battle"),
        String::from("Disabled: The position in which a unit is unable to battle (This is done with your Summon being sideways)"),
        String::from("Demote: To have a summon leave the battlefield"),
        String::from("Exile: To remove from play a summon"),
        String::from("Tiers: Represents the rank of a summon, Tier 1 being the lowest and 3 the highest"),
        String::from("DMG: The amount of cards a summon can deal an opponent to lose"),
        String::from("Fizzle: To stop an opponent's play"),
        String::from("L/x: Limit x per turn"),
        String::from("Lx: Limit x per match")
    ];
    let mut i = 0;
    while i < 11 {
        println!("\t{}", keywords[i]);
        i += 1;
    }
}

pub fn c_pile() {
    let cp:[String; 4] = [
        String::from("• A CP Card may be added to your hand if it satisfies the Card’s Create condition."),
        String::from("• Limit: 10 (In Format V1 & V2)"),
        String::from("• Abilities that include ’CP’ refer to Creation Pile"),
        String::from("• CP Cards are identified with CP in the top left along where Tier or Invocation type is located.")
    ];
    let mut i = 0;
    while i < 4 {
        println!("\t{}", cp[i]);
        i += 1;
    }
}

pub fn promote() {
    let pr = String::from(
        "
Promote brings forth two new Tiers, 0 and 4. A Tier 0/4 Summon has a limit of 1.\n
    Tier 0:\n
        \tTier 0’s are Summons that are placed and have a realm invocation ability,\n
        \tas well as having these attributes:\n
        \t• Tier : 0\n
        \t• DMG: 0\n
        \t• Type: T/S\n
        \t• Promote: Yes\n
        \t• All Tier 0’s may not be battled, and treated as a Realm Invocation\n
    Promote: \n
    \tTo Promote a Tier 0, is to flip it to it’s other side where it resides\n
    \tas Tier 4, and is placed at the Tier 3 location. To Promote, the player must\n
    \tsatisfy a Promote condition that is described on the Card. If you control a\n
    \tSummons on the Battlefield and choose to Promote, the Summons will be demoted.\n
    Tier 4:\n
    \tTier 4’s are usually a win condition card and are built to not stay on the\n
    \tfield for long. While you control a Tier 4 you cannot have any other Summons\n
    \ton the Battlefield. As well as that, they have the following attributes:\n
        \t• Tier: 4\n
        \t• DMG: 7/8\n
        \t• Type: T/S\n
        \t• All Tier 4’s cannot be demoted in Battle nor be demoted by any Abilities\n
        \t• At the end of each End Phase the player takes Damage equal to it’s DMG\n
            \t\t– Refusal to pay will result in it being exiled.
    ",
    );
    println!("{}", pr);
}
pub fn list() {
    let opt: [String; 6] = [
        String::from("types"),
        String::from("game-mech"),
        String::from("terms"),
        String::from("c-pile"),
        String::from("promote"),
        String::from("all"),
    ];
    let desc: [String; 6] = [
        String::from("Lists out all Summon/Invocation Types"),
        String::from("Lists out all the Game Mechanics"),
        String::from("Lists out all the key terms"),
        String::from("Describes what Creation Pile is"),
        String::from("Describes what Promote is"),
        String::from("Does all commands above"),
    ];
    println!("Available Options for Rules: ");
    let mut i = 0;
    while i < 6 {
        println!("\t{}: {}", opt[i], desc[i]);
        i += 1;
    }
}

pub fn rules(option: String, summmons: [Card; 2], invocations: [Card; 4]) {
    let opt: [String; 7] = [
        String::from("types"),
        String::from("game-mech"),
        String::from("terms"),
        String::from("c-pile"),
        String::from("promote"),
        String::from("all"),
        String::from("list"),
    ];
    if option == opt[0] {
        types(summmons, invocations);
    } else if option == opt[1] {
        game();
    } else if option == opt[2] {
        terms();
    } else if option == opt[3] {
        c_pile();
    } else if option == opt[4] {
        promote();
    } else if option == opt[5] {
        types(summmons, invocations);
        println!("\n");
        game();
        println!("\n");
        terms();
        println!("\n");
        c_pile();
    } else if option == opt[6] {
        list();
    }
}
