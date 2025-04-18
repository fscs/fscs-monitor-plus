use domain::trains::TrainStation;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use trains::TrainView;
pub(crate) mod domain;
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

    view! {
        <Stylesheet id="leptos" href="/pkg/fscs-monitor-plus.css"/>

        <Title text="Abfahrtsmonitor"/>
        <div style="display: flex; flex-direction: row; flex-wrap: wrap; height: 100vh;">
            <TrainView trainstation=trainstation/>
            <TrainView trainstation=trainstation2/>
            <TrainView trainstation=trainstation3/>
            <TrainView trainstation=trainstation4/>
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
