use mongodb::{bson::doc,Client, ClientOptions, Collection, error::Result, options::ServerApi, options::ServerApiVersion};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
   watermark:String,
   data:String,
   key:String,
   dimensions:(u32,u32)
}


impl Image{
pub fn new(watermark:String,data:String, dimensions:(usize,usize)) -> Self{
   Image { watermark,data, dimensions}
}

pub async fn connect_to_db() -> Result<Collection<Self>> {
    let connection_uri = "your_mongodb_connection_string";
    
    // Parse the connection string into an options struct
    let mut client_options = ClientOptions::parse(connection_uri).await?;
    
    // Set the server API version to V1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    
    // Create a new client
    let client = Client::with_options(client_options)?;
    
    // Get a handle to the database and the collection
    let db = client.database("Lab");
    let collection = db.collection::<Image>("Desiccator");
    
    Ok(collection)
}


pub async fn insert_image(&self,collection:Collection<Image>) -> Result<()> {
    let image_watermark = self.watermark;
    collection.insert_one(self, None).await?;
    Ok(())
}

pub async fn find_image(watermark:String,key:String, collection:Collection<Self>) -> Result<bool> {
    let filter = doc!{"watermark":watermark,"key":key}
    let watermark = collection.find_one(filter,None).await?
     Ok(watermark.is_some())
}
