pub struct Category {
    pub name: String,
    pub desc: String,
}

pub fn Info(name: String) {
    let cat = [
    Category{ //Origins
    name: String::from("Origin"),
    desc: String::from("\nOrigins are the founding Factions that were created by the three Origins; Helios, Prestige and King.
\nThe following Origin factions are:
\n\t-Sorcerers (Prestige)\n\t-Mythicals (Helios)\n\t-Kingdoms (King)")},
    Category{//Modernas
        name: String::from("Modernas"),
        desc: String::from("\nModernas are the present-time Factions that were built, and evolved overtime from Origin Factions
\nThe following Modernas factions are:
\n\t- Oceanics\n\t- Alchemists\n\t- Descendants\n\t- Warriors\n\t- Dragons\n\t- Phasmas\n\t- Titans")
},
    Category{//Delphic
        name: String::from("Delphic"),
        desc: String::from("\nDelphic factions were the factions erased or never recorded in the Phaktionz History, 
as much as a myth to the Origin factions, and unknown to the Modernas.
Delphics are the corrupted, forbidden factions brought to ruin by Lyzor.
\nThe following Delphic Factions are: 
\n\t- Frosters\n\t- Cryptics\n\t- Cyborgs\n\t- Probers\n\t- The Unknowns")
    }

];

    let mut i = 0;
    while i < cat.len() {
        if name == cat[i].name {
            println!("{}", cat[i].desc);
        } else if name == "list" {
            println!("\t*{}", cat[i].name);
        }
        i += 1;
    }
}
