pub struct Episode {
    pub name: String, 
    pub season: i32,
    pub episode: i32
    pub url: String
}

pub fn read(ep: Episode, pdf_application: String){
    let read = Command::new(pdf_application).arg(ep.url).output();
}

pub let Season1 = [
    Episode{
        name: String::from("First Match"),
        season: 1,
        episode 1,
        url: String::from("https://github.com/MKProj/Phaktionz/raw/main/Concepts/S1/Episodes/01/Single/01.pdf")
    },
];