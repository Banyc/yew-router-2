use yew::prelude::*;
use yew_router::prelude::*;
use log::*;

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

#[derive(Properties, PartialEq)]
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
            <h1>{ "Sub Path" }</h1>
            <Switch
                routes={ props.routes.clone() }
                render={ switch_sub }
                pathname={ props.sub_path.clone() }
            />
            <button onclick={ onclick_callback }>{ "Go Home" }</button>
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
        "" => html! { <h1>{ "Sub Home" }</h1> },
        _ => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn main() -> Html {
    let routes = routes();
    let pathname = use_location().unwrap().path().to_string();

    html! {
        <Switch
            routes={ routes }
            render={ switch_main }
            pathname={ pathname }
        />
    }
}

fn switch_main(out: RouteOutput) -> Html {
    let RouteOutput {
        sub_path,
        route,
        params,
    } = out;

    match route.path.as_str() {
        "" => html! { <h1>{ "Home" }</h1> },
        "sub" => {
            html! {
                <Sub
                    routes={ route.next_routes.unwrap() }
                    sub_path={ sub_path.unwrap().clone() }
                />
            }
        }
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
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
