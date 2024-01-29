use wasm_bindgen::closure::Closure;
use crate::context_menus::context_menus;
use crate::event_listener::EventListener;
use crate::tabs::prelude::sys;
use crate::tabs::Tab;
use crate::context_menus::OnClickData;

pub fn on_clicked() -> OnClicked {
    OnClicked(context_menus().on_clicked())
}

pub struct OnClicked(sys::EventTarget);

pub struct OnClickedEventListener<'a>(EventListener<'a, dyn FnMut(sys::OnClickData, sys::Tab)>);

impl OnClickedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget();
    }
}

impl OnClicked {
    pub fn add_listener<L>(&self, mut listener: L) -> OnClickedEventListener
    where
        L: FnMut(OnClickData, Tab) + 'static,
    {
        let listener = Closure::new(move |data, tab|
            listener(OnClickData::from(data), Tab::from(tab)));
        OnClickedEventListener(EventListener::raw_new(&self.0, listener))
    }
}