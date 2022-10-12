//! Provides routing faculties using the browser history API to build
//! Single Page Applications (SPAs) using [Yew web framework](https://yew.rs).
//!
//! # Usage
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_router::prelude::*;
//!
//! fn routes() -> RouteList {
//!     RouteList {
//!         routes: vec![
//!             Route {
//!                 path: "".to_string(),
//!                 next_routes: None,
//!             },
//!             Route {
//!                 path: "secure".to_string(),
//!                 next_routes: None,
//!             },
//!             Route {
//!                 path: "*".to_string(),
//!                 next_routes: None,
//!             },
//!         ],
//!     }
//! }
//!
//! #[function_component(Secure)]
//! fn secure() -> Html {
//!     let navigator = use_navigator().unwrap();
//!
//!     let onclick_callback = Callback::from(move |_| navigator.push("../home"));
//!     html! {
//!         <div>
//!             <h1>{ "Secure" }</h1>
//!             <button onclick={onclick_callback}>{ "Go Home" }</button>
//!         </div>
//!     }
//! }
//!
//! #[function_component(Main)]
//! fn app() -> Html {
//!     let routes = routes();
//!     let pathname = use_location().unwrap().path().clone();
//!
//!     html! {
//!         <BrowserRouter>
//!             <Switch routes={ routes } render={ switch } pathname={ pathname } />
//!         </BrowserRouter>
//!     }
//! }
//!
//! fn switch(out: RouteOutput) -> Html {
//!     let RouteOutput {
//!         sub_path,
//!         route,
//!         params,
//!     } = out;
//!
//!     match &route.path {
//!         "" => html! { <h1>{ "Home" }</h1> },
//!         "secure" => html! { <Secure /> },
//!         _ => html! { <h1>{ "404" }</h1> },
//!     }
//! }
//! ```
//!
//! # Internals
//!
//! The router registers itself as a context provider and makes location information and navigator
//! available via [`hooks`] or [`RouterScopeExt`](scope_ext::RouterScopeExt).
//!
//! # State
//!
//! The [`Location`](gloo::history::Location) API has a way to access / store state associated with
//! session history. Please consult [`location.state()`](crate::history::Location::state) for
//! detailed usage.

extern crate self as yew_router;

pub mod components;
pub mod hooks;
pub mod navigator;
pub mod router;
pub mod switch;
pub mod utils;

pub use router::{BrowserRouter, HashRouter, Router};
pub use switch::Switch;

pub mod history {
    //! A module that provides universal session history and location information.

    pub use gloo::history::{
        AnyHistory, BrowserHistory, HashHistory, History, HistoryError, HistoryResult, Location,
        MemoryHistory,
    };
}

pub mod prelude {
    //! Prelude module to be imported when working with `yew-router`.
    //!
    //! This module re-exports the frequently used types from the crate.

    pub use crate::components::{Link, Redirect};
    pub use crate::history::Location;
    pub use crate::hooks::*;
    pub use crate::navigator::{NavigationError, NavigationResult, Navigator};
    pub use crate::switch::RouteOutput;
    pub use crate::{BrowserRouter, HashRouter, Router, Switch};
    pub use nested_router::{Route, RouteList};
}
