use mkproj_lib::phaktionz::*;
// Story Season Begins
const SIZE: usize = 11;
pub fn ep_all() -> [story::Episode; SIZE] {
    let season1 = [
        story::Episode{
            name: String::from("Concepts"),
            season: 0,
            episode: 0,
            url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/Concepts.pdf")
        },
        story::Episode {
        name: String::from("First Match"),
        season: 1,
        episode: 1,
        url: String::from(
            "https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/01/Single/01.pdf",
        ),
    },
    story::Episode{
            name: String::from("Finn's Fan Fave Shop"),
            season: 1,
            episode: 2,
            url: String::from("https://github.com/MKProj/Phaktionz/blob/main/Concepts/S1/Episodes/02/Single/02.pdf")
        },
        story::Episode{
            name: String::from("The Gang"),
            season: 1,
            episode: 3,
            url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/03/Single/03.pdf")
        },
        story::Episode{
            name: String::from("Lulo's Mystery"),
            season: 1,
            episode: 4,
            url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/04/Single/04.pdf")
        },
        story::Episode{
            name: String::from("Missing"),
            season: 1,
            episode: 5,
            url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/05/Single/05.pdf")
        },
        story::Episode{
            name: String::from("Corruquatro"),
            season: 1,
            episode: 6,
            url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/06/Single/06.pdf")
        },
        story::Episode{
            name: String::from("Mythical Lores"),
            season: 1,
            episode: 7,
            url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/07/Single/07.pdf")
        },
        story::Episode{
            name: String::from("Shop Tournament"),
            season: 1,
            episode: 8,
            url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/08/Single/08.pdf")
        },
        story::Episode{
            name: String::from("Mix and Match"),
            season: 1,
            episode: 11,
            url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/11/Single/11.pdf")
        },
        story::Episode{
            name: String::from("Sleepover!"),
            season: 1,
            episode: 12,
            url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/12/Single/12.pdf")
        },
    ];
    return season1;
}
// Story Season Ends
