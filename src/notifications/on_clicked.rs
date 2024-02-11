use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::event_listener::EventListener;
use crate::notifications::{notifications, sys};

pub fn on_clicked() -> OnClicked {
    OnClicked(notifications().on_clicked())
}

pub struct OnClicked(sys::EventTarget);

pub struct OnClickedEventListener<'a>(EventListener<'a, dyn FnMut(JsValue)>);

impl OnClickedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnClicked {
    pub fn add_listener<L>(&self, mut listener: L) -> OnClickedEventListener
        where
            L: FnMut(String) + 'static
    {
        let listener = Closure::new(move |id: JsValue| {
            listener(id.as_string().unwrap())
        });
        OnClickedEventListener(EventListener::raw_new(&self.0, listener))
    }
}