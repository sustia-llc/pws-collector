use bson::DateTime;
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub r#type: String,
    pub coordinates: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub station: String,
    pub location: Location,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub temp: f32,
    pub heatIndex: f32,
    pub dewpt: f32,
    pub windChill: f32,
    pub windSpeed: f32,
    pub windGust: f32,
    pub pressure: f32,
    pub precipRate: f32,
    pub precipTotal: f32,
    pub elev: f32,
}
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwsdataResponse {
    pub observations: Vec<PwsdataRaw>,
}
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwsdataRaw {
    pub stationID: String,
    pub obsTimeUtc: String,
    pub obsTimeLocal: String,
    neighborhood: String,
    softwareType: String,
    country: String,
    solarRadiation: f32,
    epoch: f32,
    uv: f32,
    humidity: f32,
    winddir: f32,
    qcStatus: f32,
    lon: f32,
    lat: f32,
    metric: Metric,
}
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pwsdata {
    pub _id: String,
    pub metadata: Metadata,
    pub ts: DateTime,
    neighborhood: String,
    softwareType: String,
    country: String,
    solarRadiation: f32,
    epoch: f32,
    uv: f32,
    humidity: f32,
    winddir: f32,
    qcStatus: f32,
    metric: Metric,
}

impl From<PwsdataResponse> for Pwsdata {
    fn from(pws_response: PwsdataResponse) -> Self {
        let data = &pws_response.observations[0];
        Self {
            _id: nanoid::nanoid!(),
            ts: bson::DateTime::parse_rfc3339_str(data.obsTimeUtc.as_str()).unwrap(),
            metadata: Metadata {
                station: data.stationID.to_string(),
                location: Location {
                    r#type: "Point".to_string(),
                    coordinates: vec![
                        data.lon.to_string().parse::<f32>().unwrap(),
                        data.lat.to_string().parse::<f32>().unwrap(),
                    ],
                },
            },
            neighborhood: data.neighborhood.to_string(),
            softwareType: data.softwareType.to_string(),
            country: data.country.to_string(),
            solarRadiation: data.solarRadiation.to_string().parse::<f32>().unwrap(),
            epoch: data.epoch.to_string().parse::<f32>().unwrap(),
            uv: data.uv.to_string().parse::<f32>().unwrap(),
            humidity: data.humidity.to_string().parse::<f32>().unwrap(),
            winddir: data.winddir.to_string().parse::<f32>().unwrap(),
            qcStatus: data.qcStatus.to_string().parse::<f32>().unwrap(),
            metric: Metric {
                temp: data.metric.temp.to_string().parse::<f32>().unwrap(),
                heatIndex: data.metric.heatIndex
                    .to_string()
                    .parse::<f32>()
                    .unwrap(),
                dewpt: data.metric.dewpt.to_string().parse::<f32>().unwrap(),
                windChill: data.metric.windChill
                    .to_string()
                    .parse::<f32>()
                    .unwrap(),
                windSpeed: data.metric.windSpeed
                    .to_string()
                    .parse::<f32>()
                    .unwrap(),
                windGust: data.metric.windGust
                    .to_string()
                    .parse::<f32>()
                    .unwrap(),
                pressure: data.metric.pressure
                    .to_string()
                    .parse::<f32>()
                    .unwrap(),
                precipRate: data.metric.precipRate
                    .to_string()
                    .parse::<f32>()
                    .unwrap(),
                precipTotal: data.metric.precipTotal
                    .to_string()
                    .parse::<f32>()
                    .unwrap(),
                elev: data.metric.elev.to_string().parse::<f32>().unwrap(),
            },
        }
    }
}