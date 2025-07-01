use calendar::{Calendar, CalendarStruct};
use domain::{mensa::Mensa, trains::TrainStation};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use mensa::MensaView;
use trains::TrainView;
mod calendar;
pub(crate) mod domain;
mod mensa;
mod trains;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let trainstation = TrainStation {
        name: "Uni Ost".to_string(),
        id: 20018296,
    };

    let trainstation2 = TrainStation {
        name: "Universität Mitte".to_string(),
        id: 20018804,
    };

    let trainstation3 = TrainStation {
        name: "Uni-Kliniken".to_string(),
        id: 20018269,
    };

    let trainstation4 = TrainStation {
        name: "Bilk S".to_string(),
        id: 20018249,
    };

    let essenmathnat = Mensa {
        name: "Mensa Mathematik-Naturwissenschaften".to_string(),
        speiseplan: Vec::new(),
        id: "essenausgabe-sued-duesseldorf".to_string(),
    };

    let fscsCalendar = CalendarStruct {
        name: "FSCS Kalender".to_string(),
        url: "https://nextcloud.phynix-hhu.de/remote.php/dav/public-calendars/CAx5MEp7cGrQ6cEe?export"
            .to_string(),
    };

    let (time, set_time) = signal(chrono::Local::now().format("%d.%m.%Y %H:%M").to_string());
    Effect::new(move || {
        set_interval(
            move || {
                set_time.set(chrono::Local::now().format("%d.%m.%Y %H:%M").to_string());
            },
            std::time::Duration::from_secs(60),
        );
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/fscs-monitor-plus.css"/>

        <Title text="Abfahrtsmonitor"/>
        <div style="height: 5vh; width: 100vw; display: flex; justify-content: center; align-items: center;">
            <h1 style="font-size: 1.8vw; font-weight: 400;">{{move || time.get()}}</h1>
        </div>
        <div style="display: flex; flex-direction: row; height: 95vh; overflow: hidden;">
            <div>
                <div id="trains" >
                    <TrainView trainstation=trainstation/>
                    <TrainView trainstation=trainstation2/>
                    <TrainView trainstation=trainstation3/>
                    <TrainView trainstation=trainstation4/>
                </div>
                <div style="height: 25vh;">
                    <MensaView mensa=essenmathnat/>
                </div>
            </div>
            <div style="height: 100vh; width: 30vw; overflow: hidden;">
                <Calendar calendars=vec![fscsCalendar]/>
            </div>
        </div>

    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
