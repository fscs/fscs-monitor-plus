use crate::app::domain::{self, trains::TrainStation};
use leptos::{logging::log, prelude::*, task::spawn_local};

#[component]
pub fn TrainView(trainstation: TrainStation) -> impl IntoView {
    let (trains, set_trains) = signal(Vec::new());
    spawn_local(async move {
        let data = domain::trains::get_trains(trainstation.id, 200).await;
        match data {
            Ok(data) => {
                set_trains.set(data);
            }
            Err(e) => {
                log!("Error fetching trains: {:?}", e);
            }
        }
    });
    view! {
        <div style="width: 50%; height: 50%; overflow:hidden;">
        <h1>{trainstation.name}</h1>
        {move || {
            view! {
                <table class="center" style="padding-left:30px; padding-right:30px;">
                    {trains.get().iter().map(|train| {
                        if train.delay.is_none() {
                            let delay = format!("{}m", train.minutes_till_departure.clone());
                            view! {
                                <tr>
                                    <th style="color:#0f0; text-align:left;">{train.line.clone()}</th>
                                    <th style="color:#0f0; text-align:left; line-height:1; max-width:25vw">{train.direction.clone()}</th>
                                    <th style="color:#0f0; text-align:right;">{delay}</th>
                                </tr>
                            }
                        }else {
                            let delay = format!("(+{}) {}m", train.delay.unwrap(), train.minutes_till_departure.clone());
                            view! {
                                <tr>
                                    <th style="color:#f00; text-align:left;">{train.line.clone()}</th>
                                    <th style="color:#f00; text-align:left; line-height:1; max-width:25vw">{train.direction.clone()}</th>
                                    <th style="color:#f00; text-align:right;">{delay}</th>
                                </tr>
                            }
                        }
                    }).collect::<Vec<_>>()}
                </table>
            }
        }
    }
    </div>
    }
}
