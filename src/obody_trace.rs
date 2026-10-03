//! Formal outfit NRO does not install model tracing hooks.
//! Keep these no-op adapters because the outfit selection path calls them.
type P = *mut u8;

pub unsafe fn resolved(result: P, _mode: i32, _unit: P, _state: i32, _probe: bool) -> P {
    result
}

pub unsafe fn resolved_full(
    result: P,
    _mode: i32,
    _person: P,
    _god: P,
    _equipped: P,
    _state: i32,
    _probe: bool,
) -> P {
    result
}
