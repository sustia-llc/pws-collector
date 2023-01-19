use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::error::Error as MongoError;
use mongodb::Collection;
use mongodb::results::InsertOneResult;
use crate::database::{PWSDATA_COLLECTION_STR, CONNECTION};
use crate::models::{Pwsdata, PwsdataResponse};

lazy_static! {
    static ref PWSDATA_COLLECTION : AsyncOnce<Collection<Pwsdata>> = AsyncOnce::new(async {
        let connection = CONNECTION.get().await;
        let collection = connection.collection::<Pwsdata>(PWSDATA_COLLECTION_STR);
        collection
    });
}

pub async fn insert_pwsdata(pws_response: PwsdataResponse) -> Result<InsertOneResult, MongoError> {
    let pws_data = Pwsdata::from(pws_response);
    PWSDATA_COLLECTION.get().await.insert_one(pws_data, None).await
}
