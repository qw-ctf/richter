// Copyright © 2018 Cormac O'Brien
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
    str::FromStr,
    string::ToString,
};

use crate::common::{
    console::{CmdRegistry, Console},
    parse,
};

use failure::Error;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use winit::{
    dpi::PhysicalPosition,
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode as Key, WindowEvent,
    },
};

const ACTION_COUNT: usize = 19;

lazy_static! {
    static ref INPUT_MAP: HashMap<&'static str, BindInput> = {
        let mut map = HashMap::new();

        map.insert(",", BindInput::Key(Key::Comma));
        map.insert(".", BindInput::Key(Key::Period));
        map.insert("/", BindInput::Key(Key::Slash));
        map.insert("+", BindInput::Key(Key::Plus));
        map.insert("-", BindInput::Key(Key::Minus));
        map.insert("=", BindInput::Key(Key::Equals));
        map.insert("0", BindInput::Key(Key::Key0));
        map.insert("1", BindInput::Key(Key::Key1));
        map.insert("2", BindInput::Key(Key::Key2));
        map.insert("3", BindInput::Key(Key::Key3));
        map.insert("4", BindInput::Key(Key::Key4));
        map.insert("5", BindInput::Key(Key::Key5));
        map.insert("6", BindInput::Key(Key::Key6));
        map.insert("7", BindInput::Key(Key::Key7));
        map.insert("8", BindInput::Key(Key::Key8));
        map.insert("9", BindInput::Key(Key::Key9));
        map.insert("A", BindInput::Key(Key::A));
        map.insert("ALT", BindInput::Key(Key::LAlt));
        map.insert("B", BindInput::Key(Key::B));
        map.insert("BACKSPACE", BindInput::Key(Key::Back));
        map.insert("C", BindInput::Key(Key::C));
        map.insert("CTRL", BindInput::Key(Key::LControl));
        map.insert("D", BindInput::Key(Key::D));
        map.insert("DEL", BindInput::Key(Key::Delete));
        map.insert("DOWNARROW", BindInput::Key(Key::Down));
        map.insert("E", BindInput::Key(Key::E));
        map.insert("END", BindInput::Key(Key::End));
        map.insert("ENTER", BindInput::Key(Key::Return));
        map.insert("ESCAPE", BindInput::Key(Key::Escape));
        map.insert("F", BindInput::Key(Key::F));
        map.insert("F1", BindInput::Key(Key::F1));
        map.insert("F10", BindInput::Key(Key::F10));
        map.insert("F11", BindInput::Key(Key::F11));
        map.insert("F12", BindInput::Key(Key::F12));
        map.insert("F2", BindInput::Key(Key::F2));
        map.insert("F3", BindInput::Key(Key::F3));
        map.insert("F4", BindInput::Key(Key::F4));
        map.insert("F5", BindInput::Key(Key::F5));
        map.insert("F6", BindInput::Key(Key::F6));
        map.insert("F7", BindInput::Key(Key::F7));
        map.insert("F8", BindInput::Key(Key::F8));
        map.insert("F9", BindInput::Key(Key::F9));
        map.insert("G", BindInput::Key(Key::G));
        map.insert("H", BindInput::Key(Key::H));
        map.insert("HOME", BindInput::Key(Key::Home));
        map.insert("I", BindInput::Key(Key::I));
        map.insert("INS", BindInput::Key(Key::Insert));
        map.insert("J", BindInput::Key(Key::J));
        map.insert("K", BindInput::Key(Key::K));
        map.insert("L", BindInput::Key(Key::L));
        map.insert("LEFTARROW", BindInput::Key(Key::Left));
        map.insert("M", BindInput::Key(Key::M));
        map.insert("MOUSE1", BindInput::MouseButton(MouseButton::Left));
        map.insert("MOUSE2", BindInput::MouseButton(MouseButton::Right));
        map.insert("MOUSE3", BindInput::MouseButton(MouseButton::Middle));
        map.insert("MWHEELDOWN", BindInput::MouseWheel(MouseWheel::Down));
        map.insert("MWHEELUP", BindInput::MouseWheel(MouseWheel::Up));
        map.insert("N", BindInput::Key(Key::N));
        map.insert("O", BindInput::Key(Key::O));
        map.insert("P", BindInput::Key(Key::P));
        map.insert("PAUSE", BindInput::Key(Key::Pause));
        map.insert("PGDN", BindInput::Key(Key::PageDown));
        map.insert("PGUP", BindInput::Key(Key::PageUp));
        map.insert("Q", BindInput::Key(Key::Q));
        map.insert("R", BindInput::Key(Key::R));
        map.insert("RIGHTARROW", BindInput::Key(Key::Right));
        map.insert("S", BindInput::Key(Key::S));
        map.insert("SEMICOLON", BindInput::Key(Key::Semicolon));
        map.insert("SHIFT", BindInput::Key(Key::LShift));
        map.insert("SPACE", BindInput::Key(Key::Space));
        map.insert("T", BindInput::Key(Key::T));
        map.insert("TAB", BindInput::Key(Key::Tab));
        map.insert("U", BindInput::Key(Key::U));
        map.insert("UPARROW", BindInput::Key(Key::Up));
        map.insert("V", BindInput::Key(Key::V));
        map.insert("W", BindInput::Key(Key::W));
        map.insert("X", BindInput::Key(Key::X));
        map.insert("Y", BindInput::Key(Key::Y));
        map.insert("Z", BindInput::Key(Key::Z));
        map.insert("[", BindInput::Key(Key::LBracket));
        map.insert("\\", BindInput::Key(Key::Backslash));
        map.insert("]", BindInput::Key(Key::RBracket));
        map.insert("`", BindInput::Key(Key::Grave));

        map
    };
}

/// A unique identifier for an in-game action.
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
pub enum Action {
    /// Move forward.
    Forward = 0,

    /// Move backward.
    Back = 1,

    /// Strafe left.
    MoveLeft = 2,

    /// Strafe right.
    MoveRight = 3,

    /// Move up (when swimming).
    MoveUp = 4,

    /// Move down (when swimming).
    MoveDown = 5,

    /// Look up.
    LookUp = 6,

    /// Look down.
    LookDown = 7,

    /// Look left.
    Left = 8,

    /// Look right.
    Right = 9,

    /// Change move speed (walk/run).
    Speed = 10,

    /// Jump.
    Jump = 11,

    /// Interpret `Left`/`Right` like `MoveLeft`/`MoveRight`.
    Strafe = 12,

    /// Attack with the current weapon.
    Attack = 13,

    /// Interact with an object (not used).
    Use = 14,

    /// Interpret `Forward`/`Back` like `LookUp`/`LookDown`.
    KLook = 15,

    /// Interpret upward/downward vertical mouse movements like `LookUp`/`LookDown`.
    MLook = 16,

    /// If in single-player, show the current level stats. If in multiplayer, show the scoreboard.
    ShowScores = 17,

    /// Show the team scoreboard.
    ShowTeamScores = 18,
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s.to_lowercase().as_str() {
            "forward" => Action::Forward,
            "back" => Action::Back,
            "moveleft" => Action::MoveLeft,
            "moveright" => Action::MoveRight,
            "moveup" => Action::MoveUp,
            "movedown" => Action::MoveDown,
            "lookup" => Action::LookUp,
            "lookdown" => Action::LookDown,
            "left" => Action::Left,
            "right" => Action::Right,
            "speed" => Action::Speed,
            "jump" => Action::Jump,
            "strafe" => Action::Strafe,
            "attack" => Action::Attack,
            "use" => Action::Use,
            "klook" => Action::KLook,
            "mlook" => Action::MLook,
            "showscores" => Action::ShowScores,
            "showteamscores" => Action::ShowTeamScores,
            _ => bail!("Invalid action name: {}", s),
        };

        Ok(action)
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        String::from(match *self {
            Action::Forward => "forward",
            Action::Back => "back",
            Action::MoveLeft => "moveleft",
            Action::MoveRight => "moveright",
            Action::MoveUp => "moveup",
            Action::MoveDown => "movedown",
            Action::LookUp => "lookup",
            Action::LookDown => "lookdown",
            Action::Left => "left",
            Action::Right => "right",
            Action::Speed => "speed",
            Action::Jump => "jump",
            Action::Strafe => "strafe",
            Action::Attack => "attack",
            Action::Use => "use",
            Action::KLook => "klook",
            Action::MLook => "mlook",
            Action::ShowScores => "showscores",
            Action::ShowTeamScores => "showteamscores",
        })
    }
}

// for game input, we only care about the direction the mouse wheel moved, not how far it went in
// one event
/// A movement of the mouse wheel up or down.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MouseWheel {
    Up,
    Down,
}

// TODO: this currently doesn't handle NaN and treats 0.0 as negative which is probably not optimal
impl ::std::convert::From<MouseScrollDelta> for MouseWheel {
    fn from(src: MouseScrollDelta) -> MouseWheel {
        match src {
            MouseScrollDelta::LineDelta(_, y) => {
                if y > 0.0 {
                    MouseWheel::Up
                } else {
                    MouseWheel::Down
                }
            }

            MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                if y > 0.0 {
                    MouseWheel::Up
                } else {
                    MouseWheel::Down
                }
            }
        }
    }
}

/// A physical input that can be bound to a command.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BindInput {
    /// A key pressed on the keyboard.
    Key(Key),

    /// A button pressed on the mouse.
    MouseButton(MouseButton),

    /// A direction scrolled on the mouse wheel.
    MouseWheel(MouseWheel),
}

impl ::std::convert::From<Key> for BindInput {
    fn from(src: Key) -> BindInput {
        BindInput::Key(src)
    }
}

impl ::std::convert::From<MouseButton> for BindInput {
    fn from(src: MouseButton) -> BindInput {
        BindInput::MouseButton(src)
    }
}

impl ::std::convert::From<MouseWheel> for BindInput {
    fn from(src: MouseWheel) -> BindInput {
        BindInput::MouseWheel(src)
    }
}

impl ::std::convert::From<MouseScrollDelta> for BindInput {
    fn from(src: MouseScrollDelta) -> BindInput {
        BindInput::MouseWheel(MouseWheel::from(src))
    }
}

impl FromStr for BindInput {
    type Err = Error;

    fn from_str(src: &str) -> Result<BindInput, Error> {
        let upper = src.to_uppercase();

        if let Some(value) = INPUT_MAP.get(upper.as_str()) {
            return Ok(value.clone());
        }

        bail!("\"{}\" isn't a valid key", src);
    }
}

impl ToString for BindInput {
    fn to_string(&self) -> String {
        // this could be a binary search but it's unlikely to affect performance much
        for (i, input) in INPUT_MAP.iter() {
            if self == input {
                return i.to_string();
            }
        }

        String::new()
    }
}

/// An operation to perform when a `BindInput` is received.
#[derive(Clone, Debug)]
pub enum BindTarget {
    /// An action to set/unset.
    Action {
        // + is true, - is false
        // so "+forward" maps to trigger: true, action: Action::Forward
        trigger: ElementState,
        action: Action,
    },

    /// Text to push to the console execution buffer.
    ConsoleInput { text: String },
}

impl FromStr for BindTarget {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse::action(s) {
            // first, check if this is an action
            Ok((_, (trigger, action_str))) => {
                let action = match Action::from_str(&action_str) {
                    Ok(a) => a,
                    _ => return Ok(BindTarget::ConsoleInput { text: s.to_owned() }),
                };

                Ok(BindTarget::Action { trigger, action })
            }

            // if the parse fails, assume it's a cvar/cmd and return the text
            _ => Ok(BindTarget::ConsoleInput { text: s.to_owned() }),
        }
    }
}

impl ToString for BindTarget {
    fn to_string(&self) -> String {
        match *self {
            BindTarget::Action { trigger, action } => {
                String::new()
                    + match trigger {
                        ElementState::Pressed => "+",
                        ElementState::Released => "-",
                    }
                    + &action.to_string()
            }

            BindTarget::ConsoleInput { ref text } => format!("\"{}\"", text.to_owned()),
        }
    }
}

#[derive(Clone)]
pub struct GameInput {
    console: Rc<RefCell<Console>>,
    bindings: Rc<RefCell<HashMap<BindInput, BindTarget>>>,
    action_states: Rc<RefCell<[bool; ACTION_COUNT]>>,
    mouse_delta: (f64, f64),
    impulse: Rc<Cell<u8>>,
}

impl GameInput {
    pub fn new(console: Rc<RefCell<Console>>) -> GameInput {
        GameInput {
            console,
            bindings: Rc::new(RefCell::new(HashMap::new())),
            action_states: Rc::new(RefCell::new([false; ACTION_COUNT])),
            mouse_delta: (0.0, 0.0),
            impulse: Rc::new(Cell::new(0)),
        }
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        self.mouse_delta
    }

    pub fn impulse(&self) -> u8 {
        self.impulse.get()
    }

    /// Bind the default controls.
    pub fn bind_defaults(&mut self) {
        self.bind(Key::W, BindTarget::from_str("+forward").unwrap());
        self.bind(Key::A, BindTarget::from_str("+moveleft").unwrap());
        self.bind(Key::S, BindTarget::from_str("+back").unwrap());
        self.bind(Key::D, BindTarget::from_str("+moveright").unwrap());
        self.bind(Key::Space, BindTarget::from_str("+jump").unwrap());
        self.bind(Key::Up, BindTarget::from_str("+lookup").unwrap());
        self.bind(Key::Left, BindTarget::from_str("+left").unwrap());
        self.bind(Key::Down, BindTarget::from_str("+lookdown").unwrap());
        self.bind(Key::Right, BindTarget::from_str("+right").unwrap());
        self.bind(Key::LControl, BindTarget::from_str("+attack").unwrap());
        self.bind(Key::E, BindTarget::from_str("+use").unwrap());
        self.bind(Key::Grave, BindTarget::from_str("toggleconsole").unwrap());
        self.bind(Key::Key1, BindTarget::from_str("impulse 1").unwrap());
        self.bind(Key::Key2, BindTarget::from_str("impulse 2").unwrap());
        self.bind(Key::Key3, BindTarget::from_str("impulse 3").unwrap());
        self.bind(Key::Key4, BindTarget::from_str("impulse 4").unwrap());
        self.bind(Key::Key5, BindTarget::from_str("impulse 5").unwrap());
        self.bind(Key::Key6, BindTarget::from_str("impulse 6").unwrap());
        self.bind(Key::Key7, BindTarget::from_str("impulse 7").unwrap());
        self.bind(Key::Key8, BindTarget::from_str("impulse 8").unwrap());
        self.bind(Key::Key9, BindTarget::from_str("impulse 9").unwrap());
    }

    /// Bind a `BindInput` to a `BindTarget`.
    pub fn bind<I, T>(&mut self, input: I, target: T) -> Option<BindTarget>
    where
        I: Into<BindInput>,
        T: Into<BindTarget>,
    {
        self.bindings
            .borrow_mut()
            .insert(input.into(), target.into())
    }

    /// Return the `BindTarget` that `input` is bound to, or `None` if `input` is not present.
    pub fn binding<I>(&self, input: I) -> Option<BindTarget>
    where
        I: Into<BindInput>,
    {
        self.bindings.borrow().get(&input.into()).map(|t| t.clone())
    }

    pub fn handle_event<T>(&mut self, outer_event: Event<T>) {
        let (input, state): (BindInput, _) = match outer_event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => (key.into(), state),

                WindowEvent::MouseInput { state, button, .. } => (button.into(), state),
                WindowEvent::MouseWheel { delta, .. } => (delta.into(), ElementState::Pressed),
                _ => return,
            },

            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    self.mouse_delta.0 += delta.0;
                    self.mouse_delta.1 += delta.1;
                    return;
                }

                _ => return,
            },

            _ => return,
        };

        self.handle_input(input, state);
    }

    pub fn handle_input<I>(&mut self, input: I, state: ElementState)
    where
        I: Into<BindInput>,
    {
        let bind_input = input.into();

        // debug!("handle input {:?}: {:?}", &bind_input, state);
        if let Some(target) = self.bindings.borrow().get(&bind_input) {
            match *target {
                BindTarget::Action { trigger, action } => {
                    self.action_states.borrow_mut()[action as usize] = state == trigger;
                    debug!(
                        "{}{}",
                        if state == trigger { '+' } else { '-' },
                        action.to_string()
                    );
                }

                BindTarget::ConsoleInput { ref text } => {
                    if state == ElementState::Pressed {
                        self.console.borrow_mut().stuff_text(text);
                    }
                }
            }
        }
    }

    pub fn action_state(&self, action: Action) -> bool {
        self.action_states.borrow()[action as usize]
    }

    // TODO: roll actions into a loop
    pub fn register_cmds(&self, cmds: &mut CmdRegistry) {
        let states = [("+", true), ("-", false)];
        for action in Action::iter() {
            for (state_str, state_bool) in states.iter().cloned() {
                let action_states = self.action_states.clone();
                let cmd_name = format!("{}{}", state_str, action.to_string());
                cmds.insert_or_replace(
                    &cmd_name,
                    Box::new(move |_| {
                        action_states.borrow_mut()[action as usize] = state_bool;
                        String::new()
                    }),
                )
                .unwrap();
            }
        }

        // "bind"
        let bindings = self.bindings.clone();
        cmds.insert_or_replace(
            "bind",
            Box::new(move |args| {
                match args.len() {
                    // bind (key)
                    // queries what (key) is bound to, if anything
                    1 => match BindInput::from_str(args[0]) {
                        Ok(i) => match bindings.borrow().get(&i) {
                            Some(t) => format!("\"{}\" = \"{}\"", i.to_string(), t.to_string()),
                            None => format!("\"{}\" is not bound", i.to_string()),
                        },

                        Err(_) => format!("\"{}\" isn't a valid key", args[0]),
                    },

                    // bind (key) [command]
                    2 => match BindInput::from_str(args[0]) {
                        Ok(input) => match BindTarget::from_str(args[1]) {
                            Ok(target) => {
                                bindings.borrow_mut().insert(input, target);
                                debug!("Bound {:?} to {:?}", input, args[1]);
                                String::new()
                            }
                            Err(_) => {
                                format!("\"{}\" isn't a valid bind target", args[1])
                            }
                        },

                        Err(_) => format!("\"{}\" isn't a valid key", args[0]),
                    },

                    _ => "bind [key] (command): attach a command to a key".to_owned(),
                }
            }),
        )
        .unwrap();

        // "unbindall"
        let bindings = self.bindings.clone();
        cmds.insert_or_replace(
            "unbindall",
            Box::new(move |args| match args.len() {
                0 => {
                    let _ = bindings.replace(HashMap::new());
                    String::new()
                }
                _ => "unbindall: delete all keybindings".to_owned(),
            }),
        )
        .unwrap();

        // "impulse"
        let impulse = self.impulse.clone();
        cmds.insert_or_replace(
            "impulse",
            Box::new(move |args| {
                println!("args: {}", args.len());
                match args.len() {
                    1 => match u8::from_str(args[0]) {
                        Ok(i) => {
                            impulse.set(i);
                            String::new()
                        }
                        Err(_) => "Impulse must be a number between 0 and 255".to_owned(),
                    },

                    _ => "usage: impulse [number]".to_owned(),
                }
            }),
        )
        .unwrap();
    }

    // must be called every frame!
    pub fn refresh(&mut self) {
        self.clear_mouse();
        self.clear_impulse();
    }

    fn clear_mouse(&mut self) {
        self.handle_input(MouseWheel::Up, ElementState::Released);
        self.handle_input(MouseWheel::Down, ElementState::Released);
        self.mouse_delta = (0.0, 0.0);
    }

    fn clear_impulse(&mut self) {
        self.impulse.set(0);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_action_to_string() {
        let act = Action::Forward;
        assert_eq!(act.to_string(), "forward");
    }

    #[test]
    fn test_bind_target_action_to_string() {
        let target = BindTarget::Action {
            trigger: ElementState::Pressed,
            action: Action::Forward,
        };

        assert_eq!(target.to_string(), "+forward");
    }
}
