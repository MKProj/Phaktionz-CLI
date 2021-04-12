// Phaktionz CommnadLine Interface Wiki
// An MKProject Project
// MIT Licence  
// Current Version: Aristotle  [v0.5.3]
use mkproj_lib::phaktionz::*;
use structopt::StructOpt;
//Important Arrays 
mod cat_info;
use cat_info::*;
mod ep;
use ep::*;
mod ct;
use ct::*;
mod prof_char;
use prof_char::*;

#[derive(StructOpt)]
#[structopt(name = "Phaktionz CLI Wiki")]

struct Cli {
    #[structopt(subcommand)]
    _cmd: Cmd,
}

// Subcommands Begin
#[derive(StructOpt)]
enum Cmd {
    #[structopt(about = "List options of Subcommands")]
    List(List),
    #[structopt(about = "Update Phaktionz CLI")]
    Update,
    #[structopt(about = "Initialize Phaktionz TCG Book")]
    Init,
    #[structopt(about = "Serve Phaktionz TCG Book Locally")]
    Serve,
    #[structopt(about = "Displays the various rules in Phaktionz")]
    Rules(Rules),
    #[structopt(about = "Displays the profile of specified Character")]
    Profile(Profile),
    #[cfg(all(unix))]
    #[structopt(about = "Read the Story Concepts of Phaktionz")]
    Story(Story),
    #[structopt(about = "Gives info about Factions or it's Category")]
    Info(Info),
}

#[derive(StructOpt)]
struct Rules {
    _options: String,
}
#[derive(StructOpt)]
struct Profile {
    _name: String,
}
#[derive(StructOpt)]
struct Story {
    _season: i32,
    _episode: i32,
    _pdf_application: String,
}
#[derive(StructOpt)]
struct Info {
    _category: String,
}
#[derive(StructOpt)]
struct List {
    _subcommand: String,
}
// Subcommands End

fn main() {
    //CLI Command Begin
    let _args = Cli::from_args();
    let cmd = std::env::args().nth(1).expect("no command given");
    //
    let cat_info = cat_info(); // Category Info
    let season1 = ep_all(); //ep
    let summons = card_summon(); //ct
    let invocations = card_invocation(); //ct
    let prof_char = prof_char(); //prof_char

    if cmd == "rules" {
        let option = std::env::args().nth(2).expect("no option given");
        rules::rules(option, summons, invocations);
    } else if cmd == "profile" {
        let option = std::env::args().nth(2).expect("no option given");
        profiles::prof(option, prof_char);
    } else if cmd == "story" {
        let season = std::env::args().nth(2).expect("no season given");
        let episode = std::env::args().nth(3).expect("no episode given");

        let s: i32 = season.trim().parse().expect("Please type a valid number!");
        let e: i32 = episode.trim().parse().expect("Please type a valid number!");
        for i in 0..season1.len() {
            let app = std::env::args().nth(4).expect("no option given");
            if s == season1[i].season && e == season1[i].episode {
                let url = &season1[i].url;
                story::read(url.to_string(), app);
            } else if s == 1 && e == 0 {
                println!(
                    "| Name: {} |Season: {} | Episode: {} |",
                    season1[i].name, season1[i].season, season1[i].episode
                );
            }
        }
    } else if cmd == "info" {
        let option = std::env::args().nth(2).expect("no option given");
        info::Info(option, cat_info);
    } else if cmd == "update" {
        update();
    }
    //Book Commands
    else if cmd == "init" {
        book::init();
    } else if cmd == "serve" {
        book::serve();
    }
    //List Commands
    else if cmd == "list" {
        let option = std::env::args().nth(2).expect("no option given");
        if option == "rules" {
            rules::list();
        } else if option == "profile" {
            profiles::prof(String::from("list"), prof_char);
        } else if option == "story" {
            for i in 0..season1.len() {
                println!(
                    "| Name: {} |Season: {} | Episode: {} |",
                    season1[i].name, season1[i].season, season1[i].episode
                );
            }
        } else if option == "info" {
            info::Info(String::from("list"), cat_info);
        }
    }
    //CLI Command Ends
}
