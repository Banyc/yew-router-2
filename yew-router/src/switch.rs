//! The [`Switch`] Component.

use std::collections::BTreeMap;

use nested_router::{Route, RouteList};
use yew::html::ImplicitClone;
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

use std::fmt;
use std::rc::Rc;

/// Universal callback wrapper.
///
/// An `Rc` wrapper is used to make it cloneable.
pub struct Callback<IN, OUT = ()> {
    /// A callback which can be called multiple times
    pub(crate) cb: Rc<dyn Fn(IN) -> OUT>,
}

impl<IN, OUT, F: Fn(IN) -> OUT + 'static> From<F> for Callback<IN, OUT> {
    fn from(func: F) -> Self {
        Callback { cb: Rc::new(func) }
    }
}

impl<IN, OUT> Clone for Callback<IN, OUT> {
    fn clone(&self) -> Self {
        Self {
            cb: self.cb.clone(),
        }
    }
}

#[allow(clippy::vtable_address_comparisons)]
impl<IN, OUT> PartialEq for Callback<IN, OUT> {
    fn eq(&self, other: &Callback<IN, OUT>) -> bool {
        let (Callback { cb }, Callback { cb: rhs_cb }) = (self, other);
        Rc::ptr_eq(cb, rhs_cb)
    }
}

impl<IN, OUT> fmt::Debug for Callback<IN, OUT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Callback<_>")
    }
}

impl<IN, OUT> Callback<IN, OUT> {
    /// This method calls the callback's function.
    pub fn emit(&self, value: IN) -> OUT {
        (*self.cb)(value)
    }
}

impl<IN> Callback<IN> {
    /// Creates a "no-op" callback which can be used when it is not suitable to use an
    /// `Option<Callback>`.
    pub fn noop() -> Self {
        Self::from(|_| ())
    }
}

impl<IN> Default for Callback<IN> {
    fn default() -> Self {
        Self::noop()
    }
}

impl<IN: 'static, OUT: 'static> Callback<IN, OUT> {
    /// Creates a new callback from another callback and a function
    /// That when emitted will call that function and will emit the original callback
    pub fn reform<F, T>(&self, func: F) -> Callback<T, OUT>
    where
        F: Fn(T) -> IN + 'static,
    {
        let this = self.clone();
        let func = move |input| {
            let output = func(input);
            this.emit(output)
        };
        Callback::from(func)
    }

    /// Creates a new callback from another callback and a function.
    /// When emitted will call the function and, only if it returns `Some(value)`, will emit
    /// `value` to the original callback.
    pub fn filter_reform<F, T>(&self, func: F) -> Callback<T, Option<OUT>>
    where
        F: Fn(T) -> Option<IN> + 'static,
    {
        let this = self.clone();
        let func = move |input| func(input).map(|output| this.emit(output));
        Callback::from(func)
    }
}

impl<IN, OUT> ImplicitClone for Callback<IN, OUT> {}
