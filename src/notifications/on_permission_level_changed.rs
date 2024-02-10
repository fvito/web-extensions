use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::event_listener::EventListener;
use crate::notifications::{notifications, PermissionLevel, sys};

pub fn on_permission_level_changed() -> OnPermissionLevelChanged{
    OnPermissionLevelChanged(notifications().on_permission_level_changed())
}

pub struct OnPermissionLevelChanged(sys::EventTarget);

pub struct OnPermissionLevelChangedEventListener<'a>(EventListener<'a, dyn FnMut(JsValue)>);

impl OnPermissionLevelChangedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnPermissionLevelChanged {
    pub fn add_listener<L>(&self, mut listener: L) -> OnPermissionLevelChangedEventListener
        where
            L: FnMut(PermissionLevel) + 'static
    {
        let listener = Closure::new(move |id| {
            listener(PermissionLevel::from(id))
        });
        OnPermissionLevelChangedEventListener(EventListener::raw_new(&self.0, listener))
    }
}