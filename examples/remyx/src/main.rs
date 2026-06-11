//! remyx — the Elm architecture on top of Ratatui.
//!
//! remyx is an Iced-inspired framework: you implement an `Application` with `view`
//! and `update`, and it owns the event loop, async tasks, and terminal teardown.
//! `Element` is the render abstraction (stateless Ratatui widgets are supported, plus
//! `PickList`). `subscription()` feeds async streams of your `Message` type into update.
//!
//! The exact trait signatures move with the crate, so treat the skeleton below as a
//! shape to fill in against the official examples:
//!   https://github.com/manuelgdlvh/remyx/tree/master/examples
//!
//! Verdict from Digest #1: ASSESS — a convention layer over Ratatui is a real need,
//! but this is brand new and single-author. Good for a spike; let it mature for the long haul.

// Sketch of the Elm-style shape remyx asks you to implement:
//
// #[derive(Debug, Clone)]
// enum Message { Increment, Decrement }
//
// #[derive(Default)]
// struct Counter { value: i64 }
//
// impl remyx::Application for Counter {
//     type Message = Message;
//
//     fn update(&mut self, message: Self::Message) {
//         match message {
//             Message::Increment => self.value += 1,
//             Message::Decrement => self.value -= 1,
//         }
//     }
//
//     fn view(&self) -> impl remyx::Element<Self::Message> {
//         // Compose Ratatui widgets / remyx Elements describing the UI here.
//         todo!("build the view — see the remyx /examples folder")
//     }
// }

fn main() {
    println!("See src/main.rs + https://github.com/manuelgdlvh/remyx/tree/master/examples");
    println!("Implement remyx::Application (update + view) and run the event loop.");
}
