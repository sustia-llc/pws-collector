use async_once::AsyncOnce;
use lazy_static::lazy_static;
use tracing::debug;
use mongodb::options::IndexOptions;
use mongodb::{Database, IndexModel};

use mongodb::{
    bson::doc,
    options::{
        CreateCollectionOptions, Sphere2DIndexVersion, TimeseriesGranularity, TimeseriesOptions,
    },
};

use crate::models::Pwsdata;
use crate::settings::SETTINGS;

pub const PWSDATA_COLLECTION_STR: &str = "pwsdata";

lazy_static! {
    pub static ref CONNECTION: AsyncOnce<Database> = AsyncOnce::new(async {
        debug!("Connecting to MongoDB:");
        let db_uri = SETTINGS.database.uri.as_str();
        let db_name = SETTINGS.database.name.as_str();

        let connection = mongodb::Client::with_uri_str(db_uri)
            .await
            .expect("Failed to initialize MongoDB connection")
            .database(db_name);
        connection
            .run_command(doc! { "ping": 1 }, None)
            .await
            .expect("failed to ping");

        let collections = connection.list_collection_names(None).await.expect("listing the collections should succeed");
        if !collections.contains(&PWSDATA_COLLECTION_STR.to_string()) {
            debug!("Creating collection: {}", &PWSDATA_COLLECTION_STR);
            // create timeseries collection if not exists
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
                .expect("creating the 2dsphere index should succeed");
    
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
                .expect("creating the compound index should succeed");
        }
        debug!("MongoDB connected:");
        connection
    });
}

