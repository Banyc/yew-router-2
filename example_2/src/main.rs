use yew::prelude::*;
use yew_router::prelude::*;

fn sub_routes() -> RouteList {
    RouteList {
        routes: vec![
            Route {
                path: "".to_string(),
                has_sub_routes: false,
            },
            Route {
                path: "*".to_string(),
                has_sub_routes: false,
            },
        ],
    }
}

fn main_routes() -> RouteList {
    RouteList {
        routes: vec![
            Route {
                path: "".to_string(),
                has_sub_routes: false,
            },
            Route {
                path: "sub".to_string(),
                has_sub_routes: true,
            },
            Route {
                path: "*".to_string(),
                has_sub_routes: false,
            },
        ],
    }
}

#[derive(Properties, PartialEq, Debug)]
struct SubProps {
    sub_path: String,
}

#[function_component(Sub)]
fn sub(props: &SubProps) -> Html {
    let sub_routes = sub_routes();
    let navigator = use_navigator().unwrap();

    let onclick_callback = Callback::from(move |_| navigator.push("../"));
    html! {
        <div>
            <h2>{ "Sub Component" }</h2>
            <Switch
                routes={ sub_routes.clone() }
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
        sub_path: _,
        route,
        params: _,
    } = out;

    match route.path.as_str() {
        "" => html! { <h3>{ "Sub Index" }</h3> },
        _ => html! { <h3>{ "Sub 404" }</h3> },
    }
}

#[function_component(Main)]
fn main() -> Html {
    let routes = main_routes();
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
        params: _,
    } = out;

    match route.path.as_str() {
        "" => html! { <h2>{ "Main Index" }</h2> },
        "sub" => {
            html! {
                <Sub
                    sub_path={ sub_path.clone() }
                />
            }
        }
        _ => html! { <h2>{ "Main 404" }</h2> },
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
