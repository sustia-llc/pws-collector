use mongodb::error::Error as MongoError;
use mongodb::options::IndexOptions;
use mongodb::{Database as MongoDatabase, IndexModel};

use mongodb::{
    bson::doc,
    options::{
        CreateCollectionOptions, Sphere2DIndexVersion, TimeseriesGranularity,
        TimeseriesOptions,
    },
    Client,
};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::models::Pwsdata;
use crate::settings::SETTINGS;

static mut CONNECTION: Option<MongoDatabase> = None;
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub const PWSDATA_COLLECTION_STR: &str = "pwsdata";

pub async fn setup() -> Result<(), MongoError> {
    let exchange =
        IS_INITIALIZED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed);
    let can_setup = exchange == Ok(false);

    if !can_setup {
        panic!("Database already initialized");
    }

    let db_uri = SETTINGS.database.uri.as_str();
    let db_name = SETTINGS.database.name.as_str();
    let connection = Client::with_uri_str(db_uri)
        .await
        .expect("failed to connect")
        .database(db_name);
    connection
        .run_command(doc! { "ping": 1 }, None)
        .await
        .expect("failed to ping");
    // create timeseries collection if not exists
    let collections = connection.list_collection_names(None).await?;
    if !collections.contains(&PWSDATA_COLLECTION_STR.to_string()) {
        connection
            .create_collection(
                PWSDATA_COLLECTION_STR,
                CreateCollectionOptions::builder()
                    .timeseries(
                        TimeseriesOptions::builder()
                            .time_field("ts".to_string())
                            .meta_field(Some("metadata".to_string()))
                            .granularity(Some(TimeseriesGranularity::Minutes))
                            .build(),
                    )
                    .build(),
            )
            .await
            .expect("creating the collection should succeed");

        // create a 2dsphere index on the location field
        let options = IndexOptions::builder()
            .sphere_2d_index_version(Sphere2DIndexVersion::V2)
            .build();
        let model = IndexModel::builder()
            .keys(doc! { "metadata.location": 1 })
            .options(options)
            .build();
        connection
            .collection::<Pwsdata>(PWSDATA_COLLECTION_STR)
            .create_index(model, None)
            .await
            .expect("creating the index should succeed");

        // create a compound index on the ts and metadata.station fields
        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "metadata.station": 1, "ts": -1 })
            .options(options)
            .build();
        connection
            .collection::<Pwsdata>(PWSDATA_COLLECTION_STR)
            .create_index(model, None)
            .await
            .expect("creating the index should succeed");
    }

    unsafe {
        CONNECTION = Some(connection);
    };

    Ok(())
}

pub fn get_connection() -> &'static MongoDatabase {
    unsafe {
        CONNECTION
            .as_ref()
            .expect("Database connection not initialized")
    }
}
