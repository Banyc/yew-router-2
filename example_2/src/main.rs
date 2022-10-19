use yew::prelude::*;
use yew_router::prelude::*;

fn routes() -> RouteList {
    let sub = RouteList {
        routes: vec![
            Route {
                path: "".to_string(),
                next_routes: None,
            },
            Route {
                path: "*".to_string(),
                next_routes: None,
            },
        ],
    };
    RouteList {
        routes: vec![
            Route {
                path: "".to_string(),
                next_routes: None,
            },
            Route {
                path: "sub".to_string(),
                next_routes: Some(sub),
            },
            Route {
                path: "*".to_string(),
                next_routes: None,
            },
        ],
    }
}

#[derive(Properties, PartialEq, Debug)]
struct SubProps {
    routes: RouteList,
    sub_path: String,
}

#[function_component(Sub)]
fn sub(props: &SubProps) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick_callback = Callback::from(move |_| navigator.push("../"));
    html! {
        <div>
            <h2>{ "Sub Component" }</h2>
            <Switch
                routes={ props.routes.clone() }
                render={ switch_sub }
                pathname={ props.sub_path.clone() }
            />
            <button onclick={ onclick_callback }>{ "Go Home" }</button>
            <h2>{ "Sub Component End" }</h2>
        </div>
    }
}

fn switch_sub(out: RouteOutput) -> Html {
    let RouteOutput {
        sub_path,
        route,
        params,
    } = out;

    match route.path.as_str() {
        "" => html! { <h3>{ "Sub Index" }</h3> },
        _ => html! { <h3>{ "Sub 404" }</h3> },
    }
}

#[function_component(Main)]
fn main() -> Html {
    let routes = routes();
    let pathname = use_location().unwrap().path().to_string();

    // remove leading slash if present
    let pathname = if pathname.starts_with("/") {
        pathname[1..].to_string()
    } else {
        pathname
    };

    html! {
        <div>
            <h1>{ "Main Component" }</h1>
            <Switch
                routes={ routes }
                render={ switch_main }
                pathname={ pathname }
            />
            <h1>{ "Main Component End" }</h1>
        </div>
    }
}

fn switch_main(out: RouteOutput) -> Html {
    let RouteOutput {
        sub_path,
        route,
        params,
    } = out;

    match route.path.as_str() {
        "" => html! { <h1>{ "Main Index" }</h1> },
        "sub" => {
            html! {
                <Sub
                    routes={ route.next_routes.unwrap() }
                    sub_path={ sub_path.clone() }
                />
            }
        }
        _ => html! { <h1>{ "Main 404" }</h1> },
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
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
