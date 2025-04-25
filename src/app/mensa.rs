use std::{thread, time::Duration};

use leptos::{logging::log, prelude::*, task::spawn_local};

use crate::app::domain;

use super::domain::mensa::Mensa;
#[component]
pub fn MensaView(mensa: Mensa) -> impl IntoView {
    let (mensa_signal, set_mensa) = signal(Mensa::default());
    Effect::new(move || {
        let mensa = mensa.clone();
        set_interval(
            move || {
                let mensa = mensa.clone();
                spawn_local(async move {
                    log!("Fetching mensa data...");
                    let data = domain::mensa::get_essen(mensa.clone().id.to_string()).await;
                    match data {
                        Ok(data) => {
                            set_mensa.set(Mensa {
                                id: mensa.clone().id,
                                name: mensa.clone().name,
                                speiseplan: data,
                            });
                        }
                        Err(e) => {
                            log!("Error fetching mensa: {:?}", e);
                        }
                    }
                });
            },
            Duration::from_secs(5),
        );
    });
    view! {
        <div style="width: 100%; height: 100%; overflow:hidden;">
        {move || {
            view! {
                <h1>{mensa_signal.get().name}</h1>
                <div style="display: flex; flex-direction: row; flex-wrap: wrap; height: inherit;">
                    {mensa_signal.get().speiseplan.iter().map(|essen| {
                            view! {
                                    <div style=format!("background-image: url({}); background-size: cover; background-size: 110%; background-position: center; width: {}%; height: 300px",{essen.thumbnail.as_str()}, (1.0/mensa_signal.get().speiseplan.len() as f64 *100.0))>
                                        <div style="background-color: rgba(0, 0, 0, 0.5); width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                                            <h2 style="color: white; text-align: center;">{essen.name.clone()}</h2>
                                        </div>
                                    </div>
                            }

                    }).collect::<Vec<_>>()}
                </div>
            }
        }
    }
    </div>
    }
}
