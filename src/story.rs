use std::process::Command;
pub struct Episode {
    pub name: String, 
    pub season: i32,
    pub episode: i32,
    pub url: String
}

pub fn read(url: String, pdf_application: String){
    let read = Command::new(pdf_application).arg(url).output();
}
