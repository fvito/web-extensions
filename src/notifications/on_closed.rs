use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::event_listener::EventListener;
use crate::notifications::{notifications, sys};

pub fn on_closed() -> OnClosed {
    OnClosed(notifications().on_closed())
}

pub struct OnClosed(sys::EventTarget);

pub struct OnClosedEventListener<'a>(EventListener<'a, dyn FnMut(JsValue, JsValue)>);

impl OnClosedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnClosed {
    pub fn add_listener<L>(&self, mut listener: L) ->OnClosedEventListener
        where
            L: FnMut(String, bool) + 'static
    {
        let listener = Closure::new(move |id: JsValue, by_user:JsValue| {
            listener(
                id.as_string().unwrap(),
                by_user.as_bool().unwrap()
            )
        });
        OnClosedEventListener(EventListener::raw_new(&self.0, listener))
    }
}