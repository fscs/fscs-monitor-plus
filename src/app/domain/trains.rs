use chrono::NaiveDateTime;
use std::str::FromStr;

use leptos::{prelude::ServerFnError, server};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

#[derive(Clone, Debug)]
pub struct TrainStation {
    pub name: String,
    pub id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Train {
    pub line: String,
    pub direction: String,
    pub minutes_till_departure: u64,
    pub delay: Option<u64>,
}

#[server]
pub async fn get_trains(id: u64, limit: u64) -> Result<Vec<Train>, ServerFnError> {
    let url = format!("https://app.vrr.de/vrrstd/XML_DM_REQUEST?outputFormat=JSON&commonMacro=dm&type_dm=any&name_dm={}&language=de&useRealtime=1&lsShowTrainsExplicit=1&mode=direct&typeInfo_dm=stopID&limit={}", id, limit);

    let data = reqwest::get(url).await?;

    let json: Value = from_str(&data.text().await?)?;

    let departure_list = json["departureList"].clone();

    let mut trains = Vec::new();
    let now = chrono::Local::now();
    let offset = now.offset().local_minus_utc();
    let now = NaiveDateTime::from_timestamp(now.timestamp() + (offset as i64), 0);

    for departure in departure_list.as_array().unwrap() {
        let serving_line = &departure["servingLine"];

        let departure_time = &departure["dateTime"];

        let real_time = &departure["realDateTime"];

        let departure_time = NaiveDateTime::parse_from_str(
            &format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:00",
                i32::from_str(departure_time["year"].as_str().unwrap()).unwrap(),
                i32::from_str(departure_time["month"].as_str().unwrap()).unwrap(),
                i32::from_str(departure_time["day"].as_str().unwrap()).unwrap(),
                i32::from_str(departure_time["hour"].as_str().unwrap()).unwrap(),
                i32::from_str(departure_time["minute"].as_str().unwrap()).unwrap(),
            ),
            "%Y-%m-%dT%H:%M:%S",
        )
        .unwrap();

        let minutes_till_departure = departure_time - now;

        let mut delay = if real_time.is_null() {
            None
        } else {
            let delay = NaiveDateTime::parse_from_str(
                &format!(
                    "{:04}-{:02}-{:02}T{:02}:{:02}:00",
                    i32::from_str(real_time["year"].as_str().unwrap()).unwrap(),
                    i32::from_str(real_time["month"].as_str().unwrap()).unwrap(),
                    i32::from_str(real_time["day"].as_str().unwrap()).unwrap(),
                    i32::from_str(real_time["hour"].as_str().unwrap()).unwrap(),
                    i32::from_str(real_time["minute"].as_str().unwrap()).unwrap(),
                ),
                "%Y-%m-%dT%H:%M:%S",
            )
            .unwrap()
                - departure_time;
            Some(delay.num_minutes() as u64)
        };

        if delay.is_some() && delay < Some(5) {
            delay = None;
        }

        let mut replaced = serving_line["number"].as_str().unwrap().to_string();
        if let Some(start) = serving_line["number"].as_str().unwrap().find('(') {
            if let Some(end) = serving_line["number"].as_str().unwrap()[start..].find(')') {
                let end = start + end;
                replaced = format!(
                    "{}{}",
                    &serving_line["number"].as_str().unwrap()[..start],
                    &serving_line["number"].as_str().unwrap()[end + 1..]
                );
            }
        }

        let train = Train {
            line: replaced,
            direction: serving_line["direction"].as_str().unwrap().to_string(),
            minutes_till_departure: minutes_till_departure.num_minutes() as u64,
            delay,
        };
        if train.minutes_till_departure > 5 && train.minutes_till_departure < u64::MAX - 100 {
            trains.push(train);
        }
    }

    trains.sort_by(|a, b| {
        if a.minutes_till_departure == b.minutes_till_departure {
            return a.line.cmp(&b.line);
        }
        a.minutes_till_departure.cmp(&b.minutes_till_departure)
    });

    trains.truncate(10);

    Ok(trains)
}
