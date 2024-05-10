use cursive::{
  event::{Event, EventResult, Key},
  view::ViewWrapper,
  View,
};

/// Wraps the inner view with HJKL directional controls.
pub struct HjklToDirectionWrapperView<T> {
  view: T,
}

impl<T> HjklToDirectionWrapperView<T> {
  pub fn new(view: T) -> Self {
    Self { view }
  }

  cursive::inner_getters!(self.view: T);
}

impl<T> ViewWrapper for HjklToDirectionWrapperView<T>
where
  T: View,
{
  cursive::wrap_impl!(self.view: T);

  fn wrap_on_event(&mut self, ev: Event) -> EventResult {
    let ev_result = self.view.on_event(ev.clone());
    if !matches!(&ev_result, EventResult::Ignored) {
      return ev_result;
    }

    // tuple enum variants are secretly 1-argument functions
    // which means you can pull stuff like this
    type EventCtor = fn(Key) -> Event;
    let (ch, ctor) = match &ev {
      Event::Char(c) => (
        c,
        if c.is_ascii_uppercase() {
          Event::Shift
        } else {
          Event::Key
        } as EventCtor,
      ),
      Event::CtrlChar(c) => (
        c,
        if c.is_ascii_uppercase() {
          Event::CtrlShift
        } else {
          Event::Ctrl
        } as EventCtor,
      ),
      Event::AltChar(c) => (
        c,
        if c.is_ascii_uppercase() {
          Event::AltShift
        } else {
          Event::Alt
        } as EventCtor,
      ),
      _ => return EventResult::Ignored,
    };

    let dir = match ch {
      'h' => Key::Left,
      'j' => Key::Down,
      'k' => Key::Up,
      'l' => Key::Right,
      _ => return EventResult::Ignored,
    };
    let the_cooler_event = ctor(dir);
    self.view.on_event(the_cooler_event)
  }
}
