//! The [`Switch`] Component.

use std::collections::BTreeMap;

use nested_router::{Route, RouteList};
use yew::prelude::*;

/// Props for [`Switch`]
#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps {
    /// Callback which returns [`Html`] to be rendered for the current route.
    pub render: Callback<RouteOutput, Html>,
    pub routes: RouteList,
    pub pathname: String,
}

/// A Switch that dispatches route among variants of a [`RouteList`].
///
/// When a route can't be matched, including when the path is matched but the deserialization fails,
/// it looks for the route with `not_found` attribute.
/// If such a route is provided, it redirects to the specified route.
/// Otherwise `html! {}` is rendered and a message is logged to console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let nested_router::RouteOutput {
        sub_path,
        route,
        params,
    } = match props.routes.route(&props.pathname) {
        Ok(output) => output,
        Err(e) => match e {
            nested_router::Error::InvalidPath => {
                tracing::warn!("Invalid path: {}", props.pathname);
                return Html::default();
            }
            nested_router::Error::NotFound => {
                tracing::warn!("no route matched");
                return Html::default();
            }
        },
    };

    props.render.emit(RouteOutput {
        sub_path,
        route: route.clone(),
        params,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct RouteOutput {
    pub sub_path: Option<String>,
    pub route: Route,
    pub params: BTreeMap<String, String>,
}
