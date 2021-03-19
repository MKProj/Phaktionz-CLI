pub struct Episode {
    pub name: String, 
    pub season: i32,
    pub episode: i32
    pub url: String
}

pub fn read(ep: Episode, pdf_application: String){
    let read = Command::new(pdf_application).arg(ep.url).output();
}