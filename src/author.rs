use std::collections::HashMap;
use std::error::Error;
use std::string::String;
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::*;
use std::fmt::format;
use std::ops::{DerefMut};
use std::time::Duration;
use regex::Regex;
use tokio::sync::Semaphore;
use tokio::time::sleep;



#[derive(Debug,Clone,Deserialize,Serialize)]
 pub struct Author{
    pub(crate) full_name: String, // ID
    last_institution:u64,
    pub(crate) publications: Vec<String>,
    pub(crate) stat: Stats,
    weight: u64,
    pub(crate) working_oa_url: String,
}
#[derive(Debug,Clone,Deserialize,Serialize)]
pub(crate) struct Stats {
    impact_factor: f64,
    pub(crate) h_index: u64,
    i10_index: u64,
}
#[derive(Debug,Deserialize)]
pub struct Publication{
    pub(crate) title:String, // ID
    pub(crate) doi:String,
    pub(crate) authors:Vec<String>,
    weight: f32,
}

#[derive(Debug,Deserialize)]
pub(crate) enum VenuesType {
    Journal,
    Conference
}

#[derive(Debug,Deserialize)]
pub struct Venues{
    venue_type: VenuesType,
    title: String,
    pub(crate) years: HashMap<u64,VenuesYear>,
    pub(crate) weight: u64,
}
#[derive(Debug,Deserialize)]
pub struct VenuesYear{
    pub(crate) weight: u64,
    pub(crate) publications: Vec<String>,
}

impl VenuesYear {
    pub(crate) fn new() -> VenuesYear{
        VenuesYear{
            publications: Vec::new(),
            weight: 0
        }
    }

    pub(crate) fn weight(&mut self,authors: Vec<Author>){
        let nb_authors = authors.len() as u64;
        for author in authors {
            self.weight += (author.weight / nb_authors) ;
        }
    }
}
impl Venues{
    pub(crate) fn new(title: String,venue_type: VenuesType) -> Venues{
        Venues{
            venue_type,
            title,
            years: HashMap::new(),
            weight: 0
        }
    }

    pub(crate) fn add_year(&mut self,year:u64,name:String){
        if !self.years.contains_key(&year) {
            self.years.insert(year,VenuesYear::new());
        }
        self.years.get_mut(&year).unwrap().publications.push(name);
    }
    pub(crate) fn year_exist(&self,id:u64)->bool{
        return self.years.contains_key(&id);
    }

    pub(crate) fn to_string(&self) -> String {
        format!("NAME: {0} \n {:?} {:?}", &self.title, &self.years )
    }

    pub(crate) fn add_weight(&mut self){
        let len= self.years.len() as u64;
        for years in self.years.iter_mut() {
            self.weight += years.1.weight/len;
        }
    }

    pub(crate) fn sort(&mut self){
        let mut venues_year: Vec<(&u64, &VenuesYear)> = self.years.iter().collect();
        venues_year.sort_by_key(|(_,year)|year.weight);
        println!("Sorting: {}",self.title);
        for (year, weight) in venues_year.iter().rev() {
            println!("year: {}, weight: {}", year, weight.weight);
        }
    }
}

impl Author {
    pub(crate) fn new(full_name: String, last_institution:u64,impact_factor:f64,h_index:u64,i10_index:u64, url: String,weight:u64) -> Self{
        Author{
            full_name,
            last_institution,
            publications: Vec::new(),
            stat: Stats { impact_factor, h_index, i10_index },
            working_oa_url: url,
            weight,
        }
    }
    pub(crate) fn add_publication(&mut self, id:String){
        self.publications.push(id);
    }

    pub(crate) async fn in_dblp(&mut self, resp:Response) -> Result<bool,Box<dyn Error>> {
        let name = self.full_name.clone();
        if resp.status() != StatusCode::OK { return Ok(false); }
        let xml = resp.text().await?;
        let xml_encoded_name = encode(name);
        let regex = Regex::new(format!(r#">{xml_encoded_name}</author>"#).as_str()).unwrap();
        if let Some(captures) = regex.captures(&*xml){
            return Ok(true);
        }


        Ok(false)
    }
}

impl Publication {
    pub(crate) fn new(title:String, doi: String) -> Self{
        Publication{
            title,
            doi,
            authors: Vec::new(),
            weight: 0.0
        }
    }

    pub(crate) fn add_author(&mut self, id:String){
        self.authors.push(id);
    }
}


pub(crate) fn extract_booktitle_url(xml: &str) -> Option<String> {
    let re = Regex::new(r#"<booktitle>(.*?)</booktitle>"#).unwrap();
    if let Some(captures) = re.captures(xml) {
        if let Some(url) = captures.get(1) {
            return Some(url.as_str().to_string());
        }
    }
    None
}

pub(crate) fn extract_journals_url(xml: &str) -> Option<String> {
    let re = Regex::new(r#"<journal>(.*?)</journal>"#).unwrap();
    if let Some(captures) = re.captures(xml) {
        if let Some(url) = captures.get(1) {
            return Some(url.as_str().to_string());
        }
    }
    None
}

pub(crate) fn extract_year_url(xml: &str) -> Option<String> {
    let re = Regex::new(r#"<year>(.*?)</year>"#).unwrap();
    if let Some(captures) = re.captures(xml) {
        if let Some(url) = captures.get(1) {
            return Some(url.as_str().to_string());
        }
    }
    None
}

pub(crate) fn encode(text: String) -> String{
    let mut result = String::new();

    for c in text.chars() {
        match c {
            'é' => result.push_str("&#233;"),
            'è' => result.push_str("&#232;"),
            'ü' => result.push_str("&#252;"),
            'ä' => result.push_str("&#228;"),
            'ö' => result.push_str("&#246;"),
            _ => result.push(c),
        }
    }

    result
}