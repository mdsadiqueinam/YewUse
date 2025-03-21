//! Tests for YewUse
//!
//! This module provides integration tests for all YewUse hooks.

#[cfg(test)]
mod state {
    mod use_counter_test;
}

#[cfg(test)]
mod browser {
    mod use_local_storage_test;
}

#[cfg(test)]
mod sensors {
    mod use_mouse_position_test;
}

#[cfg(test)]
mod ui {
    mod use_click_outside_test;
}

#[cfg(test)]
mod utils {
    mod use_interval_test;
}