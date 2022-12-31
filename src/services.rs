use mongodb::error::Error as MongoError;
use mongodb::Collection;
use tracing::{error, debug};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::database;
use crate::models::{Pwsdata, PwsdataResponse};

static mut PWSDATA_COLLECTION: Option<Collection<Pwsdata>> = None;
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub async fn setup() -> Result<(), MongoError> {
    let exchange =
        IS_INITIALIZED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed);
    let can_setup = exchange == Ok(false);

    if !can_setup {
        panic!("Services already initialized");
    }
    let connection = database::get_connection();
    let collection = connection.collection::<Pwsdata>(database::PWSDATA_COLLECTION_STR);
    unsafe {
        PWSDATA_COLLECTION = Some(collection);
    };

    Ok(())
}

pub async fn insert_pwsdata(pws_response: PwsdataResponse) -> Result<(), MongoError> {
    let pws_data = Pwsdata::from(pws_response);
    let result = get_collection().insert_one(pws_data, None).await;
    match result {
        Ok(_) => debug!("Inserted pwsdata"),
        Err(err) => error!("Error inserting pwsdata: {}", err),
    }
    Ok(())
}

fn get_collection() -> &'static Collection<Pwsdata> {
    unsafe {
        PWSDATA_COLLECTION
            .as_ref()
            .expect("Collection not initialized")
    }
}