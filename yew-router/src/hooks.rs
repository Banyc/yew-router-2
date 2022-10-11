//! Hooks to access router state and navigate between pages.

use yew::prelude::*;

use crate::history::*;
use crate::navigator::Navigator;
use crate::router::{LocationContext, NavigatorContext};

/// A hook to access the [`Navigator`].
#[hook]
pub fn use_navigator() -> Option<Navigator> {
    use_context::<NavigatorContext>().map(|m| m.navigator())
}

/// A hook to access the current [`Location`].
#[hook]
pub fn use_location() -> Option<Location> {
    Some(use_context::<LocationContext>()?.location())
}
