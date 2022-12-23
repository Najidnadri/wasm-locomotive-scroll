use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, window, Element};

use crate::{option::LocomotiveOption, utils::els::{ScrollToTarget, ScrollToOption}};

use super::Core;




impl Core {
    pub fn check_scroll_callback(core: Rc<RefCell<Core>>) {
        
        let core2 = core.clone();

        let callback: Rc<RefCell<Option<Closure<dyn FnMut() >>>> = Rc::new(RefCell::new(None));
        *callback.borrow_mut() = Some(Closure::new(move || {
            let core = core.clone();
            let core_ref = core.borrow();
            let option = core_ref.scroll.get_option();
            let element = option.el.clone();
            let namespace = &core.as_ref().borrow().namespace;


            let scroll_event = Event::new(&format!("{}scroll", namespace.as_ref().borrow())).unwrap();
            element.dispatch_event(&scroll_event);
        }));
        {
            core2.borrow_mut().check_scroll = callback;
        }

    }

    pub fn check_resize_callback(core: Rc<RefCell<Core>>) {
        let callback: Rc<RefCell<Option<Closure<dyn FnMut() >>>> = Rc::new(RefCell::new(None));
        let core2 = core.clone();

        
        *callback.borrow_mut() = Some(Closure::new(move || {
            let core = core.clone();
            let mut _resize_tick = true;
            {
                _resize_tick = *core.as_ref().borrow().resize_tick.as_ref().borrow();
            }
            if !_resize_tick {
                {
                    *core.as_ref().borrow().resize_tick.as_ref().borrow_mut() = true;
                }
                let callback = Closure::wrap(Box::new(move || {
                    // todo
                    Core::resize(core.clone());
                    *core.as_ref().borrow().resize_tick.as_ref().borrow_mut() = false;
                }) as Box<dyn Fn()>);
                window().unwrap().request_animation_frame(callback.as_ref().unchecked_ref()).unwrap();
                callback.forget();
            }   
        }));
        {
            core2.as_ref().borrow_mut().check_resize = callback;
        }
    }

    pub fn check_event_callback(core: Rc<RefCell<Core>>) {
        let callback: Rc<RefCell<Option<Closure<dyn FnMut(Event) >>>> = Rc::new(RefCell::new(None));
        let core2 = core.clone();

        *callback.borrow_mut() = Some(Closure::new(move |event: Event| {
            let core = core.clone();
            let namespace = &core.as_ref().borrow().namespace;
            let _event_name = event.type_().replace(&namespace.as_ref().borrow().to_string(), "");
            //todo
        }));

        {
            core2.as_ref().borrow_mut().check_event = callback;
        }
        
    }

    pub fn set_scroll_to_callback(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let callback: Rc<RefCell<Option<Closure<dyn FnMut(Event) >>>> = Rc::new(RefCell::new(None));
        let name = options.name.clone();
        let core = core.clone();
        let core2 = core.clone();
        let options = options.clone();


        *callback.borrow_mut() = Some(Closure::new(move |event: Event| {
            let core = core.clone();
            let scroll = core.as_ref().borrow().scroll.clone();
            let options = options.clone();

            event.prevent_default();

            let element = event.current_target().unwrap().dyn_into::<Element>().unwrap();
            let attr = match element.get_attribute(&format!("data-{}-href", name.clone())) {
                Some(attr) => attr,
                None => element.get_attribute("href").unwrap()
            };
            let target = ScrollToTarget::String(attr);
            let option = ScrollToOption {
                offset: element.get_attribute(&format!("data-{}-offset", name.clone())),
                callback: None,
                duration: None,
                ..Default::default()
            };

            Core::scroll_to(scroll.clone(), target, option, core.clone(), &options);
        }));

        {
            core2.as_ref().borrow_mut().set_scroll_to = callback;
        }
        
    }
}