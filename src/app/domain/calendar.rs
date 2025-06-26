use crate::app::CalendarStruct;
use icalendar::{Calendar, CalendarComponent, Component};
use leptos::server;
use leptos::{logging::log, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub title: String,
    pub start: String,
    pub description: Option<String>,
}

#[server]
pub async fn get_calendars(
    calendars: Vec<CalendarStruct>,
) -> Result<Vec<Vec<Event>>, ServerFnError> {
    let mut all_calendars_with_events = Vec::new();
    for calendar in calendars {
        let url = calendar.url;
        let data = reqwest::get(url).await?;
        let text = data.text().await?;
        let parsed_calendar: Calendar = text.parse().unwrap();

        let mut events = Vec::new();
        for component in &parsed_calendar.components {
            if let CalendarComponent::Event(event) = component {
                let title = event.get_summary().unwrap_or_default();
                let start = event.get_start().unwrap();
                let description = event.get_description();
                if start.date_naive() > chrono::Utc::now().date_naive() {
                    //check if the event has a time component
                    events.push(Event {
                        title: title.to_string(),
                        start: start.date_naive().format("%d.%m.%Y").to_string(),
                        description: description.map(|d| d.to_string()),
                    });
                }
            }
        }

        all_calendars_with_events.push(events);
    }
    Ok(all_calendars_with_events)
}
