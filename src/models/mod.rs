use mongodb::{Collection,Client,options::ClientOptions};
use std::env;
#[derive(Comparable,Debug)]
struct Image<'r>{
  water_mark:Option<'r &str>,
  key :Option<'r &str>
}
const MONGO_URI:&str = "";

impl<'r> Image<'r>{
  pub fn new(water_mark:Option<'r &str>,key:Option<'r &str>) -> Self{
     Image{
       water_mark,
       key
     }
  }
  
  pub async fn connect_database()-> Result<Collection<Image>>{
        let mut options = ClientOptions::parse(MONGO_URI).await?;
        let client = Client::with_options(options)?;
        let database= client.database("quake");
         let collection= database.collections::<Image>("images");
    Ok(collection)
  }
  
  
  pub fn get_image_by_id(id:'r &str,col: Collection<Image<'r>>) -> {
     let 
  }
}