use mkproj_lib::phaktionz::profiles::*;
const SIZE: usize = 11;
pub fn prof_char() -> [Profile; SIZE] {
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
        },
        Profile{
            fname: String::from("Tesa"),
            lname: String::from("Heinds"),
            age: 17,
            height: String::from("5'5\""),
            factions: String::from("
            1.Probers\n
            2.Cyborgs"),
            decks: String::from("
            1.Probers - Clone Madness\n
            2.Probers - Mad Experiments\n
            3.Probers - Time Shift\n
            4.Probers/Cyborgs - Delphic Synthesis
        "),
            description: String::from("\tTesa is introduced as the Phaktionz Prodigy and considered one of the best players overall.\n
        She is first part of Team Alpha Legends, but later is brought in as an addition to Team Finn, and she likes to always help\n
        the others, mostly if it’s Zane. She plays Probers as it suits her personality of being an intellect and is usually seen\n
        being turns ahead of the opponent.")
        },
        Profile{
            fname: String::from("Aziza"),
            lname: String::from("Cambri"),
            age: 20,
            height: String::from("5'6\""),
            factions: String::from("Kingdoms"),
            decks: String::from("Kingdoms - Insects"),
            description: String::from("\tAziza is a friend of Khloe’s and part of the team Wild Kingdom. She is not a main character\n
        and only appears every so often, but when the Gang needs some help, the Wild Kingdom are usually there for their aid. Aziza also\n
        has a soft personality, except when she’s in a battle, then she’s the complete opposite and is completely competitive.")
        },
        Profile{
            fname: String::from("Svnether"),
            lname: String::from("Panther"),
            age: 20,
            height: String::from("6'2\""),
            factions: String::from("Kingdom"),
            decks: String::from("Kingdoms - Slithering Venom"),
            description: String::from("\tSventher is like the typical emo that is most religious to the stereotypes. Dramatic with every\n
        word that is descended from his mouth to the ever so listening boxes of mankind. He has his hair covering his left side of his face,\n
        and has many snake tattoos. He is the most weirdest member, and in terms of competitiveness, he wishes his opponent to be poisoned, not\n
        clear whether if it’s metaphorical or literal.")
        },
        Profile{
            fname: String::from("Tigero"),
            lname: String::from("Serpentine"),
            age: 20,
            height: String::from("6'2\""),
            factions: String::from("Kingdoms"),
            decks: String::from("Kingdoms - Tigero Charge"),
            description: String::from("\tTigero is one of the fast learning, smart member that others are fooled of due to his jock looks.\n
        He can be heavily competitive with his superior intellects doing planning plays ahead of time.")
        },
        Profile{
            fname: String::from("Elle"),
            lname: String::from("L'Fant"),
            age: 20,
            height: String::from("5'2\""),
            factions: String::from("Kingoms"),
            decks: String::from("Kingdoms - Elephant Rampage"),
            description: String::from("\tElle is the french sweetheart that is beautiful and cute to Zane’s world. Being his first crush,\n
        and girlfriend, we really get to see a lot more of the gentle side of him from her. She is kind and can be scary in battle when\n
        she acts innocent but turns out to have manipulated you to her favor.")
        },
        Profile{
            fname: String::from("Syco"),
            lname: String::from("Liscqunious"),
            age: 18,
            height: String::from("5'4\""),
            factions: String::from("Cyborgs"),
            decks: String::from("
            1.Cyborgs – Overloaded Madness\n
            2.Cyborgs – Override Minds
        "),
            description: String::from("\tSyco is an interesting person, sort of like a mad scientist in a small person. He plays the\n
        Delphic faction, Cyborgs and shows his superior intellect as he thinks of strategy as a simple program. He loves using\n
        opponent’s resources to help him out, and has a dark side that is felt but hard to analyze upon.")
        }
    ];
    characters
}
