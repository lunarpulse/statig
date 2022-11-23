#![allow(unused)]

use statig::prelude::*;
use statig::StateMachine;
use statig::StateOrSuperstate;
use std::io::Write;

#[derive(Default)]
pub struct Blinky;

#[derive(Debug)]
pub enum Event {
    TimerElapsed,
    ButtonPressed,
}

/// The `state_machine` procedural macro generates the `State` and `Superstate`
/// enums by parsing the function signatures with a `state`, `superstate` or
/// `action` attribute. It also implements the `statig::State` and
/// `statig::Superstate` traits.
#[state_machine(
    // This sets the initial state to `led_on`.
    initial = "State::led_on()",
    // Derive the Debug trait on the `State` enum.
    state(derive(Debug)),
    // Derive the Debug trait on the `Superstate` enum.
    superstate(derive(Debug)),
    // Set the `on_transition` callback.
    on_transition = "Self::on_transition",
    // Set the `on_dispatch` callback.
    on_dispatch = "Self::on_dispatch"
)]
impl Blinky {
    /// The `#[state]` attibute marks this as a state handler.  By default the
    /// `event` argument will map to the event handler by the state machine.
    /// Every state must return a `Response<State>`.
    #[state(superstate = "blinking")]
    fn led_on(event: &Event) -> Response<State> {
        match event {
            // When we receive a `TimerElapsed` event we transition to the `led_off` state.
            Event::TimerElapsed => Transition(State::led_off()),
            // Other events are deferred to the superstate, in this case `blinking`.
            _ => Super,
        }
    }

    #[state(superstate = "blinking")]
    fn led_off(event: &Event) -> Response<State> {
        match event {
            Event::TimerElapsed => Transition(State::led_on()),
            _ => Super,
        }
    }

    #[superstate]
    fn blinking(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::not_blinking()),
            _ => Super,
        }
    }

    #[state]
    fn not_blinking(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::led_on()),
            // Altough this state has no superstate, we can still defer the event which
            // will cause the event to be handled by an implicit `top` state handled all
            // events.
            _ => Super,
        }
    }
}

impl Blinky {
    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{:?}` to `{:?}`", source, target);
    }

    fn on_dispatch(&mut self, state: StateOrSuperstate<Self>, event: &Event) {
        println!("dipatching `{:?}` to `{:?}`", event, state);
    }
}

fn main() {
    let mut state_machine = Blinky::default().state_machine().init();

    state_machine.handle(&Event::TimerElapsed);
    state_machine.handle(&Event::ButtonPressed);
    state_machine.handle(&Event::TimerElapsed);
    state_machine.handle(&Event::ButtonPressed);
}
