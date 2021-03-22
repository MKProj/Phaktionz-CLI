use std::process::Command;
use structopt::StructOpt;
use std::path::Path;

use std::io;
mod rules;
use rules::*;
mod profiles;
use profiles::*;
mod story;
use story::*;

#[derive(StructOpt)]
#[structopt(name = "Phaktionz CLI Wiki")]

struct Cli {
    #[structopt(subcommand)]
    cmd: Cmd,
    
}
// Subcommands Begin
#[derive(StructOpt)]
enum Cmd {
    #[structopt(
        about = "Displays the various rules in Phaktionz\n\t$ phaktionz rules list # Gives  List of available options"
    )]
    Rules(Rules),
    #[structopt(about = "Displays the profile of specified Character")]
    Profile(Profile),
    #[structopt(about = "Read the Story Concepts of Phaktionz")]
    Story(Story)
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
struct Story{
    season: i32, 
    episode: i32, 
    pdf_application: String
}
// Subcommands End

// Important functions Begins 


// Important functions Ends 

fn main() {
    // Rules Types Begin
    let striker = Card {
        name: String::from("Striker"),
        description: String::from("Can Battle any Opponent's Summons, but not directly"),
    };

    let tech = Card {
        name: String::from("Tech"),
        description: String::from("Can only Battle in the same column, and directly"),
    };
    let summons: [Card; 2] = [striker, tech];
    /////////////////////////////////
    let regular = Card {
        name: String::from("Regular"),
        description: String::from("This type of invocation may only be cast on your turn"),
    };
    let counter = Card {
        name: String::from("Counter"),
        description: String::from("This type of invocation may be cast on any turn"),
    };

    let weapon = Card {
        name: String::from("Weapon"),
        description: String::from(
            "This type of invocation attaches itself to a Summon on the Battlefield",
        ),
    };

    let realm = Card {
        name: String::from("Realm"),
        description: String::from(
            "This type of invocation remains on the battlefield, with continuous abilities",
        ),
    };
    let invocations: [Card; 4] = [regular, counter, weapon, realm];
    //Rules Types End

    // Story Season Begins 
    let mut season1 : [Episode; 1] = [
    Episode{
        name: String::from("First Match"),
        season: 1,
        episode: 1,
        url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/01/Single/01.pdf")
        },
    ];



    // Story Season Ends 




    //CLI Command Begin
    let args = Cli::from_args();
    let cmd = std::env::args().nth(1).expect("no command given");
    let option = std::env::args().nth(2).expect("no option given");



    if cmd == "rules" {
       rules(option, summons, invocations);
    } else if cmd == "profile" {
       prof(option);
    }
    else if cmd == "story"{
        let season = std::env::args().nth(2).expect("no season given");
        let episode = std::env::args().nth(3).expect("no episode given");
        

        let s :i32 = season.trim().parse().expect("Please type a valid number!");
        let e :i32 = episode.trim().parse().expect("Please type a valid number!");
        let mut i = 0;
        while i < season1.len(){
        let app = std::env::args().nth(4).expect("no option given");
          if s == season1[i].season && e == season1[i].episode{
            let url  = &season1[i].url;
            println!("Would you like to save?[Y/N]: ");
            let mut save = String::new(); 
            io::stdin().read_line(&mut save).expect("no answer given");
            if save == "y" || save == "Y"{

            }
            read(url.to_string(), app);
          }  
          i += 1;
        }
        
    }
    //CLI Command Ends
    // Flag Options

}
