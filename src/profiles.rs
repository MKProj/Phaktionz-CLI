pub struct Profile {
    pub fname: String,
    pub lname: String,
    pub age: i64,
    pub height: String,
    pub factions: String,
    pub decks: String,
    pub description: String,
}

pub fn prof(option: String) {
    let characters = [
       Profile{
            fname: String::from("Zane"),
            lname: String::from("Fulmore"),
            age: 17,
            height: String::from("5\'7\""),
            factions: String::from("
            1.Warriors\n
            2.Sorcerers\n
            3.Dragons\n
        "),
            decks: String::from("
            1.Warriors - Basic\n
            2.Warriors - The Warrior’s Path\n
            3.Sorcerers - Prestige’s Will\n
            4.Dragons - Dragonic Reign\n
            5.Warriors/Sorcerers/Dragons - Z Throttle Synthesis\n
        "),
        description: String::from("\tZane is the energetic character that is always determined, and pushing himself to do better.\n
    In Season 1 we see him starting out with some success in Phaktionz and as he progresses becomes the best player,\n
    and later becomes full of himself to it, until he sees what greed really becomes of someone.")
        },
        Profile{
            fname: String::from("Lulo"),
            lname: String::from("Tsuzaki"),
            age: 17,
            height: String::from("5'7\""),
            factions: String::from("Mythicals"),
            decks: String::from("
            1.Mythicals - United Army\n
            2.Mythicals - Lin’s Forbidden Army\n
            3.Mythicals - Helio’s Will\n
            4.Mythicals - Lin’s Legacy\n
            "),
            description: String::from("\tLulo is calm and collected, who’s rather the person who keeps the team focus. When in battle,\n
        he is fierce and competitive, he’s secretive and lots of the others don’t know about him, but he works as Finn’s assistant\n
        and lives with him. His family history is unknown, and there’s always something he knows that the others don’t know about.")
        }, 
        Profile{
            fname: String::from("Percy"),
            lname: String::from("Thomson"),
            age: 17,
            height: String::from("5'8\""),
            factions: String::from("
            1.Mythicals\n
            2.Alchemists
            "),
            decks: String::from("
            1.Mythicals - Elvish Army\n
            2.Alchemists - Inorganics
            "),
            description: String::from("\tPercy is the fun, wacky sort of dull person of the group, he’s always trying to motivate the\n
        team in the most weirdest ways, and is dedicated to the game even with his many losses. He’s high spirited and becomes\n
        a whole package of encouragement and headaches.")
        },
        Profile{
            fname: String::from("Khloe"),
            lname: String::from("Bright"),
            age: 18,
            height: String::from("5'5\""),
            factions: String::from("Kingdoms"),
            decks: String::from("
            1.Kingdoms - Insect Manipulation\n
            2.Kingdoms - King’s Will\n
            3.Kingdoms - Bright Kingdom
            "),
            description: String::from("\tKhloe is the “Mom” in the group, where she always makes sure the others are okay, and usually the\n
        one to make the two opposing sides talk. She is very welcoming, and someone who made Zane comfortable in the beginning.")
        },
        Profile{
            fname: String::from("Finn"),
            lname: String::from(" "),
            age: 46,
            height: String::from("6'"),
            factions: String::from("Titans"),
            decks: String::from("Titans - Hibernated Legends"),
            description: String::from("\tFinn is not only the Coach to the team, but a father figure to Zane and Lulo who don’t exactly have a\n
        father to be loved by. He is usually energetic and wild with ideas, however can get serious and depressing when reminded of the\n
        past he lived with, and the disappointments he had caused.")
        }
    ];
    let mut i = 0;
    while i < characters.len() {
        if option == characters[i].fname {
            println!(
                "Name: {} {}\nAge: {}\nHeight: {}\nFaction: {}\nDecks: {}\nDescription: \n{}\n",
                characters[i].fname,
                characters[i].lname,
                characters[i].age,
                characters[i].height,
                characters[i].factions,
                characters[i].decks,
                characters[i].description
            );
        }
        i += 1;
    }
}
