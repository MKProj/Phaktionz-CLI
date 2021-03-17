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
