/// CSS state selectors — port of PHP `Style_States`.

pub const HOVER: &str = "hover";
pub const ACTIVE: &str = "active";
pub const FOCUS: &str = "focus";
pub const FOCUS_VISIBLE: &str = "focus-visible";
pub const CHECKED: &str = "checked";
pub const SELECTED: &str = "e--selected";

const PSEUDO_STATES: &[&str] = &[HOVER, ACTIVE, FOCUS, FOCUS_VISIBLE, CHECKED];
const CLASS_STATES: &[&str] = &[SELECTED];

/// Returns extra states implied by `state`.
/// e.g. `hover` also implies `focus-visible`.
fn additional_states(state: &str) -> &'static [&'static str] {
    match state {
        HOVER => &[FOCUS_VISIBLE],
        _ => &[],
    }
}

fn state_selector(state: &str) -> String {
    if CLASS_STATES.contains(&state) {
        return format!(".{}", state);
    }
    if PSEUDO_STATES.contains(&state) {
        return format!(":{}", state);
    }
    state.to_string()
}

/// Builds `base_selector:hover,base_selector:focus-visible` etc.
pub fn selector_with_state(base: &str, state: &str) -> String {
    let mut all_states = vec![state];
    all_states.extend_from_slice(additional_states(state));

    all_states
        .into_iter()
        .map(|s| format!("{}{}", base, state_selector(s)))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn is_valid_state(state: Option<&str>) -> bool {
    match state {
        None => true,
        Some(s) => PSEUDO_STATES.contains(&s) || CLASS_STATES.contains(&s),
    }
}
