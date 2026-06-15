use smithay::{
    backend::input::{
        AbsolutePositionEvent, Axis, AxisSource, ButtonState, Event, InputBackend, InputEvent,
        KeyboardKeyEvent, PointerAxisEvent, PointerButtonEvent,
    },
    input::{
        keyboard::FilterResult,
        pointer::{AxisFrame, ButtonEvent, MotionEvent},
    },
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::SERIAL_COUNTER,
};

use crate::state::MeltState;

impl MeltState {
    pub fn process_input_event<I: InputBackend>(&mut self, event: InputEvent<I>) {
        match event {
            InputEvent::Keyboard { event, .. } => {
                let serial = SERIAL_COUNTER.next_serial();
                let time = Event::time_msec(&event);
                let key_state = event.state();

                self.seat.get_keyboard().unwrap().input::<(), _>(
                    self,
                    event.key_code(),
                    key_state,
                    serial,
                    time,
                    |state, modifiers, handle| {
                        use smithay::backend::input::KeyState;
                        use smithay::input::keyboard::keysyms;

                        // Check Alt key for Mnemonic mode
                        if modifiers.alt && !modifiers.ctrl && !modifiers.shift && !modifiers.logo {
                            if !state.mnemonic_engine.is_active() {
                                tracing::info!("Entering Mnemonic mode");
                                state.mnemonic_engine.activate();
                            }
                        } else {
                            if state.mnemonic_engine.is_active() {
                                tracing::info!("Exiting Mnemonic mode");
                                state.mnemonic_engine.deactivate();
                            }
                        }

                        if key_state == KeyState::Pressed {
                            let keysym = handle.modified_sym();
                            let k: u32 = keysym.into(); // Keysym to u32

                            // 1. Mnemonic Action matching
                            if state.mnemonic_engine.is_active() {
                                // Simple mapping for alphabetic keys
                                let key_char = match k {
                                    keysyms::KEY_a..=keysyms::KEY_z => Some((k - keysyms::KEY_a + b'a' as u32) as u8 as char),
                                    _ => None,
                                };
                                
                                if let Some(c) = key_char {
                                    if let Some(action) = state.mnemonic_engine.match_key(c).map(|a| a.to_string()) {
                                        tracing::info!("Mnemonic triggered: {}", action);
                                        match action.as_str() {
                                            "terminal" => {
                                                let term = state.config.general.terminal.clone();
                                                std::process::Command::new(&term).spawn().ok();
                                            }
                                            "close_window" => {
                                                if let Some(focus) = state.seat.get_keyboard().unwrap().current_focus() {
                                                    state.space.elements().find(|w| {
                                                        w.toplevel().map_or(false, |tl| tl.wl_surface() == &focus)
                                                    }).map(|w| w.toplevel().unwrap().send_close());
                                                }
                                            }
                                            "workspace_1" => { state.workspace_manager.switch_to(0); },
                                            "workspace_2" => { state.workspace_manager.switch_to(1); },
                                            _ => {}
                                        }
                                        return FilterResult::Intercept(());
                                    }
                                }
                            }

                            // 2. Global Keybindings
                            if modifiers.logo {
                                match k {
                                    keysyms::KEY_Return => {
                                        let term = state.config.general.terminal.clone();
                                        std::process::Command::new(&term).spawn().ok();
                                        return FilterResult::Intercept(());
                                    }
                                    keysyms::KEY_q => {
                                        tracing::info!("Super+Q pressed: Closing window");
                                        if let Some(focus) = state.seat.get_keyboard().unwrap().current_focus() {
                                            state.space.elements().find(|w| {
                                                w.toplevel().map_or(false, |tl| tl.wl_surface() == &focus)
                                            }).map(|w| w.toplevel().unwrap().send_close());
                                        }
                                        return FilterResult::Intercept(());
                                    }
                                    keysyms::KEY_1 => {
                                        state.workspace_manager.switch_to(0);
                                        return FilterResult::Intercept(());
                                    }
                                    keysyms::KEY_2 => {
                                        state.workspace_manager.switch_to(1);
                                        return FilterResult::Intercept(());
                                    }
                                    _ => {}
                                }
                            } else if modifiers.alt {
                                match k {
                                    keysyms::KEY_Tab => {
                                        tracing::info!("Alt+Tab pressed");
                                        return FilterResult::Intercept(());
                                    }
                                    keysyms::KEY_F4 => {
                                        tracing::info!("Alt+F4 pressed");
                                        if let Some(focus) = state.seat.get_keyboard().unwrap().current_focus() {
                                            state.space.elements().find(|w| {
                                                w.toplevel().map_or(false, |tl| tl.wl_surface() == &focus)
                                            }).map(|w| w.toplevel().unwrap().send_close());
                                        }
                                        return FilterResult::Intercept(());
                                    }
                                    _ => {}
                                }
                            }
                        }

                        FilterResult::Forward
                    },
                );
            }
            InputEvent::PointerMotion { .. } => {}
            InputEvent::PointerMotionAbsolute { event, .. } => {
                let output = self.space.outputs().next().unwrap();

                let output_geo = self.space.output_geometry(output).unwrap();

                let pos = event.position_transformed(output_geo.size) + output_geo.loc.to_f64();

                let serial = SERIAL_COUNTER.next_serial();

                let pointer = self.seat.get_pointer().unwrap();

                let under = self.surface_under(pos);

                pointer.motion(
                    self,
                    under,
                    &MotionEvent {
                        location: pos,
                        serial,
                        time: event.time_msec(),
                    },
                );
                pointer.frame(self);
            }
            InputEvent::PointerButton { event, .. } => {
                let pointer = self.seat.get_pointer().unwrap();
                let keyboard = self.seat.get_keyboard().unwrap();

                let serial = SERIAL_COUNTER.next_serial();

                let button = event.button_code();

                let button_state = event.state();

                if ButtonState::Pressed == button_state && !pointer.is_grabbed() {
                    let ptr_loc = pointer.current_location();
                    
                    // 1. Check if we clicked on a window decoration (title bar or borders)
                    let mut decoration_click = None;
                    for window in self.space.elements() {
                        if let Some(loc) = self.space.element_location(window) {
                            let outer_geo = crate::decorations::WindowDecorations::outer_geometry(loc, window.geometry().size, &self.decoration_config);
                            let inner_geo = smithay::utils::Rectangle::new(loc, window.geometry().size);
                            let ptr_round = ptr_loc.to_i32_round();
                            
                            if outer_geo.contains(ptr_round) && !inner_geo.contains(ptr_round) {
                                decoration_click = Some((window.clone(), loc));
                                break;
                            }
                        }
                    }

                    if let Some((window, loc)) = decoration_click {
                        // Focus the window we clicked on
                        self.space.raise_element(&window, true);
                        keyboard.set_focus(
                            self,
                            Some(window.toplevel().unwrap().wl_surface().clone()),
                            serial,
                        );
                        self.space.elements().for_each(|w| {
                            w.toplevel().unwrap().send_pending_configure();
                        });

                        // Check if we hit a specific button
                        if let Some(action) = crate::decorations::buttons::hit_test(ptr_loc, loc, window.geometry().size, &self.decoration_config) {
                            match action {
                                crate::decorations::buttons::ButtonAction::Close => window.toplevel().unwrap().send_close(),
                                _ => {} // Maximize and Minimize not fully implemented yet
                            }
                        } else {
                            // Clicked title bar or border, start move grab
                            let start_data = smithay::input::pointer::GrabStartData {
                                focus: None,
                                button,
                                location: ptr_loc,
                            };
                            let grab = crate::grabs::MoveSurfaceGrab {
                                start_data,
                                window: window.clone(),
                                initial_window_location: loc,
                            };
                            pointer.set_grab(self, grab, serial, smithay::input::pointer::Focus::Clear);
                        }
                        
                        pointer.frame(self);
                        return; // Stop processing pointer button
                    }

                    // 2. Check if we clicked on the window content
                    if let Some((window, _loc)) = self
                        .space
                        .element_under(pointer.current_location())
                        .map(|(w, l)| (w.clone(), l))
                    {
                        self.space.raise_element(&window, true);
                        keyboard.set_focus(
                            self,
                            Some(window.toplevel().unwrap().wl_surface().clone()),
                            serial,
                        );
                        self.space.elements().for_each(|window| {
                            window.toplevel().unwrap().send_pending_configure();
                        });
                    } else {
                        self.space.elements().for_each(|window| {
                            window.set_activated(false);
                            window.toplevel().unwrap().send_pending_configure();
                        });
                        keyboard.set_focus(self, Option::<WlSurface>::None, serial);
                    }
                };

                pointer.button(
                    self,
                    &ButtonEvent {
                        button,
                        state: button_state,
                        serial,
                        time: event.time_msec(),
                    },
                );
                pointer.frame(self);
            }
            InputEvent::PointerAxis { event, .. } => {
                let source = event.source();

                let horizontal_amount = event
                    .amount(Axis::Horizontal)
                    .unwrap_or_else(|| event.amount_v120(Axis::Horizontal).unwrap_or(0.0) * 15.0 / 120.);
                let vertical_amount = event
                    .amount(Axis::Vertical)
                    .unwrap_or_else(|| event.amount_v120(Axis::Vertical).unwrap_or(0.0) * 15.0 / 120.);
                let horizontal_amount_discrete = event.amount_v120(Axis::Horizontal);
                let vertical_amount_discrete = event.amount_v120(Axis::Vertical);

                let mut frame = AxisFrame::new(event.time_msec()).source(source);
                if horizontal_amount != 0.0 {
                    frame = frame.value(Axis::Horizontal, horizontal_amount);
                    if let Some(discrete) = horizontal_amount_discrete {
                        frame = frame.v120(Axis::Horizontal, discrete as i32);
                    }
                }
                if vertical_amount != 0.0 {
                    frame = frame.value(Axis::Vertical, vertical_amount);
                    if let Some(discrete) = vertical_amount_discrete {
                        frame = frame.v120(Axis::Vertical, discrete as i32);
                    }
                }

                if source == AxisSource::Finger {
                    if event.amount(Axis::Horizontal) == Some(0.0) {
                        frame = frame.stop(Axis::Horizontal);
                    }
                    if event.amount(Axis::Vertical) == Some(0.0) {
                        frame = frame.stop(Axis::Vertical);
                    }
                }

                let pointer = self.seat.get_pointer().unwrap();
                pointer.axis(self, frame);
                pointer.frame(self);
            }
            _ => {}
        }
    }
}
