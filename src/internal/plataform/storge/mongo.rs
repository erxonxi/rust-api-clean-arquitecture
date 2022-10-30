use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub enum ErrorsMongo {
    NotConected,
}

pub struct MongoClientFactory {
    url: String,
}

impl MongoClientFactory {
    pub async fn new(url: String) -> Result<Client, ErrorsMongo> {
        if let Ok(options) = ClientOptions::parse(url).await {
            if let Ok(client) = Client::with_options(options) {
                Ok(client)
            } else {
                Err(ErrorsMongo::NotConected)
            }
        } else {
            Err(ErrorsMongo::NotConected)
        }
    }
}
