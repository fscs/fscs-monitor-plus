use std::{thread, time::Duration};

use crate::app::domain::{self, trains::TrainStation};
use leptos::{logging::log, prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalendarStruct {
    pub name: String,
    pub url: String,
}

#[component]
pub fn Calendar(calendars: Vec<CalendarStruct>) -> impl IntoView {
    let (calendar, set_calendar) = signal(Vec::new());
    Effect::new(move || {
        let calendars = calendars.clone();
        set_interval(
            move || {
                let calendars = calendars.clone();
                spawn_local(async move {
                    log!("Fetching calendars data...");
                    let data = domain::calendar::get_calendars(calendars.clone()).await;
                    match data {
                        Ok(data) => {
                            set_calendar.set(data);
                        }
                        Err(e) => {
                            log!("Error fetching calendars: {:?}", e);
                        }
                    }
                });
            },
            Duration::from_secs(5),
        );
    });
    view! {
        <div style="width: 100%; height: 100%; overflow:hidden;">
        {{move || {
            calendar.get().iter().enumerate().map(|(i, events)| {
            view! {
                <div class="calendar" style="width: 100%; height: 100%; overflow: auto;">
                    <ul>
                        {events.iter().map(|event| {
                            view! {
                                <div>
                                    <strong style="color: #0f0; font-size: 1.8vw; font-weight:400;">{event.title.clone()}</strong><br/>
                                    <span style="font-size: 1.2vw">{event.start.clone()}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                </ul>
                </div>
            }
            }).collect::<Vec<_>>()
                }
         }}
        </div>
    }
}
