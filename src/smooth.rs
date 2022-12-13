
use std::{rc::Rc, cell::RefCell};

use js_sys::Function;
use wasm_bindgen::{JsValue, prelude::Closure, JsCast};
use web_sys::{Element, KeyboardEvent, window};

use crate::{option::LocomotiveOption, virtual_scroll::VirtualScroll, TAB, instance::Instance, UP, DOWN, PAGEUP, PAGEDOWN, HOME, END};

#[derive(Debug, Clone)]
pub struct ParallaxElements {

}

impl ParallaxElements {
    pub fn new() -> Self {
        ParallaxElements {  }
    }
}


#[derive(Debug, Clone)]
pub struct Section {

}

#[derive(Debug, Clone)]
pub struct SmoothScroll {
    pub options: LocomotiveOption,
    pub is_scrolling: Rc<RefCell<bool>>,
    pub is_dragging_scrollbar: bool,
    pub is_ticking: bool,
    pub has_scroll_ticking: bool,
    pub parallax_elements: ParallaxElements,
    pub stop: Rc<RefCell<bool>>,
    pub scrollbar_container: bool,
    pub check_key: Rc<RefCell<Option<Closure<dyn FnMut(KeyboardEvent)>>>>,

    pub virtual_scroll: Option<VirtualScroll>,
    pub animating_scroll: Rc<RefCell<bool>>,

    //stop scrolling
    pub check_scroll_raf: Rc<RefCell<Option<i32>>>,
    pub start_scroll_ts: Rc<RefCell<Option<bool>>>,
    pub scroll_to_raf: Rc<RefCell<Option<i32>>>,

    //checkScroll
    pub sections: Rc<RefCell<Vec<Section>>>

}

impl SmoothScroll {
    pub fn new(mut options: LocomotiveOption, html: Rc<RefCell<Element>>, instance: Rc<RefCell<Instance>>, direction_axis: Rc<RefCell<char>>) -> Self {

        //1
        if let Some(inertia) = options.inertia {
            options.lerp = inertia * 0.1;
        }

        let scrollbar_container = options.scroll_bar_container;

        let mut smooth = Self {
            options,
            is_scrolling: Rc::new(RefCell::new(false)),
            is_dragging_scrollbar: false,
            is_ticking: false,
            has_scroll_ticking: false,
            parallax_elements: ParallaxElements::new(),
            stop: Rc::new(RefCell::new(false)),
            scrollbar_container,
            check_key: Rc::new(RefCell::new(None)),

            virtual_scroll: None,
            animating_scroll: Rc::new(RefCell::new(false)),

            check_scroll_raf: Rc::new(RefCell::new(None)),
            start_scroll_ts: Rc::new(RefCell::new(None)),
            scroll_to_raf: Rc::new(RefCell::new(None)),
            sections: Rc::new(RefCell::new(vec![]))
        };

        smooth.check_key_callback(html, instance, direction_axis);
        todo!()
    }

    pub fn init(&mut self, html: Rc<RefCell<Element>>, instance: &mut Instance) {
        //1
        html.as_ref().borrow().class_list().add_1(&self.options.smooth_class).unwrap();
        html.as_ref().borrow().set_attribute(&format!("data-{}-direction", &self.options.name), &self.options.direction).unwrap();

        //2
        instance.set_delta(self.options.init_position.clone());
        instance.set_scroll(self.options.init_position.clone());


        //todo //3
        self.virtual_scroll = Some(VirtualScroll::new(JsValue::from_bool(true)));
    }

    pub fn scroll_to(&self, _target: Option<Element>, _attr: Option<String>) {

    }

    pub fn update(&self) {

    }

    pub fn start(&self) {
        
    }

    pub fn stop(&self) {
        
    }

    pub fn set_scroll(&mut self, _x: Option<f64>, _y: Option<f64>) {

    }

    pub fn on(&self, _event: &str, _callback: Function) {
        
    }

    pub fn off(&self, _event: &str, _callback: Function) {

    }

    pub fn destroy(&self) {
        
    }
}


//EVENT INITS
impl SmoothScroll { 
    fn check_key_callback(&mut self, html: Rc<RefCell<Element>>, instance: Rc<RefCell<Instance>>, direction_axis: Rc<RefCell<char>>) {
        let callback: Rc<RefCell<Option<Closure<dyn FnMut(KeyboardEvent)>>>> = Rc::new(RefCell::new(None));
        let stop = self.stop.clone();
        let scroll_to_raf = self.scroll_to_raf.clone();
        let start_scroll_ts = self.start_scroll_ts.clone();
        let check_scroll_raf = self.check_scroll_raf.clone();
        let is_scrolling = self.is_scrolling.clone();
        let scrolling_class = self.options.scrolling_class.clone();


        //callback starts here
        *callback.borrow_mut() = Some(Closure::new(move |event: KeyboardEvent| {
            let html = html.clone();
            let instance = instance.clone();
            let direction_axis = direction_axis.clone();
            let scroll_to_raf = scroll_to_raf.clone();
            let start_scroll_ts = start_scroll_ts.clone();
            let check_scroll_raf = check_scroll_raf.clone();
            let is_scrolling= is_scrolling.clone();
            let scrolling_class = scrolling_class.clone();

            //1
            let stop = stop.clone();
            let mut _stop_res = false;
            {
                _stop_res = *stop.as_ref().borrow();
            }
            if _stop_res {
                if event.key_code() == TAB {
                    let animate_callback = Closure::wrap(Box::new(move || {
                        let html = html.clone();
                        let body = window().unwrap().document().unwrap().body().unwrap();
                        html.as_ref().borrow().set_scroll_top(0);
                        body.set_scroll_top(0);
                        html.as_ref().borrow().set_scroll_left(0);
                        body.set_scroll_left(0);


                    }) as Box<dyn Fn()>);
                    window().unwrap().request_animation_frame(animate_callback.as_ref().unchecked_ref()).unwrap();
                    animate_callback.forget();
                }

                return;
            }


            let mut _direction = 'x';
            {
                _direction = *direction_axis.as_ref().borrow();
            }
            //2
            check_key(&event, instance.clone(), _direction);

            //3
            update_instance_delta(instance.clone(), _direction);
            
            //4
            Self::stop_scrolling(check_scroll_raf, scroll_to_raf, start_scroll_ts, is_scrolling.clone(), instance, html, scrolling_class);
            *is_scrolling.as_ref().borrow_mut() = true;

        }));

        self.check_key = callback;
    }


    fn stop_scrolling(check_scroll_raf: Rc<RefCell<Option<i32>>>, scroll_to_raf: Rc<RefCell<Option<i32>>>, start_scroll_ts: Rc<RefCell<Option<bool>>>,
        is_scrolling: Rc<RefCell<bool>>, instance: Rc<RefCell<Instance>>, html: Rc<RefCell<Element>>, scrolling_class: String
    ) {
        let check_scroll_raf = check_scroll_raf.as_ref().clone().into_inner();
        let scroll_to_raf_res = scroll_to_raf.as_ref().clone().into_inner();
        if let Some(val) = check_scroll_raf {
            window().unwrap().cancel_animation_frame(val).unwrap();
        }

        *start_scroll_ts.as_ref().borrow_mut() = None;

        if let Some(val) = scroll_to_raf_res {
            window().unwrap().cancel_animation_frame(val).unwrap();
            *scroll_to_raf.as_ref().borrow_mut() = None;
        }

        *is_scrolling.as_ref().borrow_mut() = false;
        let rounded_y = instance.as_ref().clone().into_inner().scroll.y.round();
        instance.as_ref().borrow_mut().scroll.y = rounded_y;
        html.as_ref().borrow().class_list().remove_1(&scrolling_class).unwrap();
    }

    fn _check_scroll(_forced: Option<bool>) {
        
    }
}





























//UTILS
fn check_key(event: &KeyboardEvent, _instance: Rc<RefCell<Instance>>, _direction_axis: char) {
    //window().unwrap().document().unwrap().active_element().unwrap()

    match event.key_code() {
        TAB => {

        },
        UP => {

        },
        DOWN => {

        },
        PAGEUP => {

        },
        PAGEDOWN => {

        },
        HOME => {

        },
        END => {

        },
        _ => {
            ()
        }
    }
}

fn update_instance_delta(instance: Rc<RefCell<Instance>>, direction_axis: char) {
    let mut instance = instance.borrow_mut();
    let limit = instance.get_limit(direction_axis);

    if instance.get_delta(direction_axis) < 0.0 {
        instance.update_delta(direction_axis, 0.0);
    }
    if instance.get_delta(direction_axis) > limit {
        instance.update_delta(direction_axis, limit);
    }
}




#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell};

    use crate::{instance::Instance, option::Position, core::CurrentElements};

    use super::update_instance_delta;

    #[test]
    fn it_works() {
        let instance = Instance {
            scroll: Position::new(0.0, 0.0),
            limit: Position { x: -10.0, y: 20.0 },
            delta: Some(Position::new(-2.0, 2.0)),
            current_elements: CurrentElements::new(),
            direction: Some(String::from("horizontal")),
            speed: None,
        };
        let direction_axis = 'x';
        let instance = Rc::new(RefCell::new(instance));

        update_instance_delta(instance.clone(), direction_axis);
        println!("{:?}", instance)
    }
}