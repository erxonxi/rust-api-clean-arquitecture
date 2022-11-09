use mongodb::{bson::Document, options::ClientOptions, Client, Collection};

#[derive(Debug)]
pub enum ErrorsMongo {
    NotConected,
}

pub struct MongoClientFactory {}
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

#[async_trait::async_trait]
pub trait MongoRepository {
    async fn get_collection(
        url: String,
        database: String,
        collection: String,
    ) -> Collection<Document> {
        let client = MongoClientFactory::new(url).await.unwrap();

        client
            .database(&database)
            .collection::<Document>(&collection)
    }
}
