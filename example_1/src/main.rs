use yew::prelude::*;
use yew_router::prelude::*;

fn routes() -> RouteList {
    RouteList {
        routes: vec![
            Route {
                path: "".to_string(),
                has_sub_routes: false,
            },
            Route {
                path: "secure".to_string(),
                has_sub_routes: false,
            },
            Route {
                path: "*".to_string(),
                has_sub_routes: false,
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
fn main() -> Html {
    let routes = routes();
    let pathname = use_location().unwrap().path()[1..].to_string();

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
        sub_path: _,
        route,
        params: _,
    } = out;

    match route.path.as_str() {
        "" => html! { <h1>{ "Home" }</h1> },
        "secure" => html! { <Secure /> },
        _ => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Main />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
