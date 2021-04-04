use std::path::Path;
use std::process::Command;
use structopt::StructOpt;

use mkproj_lib::phaktionz::*;
//use std::io;
mod profiles;
use profiles::*;
mod info;
use info::*;

#[derive(StructOpt)]
#[structopt(name = "Phaktionz CLI Wiki")]

struct Cli {
    #[structopt(subcommand)]
    cmd: Cmd,
}
// Subcommands Begin
#[derive(StructOpt)]
enum Cmd {
    #[structopt(about = "Displays the various rules in Phaktionz")]
    Rules(Rules),
    #[structopt(about = "Displays the profile of specified Character")]
    Profile(Profile),
    #[structopt(about = "Read the Story Concepts of Phaktionz")]
    Story(Story),
    #[structopt(about = "Gives info about Factions or it's Category")]
    Info(Info),
}
/*#[derive(StructOpt)]
struct Flag{
    #[structopt(short="s")]
    save: Option<bool>,
    #[structopt(short="l")]
    list: Option<bool>,
    #[structopt(short="o", parse(from_os_str))]
    output: Option<std::path::PathBuf>,
}*/

#[derive(StructOpt)]
struct Rules {
    options: String,
}
#[derive(StructOpt)]
struct Profile {
    name: String,
}
#[derive(StructOpt)]
struct Story {
    season: i32,
    episode: i32,
    pdf_application: String,
}
#[derive(StructOpt)]
struct Info {
    /* faction_*/ Category: String,
}
// Subcommands End

// Important functions Begins
fn save(url: String) {
    let path: bool = Path::new("~/Phaktionz-Wiki").is_dir();
    if path == true {
        let wget = Command::new("wget")
            .arg(url)
            .arg("-P")
            .arg("~/Phaktionz-Wiki")
            .output();
    } else {
        let mkdir = Command::new("mkdir").arg("~/Phaktionz-Wiki").output();
        let wget = Command::new("wget")
            .arg(url)
            .arg("-P")
            .arg("~/Phaktionz-Wiki")
            .output();
    }
}

// Important functions Ends

fn main() {
    // Rules Types Begin
    let striker = rules::Card {
        name: String::from("Striker"),
        description: String::from("Can Battle any Opponent's Summons, but not directly"),
    };

    let tech = rules::Card {
        name: String::from("Tech"),
        description: String::from("Can only Battle in the same column, and directly"),
    };
    let summons: [rules::Card; 2] = [striker, tech];
    /////////////////////////////////
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
    //Rules Types End

    // Story Season Begins
    let mut season1 = [
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

    // Story Season Ends

    //CLI Command Begin
    let args = Cli::from_args();
    let cmd = std::env::args().nth(1).expect("no command given");
    let option = std::env::args().nth(2).expect("no option given");

    if cmd == "rules" {
        rules::rules(option, summons, invocations);
    } else if cmd == "profile" {
        prof(option);
    } else if cmd == "story" {
        let season = std::env::args().nth(2).expect("no season given");
        let episode = std::env::args().nth(3).expect("no episode given");

        let s: i32 = season.trim().parse().expect("Please type a valid number!");
        let e: i32 = episode.trim().parse().expect("Please type a valid number!");
        let mut i = 0;
        while i < season1.len() {
            let app = std::env::args().nth(4).expect("no option given");
            if s == season1[i].season && e == season1[i].episode {
                let url = &season1[i].url;
                /*
                println!("Would you like to save?[Y/N]: ");
                let mut s = String::new();
                io::stdin().read_line(&mut s).expect("no answer given");
                */
                //if s == "y" || s == "Y"{
                //save(url.to_string());
                story::read(url.to_string(), app);
                /*} else {
                    read(url.to_string(), app);
                }*/
            } else if s == 1 && e == 0 {
                println!(
                    "| Name: {} |Season: {} | Episode: {} |",
                    season1[i].name, season1[i].season, season1[i].episode
                );
            }
            i += 1;
        }
    } else if cmd == "info" {
        Info(option);
    }
    //CLI Command Ends
}
