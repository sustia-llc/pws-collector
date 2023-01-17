use std::env;
use tracing::{debug, error};
use super::*;
#[tokio::test]
async fn test() {
    env::set_var("RUN_MODE", "test");
    logger::setup();
    debug!("Running test: {}", env!("CARGO_PKG_NAME"));

    let test_response = r#"{"observations":[{"stationID":"STATION1","obsTimeUtc":"2022-12-29T16:17:31Z","obsTimeLocal":"2022-12-29 12:17:31","neighborhood":"Zuid Santa Rosa","softwareType":"EasyWeatherV1.6.1","country":"CW","solarRadiation":214.3,"lon":-68.871901,"realtimeFrequency":null,"epoch":1672330651,"lat":12.112722,"uv":2.0,"winddir":135,"humidity":74,"qcStatus":1,"metric":{"temp":29,"heatIndex":33,"dewpt":24,"windChill":29,"windSpeed":8,"windGust":11,"pressure":1016.93,"precipRate":0.00,"precipTotal":0.20,"elev":15}}]}"#;
    let pwsdata_response: PwsdataResponse = serde_json::from_str(test_response).unwrap();
    debug!("pwsdataResponse: {:?}", pwsdata_response);
    assert_eq!(pwsdata_response.observations[0].stationID, "STATION1");
    
    let inserted = services::insert_pwsdata(pwsdata_response).await;
    
    if let Ok(inserted) = inserted {
        debug!("Inserted: {:?}", inserted);
    } else {
        error!("Error: {:?}", inserted.unwrap_err());
        assert!(false);
    }
}
