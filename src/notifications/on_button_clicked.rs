use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::event_listener::EventListener;
use crate::notifications::{notifications, sys};

pub fn on_button_clicked() -> OnButtonClicked {
    OnButtonClicked(notifications().on_button_clicked())
}

pub struct OnButtonClicked(sys::EventTarget);

pub struct OnButtonClickedEventListener<'a>(EventListener<'a, dyn FnMut(JsValue, JsValue)>);

impl OnButtonClickedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnButtonClicked {
    pub fn add_listener<L>(&self, mut listener: L) -> OnButtonClickedEventListener
        where
            L: FnMut(String, f64) + 'static
    {
        let listener = Closure::new(move |id: JsValue, index: JsValue| {
            listener(
                id.as_string().unwrap(),
                index.as_f64().unwrap()
            )
        });
        OnButtonClickedEventListener(EventListener::raw_new(&self.0, listener))
    }
}