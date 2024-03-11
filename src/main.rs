mod author;
mod institution;


use crate::institution::*;

#[tokio::main]
async fn main() {

    let mut inst = Institution::new(0, "icube".to_string());
    inst.add_authors(vec!["Quentin Bramas".to_string(), "Pascal Mérindol".to_string()]).await.expect("TODO: panic message");
    //inst.get_authors_from_institution_ror("https://ror.org/00k4e5n71".to_string()).await.expect("TODO: panic message");
    //inst.get_inst_authors_from_file("/Users/nico/RustroverProjects/ter/src/tmp01.txt".to_string());
    println!("{:?}",inst.authors);
    println!("{:?}",inst.publications.len());
    println!("{:?}", inst.venues);
    inst.set_venue_weight();
    println!("SORTING!!!");
    inst.venues_sort_weight();
    inst.venues.get_mut(&"NETYS".to_string()).unwrap().sort();
    inst.write_to_file("/Users/nico/RustroverProjects/ter/src/tmp02.txt".to_string())
}