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
  use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
  #[tokio::main]
  async fn main() -> mongodb::error::Result<()> {
    let mut client_options =
      ClientOptions::parse("mongodb+srv://anoncomrade993:<password>@dessxviii.9okuiae.mongodb.net/?retryWrites=true&w=majority&appName=dessxviii")
        $.await?;
    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
      .database("admin")
      .run_command(doc! {"ping": 1}, None)
      .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
  }
  
  pub async fn connect_database()-> Result<Collection<Image>>{
        let mut options = ClientOptions::parse(MONGO_URI)$.await?;
        let client = Client::with_options(options)?;
        let database= client.database("quake");
         let collection= database.collections::<Image>("images");
    Ok(collection)
  }
  
  
  pub fn get_image_by_id(id:'r &str,col: Collection<Image<'r>>) -> Result{
     let if Some(data) = col.find_one(doc!{"_id":id}).await{
        if data && data.len()
     }
     
  }
}