use std::borrow::Cow;

use serde::Serialize;

use crate::history::{AnyHistory, History, HistoryError, HistoryResult};

pub type NavigationError = HistoryError;
pub type NavigationResult<T> = HistoryResult<T>;

/// The kind of Navigator Provider.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NavigatorKind {
    /// Browser History.
    Browser,
    /// Hash History.
    Hash,
    /// Memory History.
    Memory,
}

/// A struct to navigate between locations.
#[derive(Debug, PartialEq, Clone)]
pub struct Navigator {
    inner: AnyHistory,
    basename: Option<String>,
}

impl Navigator {
    pub(crate) fn new(history: AnyHistory, basename: Option<String>) -> Self {
        Self {
            inner: history,
            basename,
        }
    }

    /// Returns basename of current navigator.
    pub fn basename(&self) -> Option<&str> {
        self.basename.as_deref()
    }

    /// Navigate back 1 page.
    pub fn back(&self) {
        self.go(-1);
    }

    /// Navigate forward 1 page.
    pub fn forward(&self) {
        self.go(1);
    }

    /// Navigate to a specific page with a `delta` relative to current page.
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/API/History/go>
    pub fn go(&self, delta: isize) {
        self.inner.go(delta);
    }

    pub fn push<'a>(&self, route_s: &'a str) {
        self.inner.push(self.prefix_basename(&route_s));
    }

    pub fn replace<'a>(&self, route_s: &'a str) {
        self.inner.replace(self.prefix_basename(&route_s));
    }

    pub fn push_with_state<'a, T>(&self, route_s: &'a str, state: T)
    where
        T: 'static,
    {
        self.inner
            .push_with_state(self.prefix_basename(&route_s), state);
    }

    pub fn replace_with_state<'a, T>(&self, route_s: &'a str, state: T)
    where
        T: 'static,
    {
        self.inner
            .replace_with_state(self.prefix_basename(&route_s), state);
    }

    /// Same as `.push()` but affix the queries to the end of the route.
    pub fn push_with_query<'a, Q>(&self, route_s: &'a str, query: &Q) -> NavigationResult<()>
    where
        Q: Serialize,
    {
        self.inner
            .push_with_query(self.prefix_basename(&route_s), query)
    }

    /// Same as `.replace()` but affix the queries to the end of the route.
    pub fn replace_with_query<'a, Q>(&self, route_s: &'a str, query: &Q) -> NavigationResult<()>
    where
        Q: Serialize,
    {
        self.inner
            .replace_with_query(self.prefix_basename(&route_s), query)
    }

    /// Same as `.push_with_state()` but affix the queries to the end of the route.
    pub fn push_with_query_and_state<'a, Q, T>(
        &self,
        route_s: &'a str,
        query: &Q,
        state: T,
    ) -> NavigationResult<()>
    where
        Q: Serialize,
        T: 'static,
    {
        self.inner
            .push_with_query_and_state(self.prefix_basename(&route_s), query, state)
    }

    /// Same as `.replace_with_state()` but affix the queries to the end of the route.
    pub fn replace_with_query_and_state<'a, Q, T>(
        &self,
        route_s: &'a str,
        query: &Q,
        state: T,
    ) -> NavigationResult<()>
    where
        Q: Serialize,
        T: 'static,
    {
        self.inner
            .replace_with_query_and_state(self.prefix_basename(&route_s), query, state)
    }

    /// Returns the Navigator kind.
    pub fn kind(&self) -> NavigatorKind {
        match &self.inner {
            AnyHistory::Browser(_) => NavigatorKind::Browser,
            AnyHistory::Hash(_) => NavigatorKind::Hash,
            AnyHistory::Memory(_) => NavigatorKind::Memory,
        }
    }

    pub(crate) fn prefix_basename<'a>(&self, route_s: &'a str) -> Cow<'a, str> {
        match self.basename() {
            Some(base) => {
                if route_s.is_empty() && route_s.is_empty() {
                    Cow::from("/")
                } else {
                    Cow::from(format!("{}{}", base, route_s))
                }
            }
            None => route_s.into(),
        }
    }

    // pub(crate) fn strip_basename<'a>(&self, path: Cow<'a, str>) -> Cow<'a, str> {
    //     match self.basename() {
    //         Some(m) => {
    //             let mut path = path
    //                 .strip_prefix(m)
    //                 .map(|m| Cow::from(m.to_owned()))
    //                 .unwrap_or(path);

    //             if !path.starts_with('/') {
    //                 path = format!("/{}", m).into();
    //             }

    //             path
    //         }
    //         None => path,
    //     }
    // }
}
