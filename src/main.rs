use std::process::Command;
use structopt::StructOpt;
mod rules;
use rules::*;
mod profiles;
use profiles::*;

#[derive(StructOpt)]
#[structopt(name = "Phaktionz CLI Wiki")]

struct Cli {
    #[structopt(subcommand)]
    cmd: Cmd,
    // flag: Option<Flag>
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
}
#[derive(StructOpt)]
struct Rules {
    options: String,
}
#[derive(StructOpt)]
struct Profile {
    name: String,
}
// Subcommands End

/* Flags Begin
#[derive(StructOpt)]
enum Flag{
    #[structopt(short = "l")]
    List(List),
}

#[derive(StructOpt)]
struct List{
    list: bool
}
//Flags End */

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

    //CLI Command Begin
    let args = Cli::from_args();
    let cmd = std::env::args().nth(1).expect("no command given");
    let option = std::env::args().nth(2).expect("no option given");

    if cmd == "rules" {
        rules(option, summons, invocations);
    } else if cmd == "profile" {
        prof(option);
    }
    //CLI Command Ends
}
