use dyn_clone::DynClone;
use leptos::ev::MouseEvent;

pub trait OnClickCallback: FnMut(MouseEvent) + DynClone + Send {}

impl<T> OnClickCallback for T where T: FnMut(MouseEvent) + Clone + Send {}

dyn_clone::clone_trait_object!(OnClickCallback);
