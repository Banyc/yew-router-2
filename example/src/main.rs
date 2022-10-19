use yew::prelude::*;
use yew_router::prelude::*;

fn routes() -> RouteList {
    RouteList {
        routes: vec![
            Route {
                path: "".to_string(),
                next_routes: None,
            },
            Route {
                path: "secure".to_string(),
                next_routes: None,
            },
            Route {
                path: "*".to_string(),
                next_routes: None,
            },
        ],
    }
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick_callback = Callback::from(move |_| navigator.push("../"));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button onclick={ onclick_callback }>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(Main)]
fn app() -> Html {
    let routes = routes();
    let pathname = use_location().unwrap().path().to_string();

    html! {
        <BrowserRouter>
            <Switch
                routes={ routes }
                render={ switch }
                pathname={ pathname }
            />
        </BrowserRouter>
    }
}

fn switch(out: RouteOutput) -> Html {
    let RouteOutput {
        sub_path,
        route,
        params,
    } = out;

    match route.path.as_str() {
        "" => html! { <h1>{ "Home" }</h1> },
        "secure" => html! { <Secure /> },
        _ => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
