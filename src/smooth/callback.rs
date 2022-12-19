use std::{cell::RefCell, rc::Rc};


use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, KeyboardEvent, WheelEvent, MouseEvent};

use crate::{core::Core, TAB, option::LocomotiveOption};

use super::SmoothScroll;


// MAIN CLOSURE
impl SmoothScroll {
    pub fn check_key_callback(&mut self, core: Rc<RefCell<Core>>) {
        let callback: Rc<RefCell<Option<Closure<dyn FnMut(KeyboardEvent)>>>> = Rc::new(RefCell::new(None));
        let stop = self.stop.clone();
        let check_key_cb_1 = self.check_key_cb_1.clone();

        *callback.borrow_mut() = Some(Closure::new(move |event: KeyboardEvent| {
            let window = window().unwrap();
            let _core = core.clone();
            let stop = stop.clone();
            let check_key_cb_1 = check_key_cb_1.clone();
            let key = event.key_code();

            if *stop.as_ref().borrow() {
                if key == TAB {
                    window.request_animation_frame(check_key_cb_1.as_ref().borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
                }
                return
            }

            match key {
                TAB => {
                    
                },
                _ => {

                }
            }

        }));

        self.check_key = callback;
    }

    pub fn get_scrollbar(&mut self, core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let callback: Rc<RefCell<Option<Closure<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let is_dragging_scrollbar = self.is_dragging_scrollbar.clone();
        let html = Core::get_html(core.clone());
        let options = options.clone();
      

        *callback.borrow_mut() = Some(Closure::new(move || {
            let is_dragging_scrollbar = is_dragging_scrollbar.clone();
            let html = html.clone();
            let options = options.clone();

            {
                *is_dragging_scrollbar.as_ref().borrow_mut() = true;
            }
            SmoothScroll::check_scroll(None, core.clone(), options.clone());
            html.as_ref().borrow().class_list().remove_1(&options.scrolling_class).unwrap();
            html.as_ref().borrow().class_list().add_1(&options.dragging_class).unwrap();
        }));

        self.get_scrollbar = callback;
    }

    pub fn release_scrollbar_cb(&mut self, core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let callback: Rc<RefCell<Option<Closure<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let scrolling_class = options.scrolling_class.clone();
        let dragging_class = options.dragging_class.clone();
      

        *callback.borrow_mut() = Some(Closure::new(move || {
            let core = core.clone();
            let core_ref = core.as_ref().borrow();
            let scroll = core_ref.scroll.get_smooth();
            let scrolling_class = scrolling_class.clone();
            let dragging_class = dragging_class.clone();

            {
                *scroll.is_dragging_scrollbar.borrow_mut() = false;
            }

            {
                if *scroll.is_scrolling.as_ref().borrow() == true {
                    core_ref.html.borrow().class_list().add_1(&scrolling_class).unwrap()
                }
            }

            let _ = core_ref.html.borrow().class_list().remove_1(&dragging_class);
        }));

        self.release_scrollbar = callback;
    }


    pub fn move_scrollbar_cb(&mut self, core: Rc<RefCell<Core>>) {
        let callback: Rc<RefCell<Option<Closure<dyn Fn(MouseEvent)>>>> = Rc::new(RefCell::new(None));
        self.move_scrollbar_cb_2(core.clone());

        *callback.borrow_mut() = Some(Closure::new(move |event: MouseEvent| {
            let core = core.clone();
            let core_ref = core.borrow();
            let scroll = core_ref.scroll.get_smooth();
            let is_dragging_scrollbar = scroll.is_dragging_scrollbar.clone();
            {
                *scroll.mouse_event.clone().borrow_mut() = Some(event);
            }

            if *is_dragging_scrollbar.borrow() {
                window().unwrap().request_animation_frame(scroll.move_scrollbar_cb_2.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
            }
        }));

        self.move_scrollbar = callback;
    }
    

    /* 
    pub fn add_elements(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let core = core.as_ref().borrow_mut();
        let els = core.els.clone();
        let section = 
        
        {
            els.as_ref().borrow_mut().data.clear();
        }

        let elements = options.el.query_selector_all(&format!("[data-{}]", options.name)).unwrap();
        
        for index in 0 .. elements.length() {
            let element = elements.get(index).unwrap();
            let parents = get_parents(element.dyn_ref::<Element>().unwrap().clone());

        }
    }
    */

}




// OTHERS CLOSURES
impl SmoothScroll {
    pub fn check_key_cb_1(&mut self, core: Rc<RefCell<Core>>) {
        let callback: Rc<RefCell<Option<Closure<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let html = Core::get_html(core.clone());

        *callback.borrow_mut() = Some(Closure::new(move || {
            let html = html.clone();
            let body = window().unwrap().document().unwrap().body().unwrap();

            html.as_ref().borrow().set_scroll_top(0);
            body.set_scroll_top(0);
            html.as_ref().borrow().set_scroll_left(0);
            body.set_scroll_left(0);
        }));

        self.check_key_cb_1 = callback;
    }

    pub fn check_key_cb_2(&mut self, core: Rc<RefCell<Core>>) {
        let callback: Rc<RefCell<Option<Closure<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let html = Core::get_html(core.clone());

        *callback.borrow_mut() = Some(Closure::new(move || {
            let html = html.clone();
            let body = window().unwrap().document().unwrap().body().unwrap();

            html.as_ref().borrow().set_scroll_top(0);
            body.set_scroll_top(0);
            html.as_ref().borrow().set_scroll_left(0);
            body.set_scroll_left(0);

            //Core::scroll_to(scroll, target, scroll_to_option, html, instance)
        }));

        self.check_key_cb_1 = callback;
    }

    pub fn vs_cb_1(&mut self, core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let callback: Rc<RefCell<Option<Closure<dyn Fn(WheelEvent)>>>> = Rc::new(RefCell::new(None));
        let stop = self.stop.clone();
        let is_dragging_scrollbar = self.is_dragging_scrollbar.clone();
        let is_scrolling = self.is_scrolling.clone();
        let options = options.clone();

        *callback.borrow_mut() = Some(Closure::new(move |event: WheelEvent| {
            let core = core.clone();
            let stop = stop.clone();
            let options = options.clone();
            let is_dragging_scrollbar = is_dragging_scrollbar.clone();
            let is_scrolling = is_scrolling.clone();

            if *stop.as_ref().borrow() == false {
                if *is_dragging_scrollbar.as_ref().borrow() == false {
                    let cb = Closure::wrap(Box::new(move || {
                        let core = core.clone();
                        let options = options.clone();
                        let event = event.clone();
                        let is_scrolling = is_scrolling.clone();
                        SmoothScroll::update_delta(event, core.clone(), options.clone());

                        if *is_scrolling.as_ref().borrow() == false {
                            SmoothScroll::start_scrolling(core.clone(), options.clone());
                        }
                    }) as Box<dyn Fn()>);

                    window().unwrap().request_animation_frame(cb.as_ref().unchecked_ref()).unwrap();
                }
            }
        }));

        self.vs_cb_1 = callback;
    }

    pub fn check_scroll_cb(&mut self, core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let callback: Rc<RefCell<Option<Closure<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let options = options.clone();

        *callback.borrow_mut() = Some(Closure::new(move || {
            let options = options.clone();
            let core = core.clone();
            SmoothScroll::check_scroll(None, core, options);
        }));

        self.check_scroll_cb = callback;
    }

    pub fn move_scrollbar_cb_2(&mut self, core: Rc<RefCell<Core>>) {
        let callback: Rc<RefCell<Option<Closure<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let event = self.mouse_event.clone();
        let scrollbar_bcr = self.scrollbar_bcr.clone();
        let scrollbar_width = self.scrollbar_width.clone();
        let scrollbar_height = self.scrollbar_height.clone();

        *callback.borrow_mut() = Some(Closure::new(move || {
            let core = core.clone();
            let core_ref = core.borrow();
            let instance = core_ref.instance.clone();
            let mut instance = instance.borrow_mut();
            let event = event.clone();
            let event = event.borrow();
            let event = event.as_ref().unwrap();
            let scrollbar_bcr = scrollbar_bcr.clone();
            let scrollbar_bcr = scrollbar_bcr.borrow();
            let scrollbar_bcr = scrollbar_bcr.as_ref().unwrap();
            let scrollbar_width = scrollbar_width.clone();
            let scrollbar_width = scrollbar_width.borrow();
            let scrollbar_width = scrollbar_width.as_ref().unwrap();
            let scrollbar_height = scrollbar_height.clone();
            let scrollbar_height = scrollbar_height.borrow();
            let scrollbar_height = scrollbar_height.as_ref().unwrap();

            let x = ((((event.client_x() as f64 - scrollbar_bcr.left()) * 100.0) / scrollbar_width) * instance.limit.x) / 100.0;
            let y = ((((event.client_y() as f64 - scrollbar_bcr.top()) * 100.0) / scrollbar_height) * instance.limit.y) / 100.0;

            if y > 0.0 && y < instance.limit.y {
                instance.delta.as_mut().unwrap().y = y;
            }
            if x > 0.0 && x < instance.limit.x {
                instance.delta.as_mut().unwrap().x = x;
            }
        }));

        self.move_scrollbar_cb_2 = callback;
    }
}