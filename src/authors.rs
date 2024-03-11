// NOT USED ONLY FOR EXAMPLE AND TEST PURPOSE
// OBSOLETE
use quick_xml::Reader;
use quick_xml::events::Event;
use std::error::Error;
use std::ptr::null;
use std::str;

pub(crate) struct Elem{
    authors: Vec<Author>,
    pubs : Vec<Publication>
}

impl Elem{
    pub(crate) fn new() -> Self{
       Elem{
           authors: Vec::new(),
           pubs: Vec::new(),
       }
    }
    pub(crate) fn search_author(id: &str, data: Elem) -> Author {
        for auth in data.authors {
            if auth.pid.eq(id) { return auth; }
        }
        return Author::new();
    }
    pub(crate) fn search_author_by_name(name: &str) -> Result<Vec<Author>, Box<dyn Error>> {
        let url = "https://dblp.uni-trier.de/search/author?xauthor=";
        let xml_url = format!("{url}{name}");
        let xml = reqwest::blocking::get(xml_url)?.text()?;

        let mut reader = Reader::from_str(&*xml);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut authors = Vec::new();
        let mut current_author = Author::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(ref e) if e.name() == b"author" => {
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"name" => current_author.name = attr.unescape_and_decode_value(&reader)?,
                            b"urlpt" => current_author.urlpt = attr.unescape_and_decode_value(&reader)?,
                            b"pid" => current_author.pid = attr.unescape_and_decode_value(&reader)?,
                            _ => {}
                        }
                    }
                }
                Event::Text(e) => {
                    current_author.name = e.unescape_and_decode(&reader)?;
                }
                Event::End(ref e) if e.name() == b"author" => {
                    authors.push(current_author.clone());
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(authors)
    }
}




#[derive(Debug,Clone)]
enum PublicationType {
    Journal,
    Conf,
}
#[derive(Debug,Clone)]
struct Publication {
    authors: Vec<Author>,
    title: String,
    year: u32,
    name: String,
    pub_type: PublicationType
}

impl Publication{
    fn new(id: String) -> Result<Self, Box<dyn Error>>{
        let url = format!("https://dblp.uni-trier.de/rec/{id}.xml");
        let xml = reqwest::blocking::get(url)?.text()?;
        let mut reader = Reader::from_str(&*xml);
        reader.trim_text(true);
        let mut buf = Vec::new();
    }
}
#[derive(Debug,Clone)]
pub struct Author {
    name: String,
    urlpt: String,
    pid: String,
    publications: Vec<Publication>,
    coauthor: Vec<Author>,
}
impl Author {
    fn new() -> Self {
        Author {
            name: String::new(),
            urlpt: String::new(),
            pid: String::new(),
            publications: Vec::new(),
            coauthor: Vec::new(),
        }
    }
}
pub fn search_author(name: &str) -> Result<Vec<Author>, Box<dyn Error>> {
    let url = "https://dblp.uni-trier.de/search/author?xauthor=";
    let xml_url = format!("{url}{name}");
    let xml = reqwest::blocking::get(xml_url)?.text()?;

    let authors = parse_xml(&xml)?;
    Ok(authors)
}

pub fn parse_xml(xml: &str) -> Result<Vec<Author>, Box<dyn Error>> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut authors = Vec::new();
    let mut current_author = Author::new();

    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) if e.name() == b"author" => {
                for attr in e.attributes() {
                    let attr = attr?;
                    match attr.key {
                        b"name" => current_author.name = attr.unescape_and_decode_value(&reader)?,
                        b"urlpt" => current_author.urlpt = attr.unescape_and_decode_value(&reader)?,
                        b"pid" => current_author.pid = attr.unescape_and_decode_value(&reader)?,
                        _ => {}
                    }
                }
            }
            Event::Text(e) => {
                current_author.name = e.unescape_and_decode(&reader)?;
            }
            Event::End(ref e) if e.name() == b"author" => {
                authors.push(current_author.clone());
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(authors)
}