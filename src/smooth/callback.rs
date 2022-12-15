use std::{cell::RefCell, rc::Rc};

use js_sys::Date;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, KeyboardEvent, WheelEvent, HtmlElement};

use crate::{core::Core, TAB, option::{LocomotiveOption, Position}, utils::{lerp, instance::Instance}};

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
            SmoothScroll::check_scroll(None, core.clone(), options.lerp, options.scrolling_class.clone(), options.direction.clone(), options.name.clone());
            html.as_ref().borrow().class_list().remove_1(&options.scrolling_class).unwrap();
            html.as_ref().borrow().class_list().add_1(&options.dragging_class).unwrap();
        }));

        self.get_scrollbar = callback;
    }

    pub fn check_scroll(forced: Option<bool>, core: Rc<RefCell<Core>>, lerp: f64, scrolling_class: String, direction: String, name: String) {
        let forced = forced.unwrap_or(false);
        let ref_core = core.clone();
        let ref_core = &ref_core.as_ref().borrow().scroll;
        let scroll = ref_core.get_smooth();
        let is_scrolling = scroll.is_scrolling.clone();
        let is_dragging_scrollbar = scroll.is_dragging_scrollbar.clone();
        let has_scroll_ticking = scroll.has_scroll_ticking.clone();
        let instance = core.as_ref().borrow().instance.clone();
        let animating_scroll = scroll.animating_scroll.clone();
        let section = scroll.sections.clone();
        let check_scroll_raf = scroll.check_scroll_raf.clone();
        let scroll_to_raf = scroll.scroll_to_raf.clone();
        let direction_axis = *core.as_ref().borrow().direction_axis.borrow();
        let start_scroll_ts = scroll.start_scroll_ts.clone();

        let core1 = core.clone();
        let scrolling_class1 = scrolling_class.clone();
        let direction1 = direction.clone();
        let name1 = name.clone();
        if forced || *is_scrolling.borrow() || *is_dragging_scrollbar.borrow() {
            {
                if !has_scroll_ticking.as_ref().clone().into_inner() {
                    let cb = Closure::wrap(Box::new(move || {
                        let core = core1.clone();
                        SmoothScroll::check_scroll(None, core, lerp, scrolling_class1.clone(), direction1.clone(), name1.clone());
                    }) as Box<dyn Fn()>);
    
                    let scroll_raf = window().unwrap().request_animation_frame(cb.as_ref().unchecked_ref()).unwrap();
                    *check_scroll_raf.borrow_mut() = Some(scroll_raf);
    
                    *has_scroll_ticking.borrow_mut() = true;
                }
            }

            SmoothScroll::update_scroll(core.clone(),  lerp);

            let (scroll_val, _limit, delta) = match direction_axis {
                'x' => (instance.borrow().scroll.x, instance.borrow().limit.x, instance.borrow().delta.as_ref().unwrap().x),
                'y' => (instance.borrow().scroll.y, instance.borrow().limit.y, instance.borrow().delta.as_ref().unwrap().y),
                _ => panic!()
            };
            let distance = (delta - scroll_val).abs();
            let time_since_start = Date::now() - start_scroll_ts.as_ref().clone().into_inner().as_ref().unwrap_or(&0.0);

            if !*animating_scroll.borrow() && time_since_start > 100.0 && 
            ((distance < 0.5 && delta != 0.0) || (distance < 0.5 && delta == 0.0)) {
                SmoothScroll::stop_scrolling(core.clone(), start_scroll_ts.clone(), check_scroll_raf.clone(), scroll_to_raf.clone(), is_scrolling.clone(), scrolling_class.clone());
            }

            for section in &scroll.sections {
                let mut section = section.as_ref().borrow_mut();
                let (offset, section_limit) = match direction_axis {
                    'x' => (section.offset.x, section.limit.x),
                    'y' => (section.offset.y, section.limit.y),
                    _ => panic!()
                };

                if section.persistent.is_some() || (scroll_val > offset && scroll_val < section_limit) {
                    match direction.as_str() {
                        "horizontal" => {
                            SmoothScroll::transform(section.el.clone(), Some(scroll_val * -1.0), Some(0.0), None);
                        },
                        _ => {
                            SmoothScroll::transform(section.el.clone(), Some(0.0), Some(scroll_val * -1.0), None);
                        }
                    }

                    if !section.in_view {
                        section.in_view = true;
                        let style = section.el.dyn_ref::<HtmlElement>().unwrap().style();
                        style.set_property("opacity", "1").unwrap();
                        style.set_property("pointerEvents", "all").unwrap();
                        section.el.set_attribute(&format!("data-{}-section-inview", &name), "").unwrap()
                    }
                } else {
                    if section.in_view || forced {
                        section.in_view = true;
                        let style = section.el.dyn_ref::<HtmlElement>().unwrap().style();
                        style.set_property("opacity", "0").unwrap();
                        style.set_property("pointerEvents", "none").unwrap();
                        section.el.remove_attribute(&format!("data-{}-section-inview", &name)).unwrap()
                    }

                    SmoothScroll::transform(section.el.clone(), Some(0.0), Some(0.0), None);
                }
            }
            //checkpoint
        }
    }

    pub fn update_scroll(core: Rc<RefCell<Core>>, lerp_val: f64) {
        let ref_core = core.clone();
        let ref_core = &ref_core.as_ref().borrow().scroll;
        let scroll = ref_core.get_smooth();
        let is_scrolling = scroll.is_scrolling.clone();
        let is_dragging_scrollbar = scroll.is_dragging_scrollbar.clone();
        let instance = core.as_ref().borrow().instance.clone();
        let direction_axis = *core.as_ref().borrow().direction_axis.clone().borrow();

        if *is_scrolling.borrow() || *is_dragging_scrollbar.borrow() {
            match direction_axis {
                'x' => {
                    let mut _start = 0.0;
                    let mut _end = 0.0;
                    {
                        _start = instance.borrow().scroll.x;
                        _end = instance.borrow().delta.as_ref().unwrap().x;
                    }
                    instance.borrow_mut().delta.as_mut().unwrap().x = lerp(_start, _end, lerp_val);
                },
                'y' => {
                    let mut _start = 0.0;
                    let mut _end = 0.0;
                    {
                        _start = instance.borrow().scroll.y;
                        _end = instance.borrow().delta.as_ref().unwrap().y;
                    }
                    instance.borrow_mut().delta.as_mut().unwrap().y = lerp(_start, _end, lerp_val);
                },
                _ => panic!()
            }
        } else {
            let (scroll, limit, delta, scroll_y) = match direction_axis {
                'x' => (instance.borrow().scroll.x, instance.borrow().limit.x, instance.borrow().delta.as_ref().unwrap().x, instance.borrow().scroll.y),
                'y' => (instance.borrow().scroll.y, instance.borrow().limit.y, instance.borrow().delta.as_ref().unwrap().y, instance.borrow().scroll.y),
                _ => panic!()
            };
            if scroll > limit {
                SmoothScroll::set_scroll(instance.clone(), scroll, limit);
            } else if scroll_y < 0.0 {
                SmoothScroll::set_scroll(instance.clone(), scroll, 0.0);
            } else {
                SmoothScroll::set_scroll(instance.clone(), scroll, delta);
            }
        }
    }

    pub fn set_scroll(instance: Rc<RefCell<Instance>>, x: f64, y: f64) {
        let mut instance = instance.as_ref().borrow_mut();
        instance.set_scroll(Position::new(x, y));
        instance.set_delta(Position::new(x, y));
        instance.speed = Some(0.0);
    }

    pub fn stop_scrolling(core: Rc<RefCell<Core>>, start_scroll_ts: Rc<RefCell<Option<f64>>>, check_scroll_raf: Rc<RefCell<Option<i32>>>,
        scroll_to_raf: Rc<RefCell<Option<i32>>>, is_scrolling: Rc<RefCell<bool>>, scrolling_class: String
    ) {
        let instance = core.as_ref().borrow().instance.clone();
        let html = &core.as_ref().borrow().html;
        window().unwrap().cancel_animation_frame(*check_scroll_raf.as_ref().clone().into_inner().as_ref().unwrap()).unwrap();

        {
            *start_scroll_ts.borrow_mut() = None;
        }

        if let Some(handler) = scroll_to_raf.as_ref().clone().into_inner() {
            window().unwrap().cancel_animation_frame(handler).unwrap();
            *scroll_to_raf.as_ref().borrow_mut() = None;
        } 

        *is_scrolling.as_ref().borrow_mut() = false;
        let mut instance = instance.borrow_mut();
        instance.scroll.y = instance.scroll.y.round();
        html.as_ref().borrow().class_list().remove_1(&scrolling_class).unwrap(); 
    }
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

    pub fn vs_cb_1(&mut self, core: Rc<RefCell<Core>>, options: LocomotiveOption) {
        let callback: Rc<RefCell<Option<Closure<dyn Fn(WheelEvent)>>>> = Rc::new(RefCell::new(None));
        let stop = self.stop.clone();
        let is_dragging_scrollbar = self.is_dragging_scrollbar.clone();
        let is_scrolling = self.is_scrolling.clone();

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
}