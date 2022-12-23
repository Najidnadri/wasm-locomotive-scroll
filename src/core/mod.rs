mod callback;
mod utils;

use std::{rc::Rc, cell::RefCell};

use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Element, NodeList, window, Event};

use crate::{option::{LocomotiveOption, Position}, Scroll, smooth::SmoothScroll, utils::{current_elements::CurrentElements, listeners::Listeners, els::{Els, MappedEl, ScrollToTarget, ScrollToOption}, instance::Instance, element_type::ElementType}};





#[derive(Clone, Debug)]
pub struct Core {
    pub namespace: Rc<RefCell<String>>,
    pub html: Rc<RefCell<Element>>,
    pub window_height: f64,
    pub window_width: f64,
    pub window_middle: Position,
    pub els: Rc<RefCell<Els>>,
    pub current_elements: Rc<RefCell<CurrentElements>>,
    pub listeners: Listeners,
    pub has_scroll_ticking: Rc<RefCell<bool>>,
    pub has_scroll_bar: Rc<RefCell<Option<bool>>>,
    pub scroll_bar_limit: Rc<RefCell<Position>>,
    pub has_call_event_set: bool,
    pub check_scroll: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
    pub check_resize: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
    pub check_event: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>,
    pub set_scroll_to: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>>,
    pub instance: Rc<RefCell<Instance>>,
    pub context: Rc<RefCell<String>>,
    pub direction_axis: Rc<RefCell<char>>,
    pub resize_tick: Rc<RefCell<bool>>,
    pub scroll_to_els: Option<NodeList>,
    pub scroll: Scroll,

    //Scroll
    call_way: Rc<RefCell<String>>,
    call_value: Rc<RefCell<Vec<String>>>,
    call_obj: Rc<RefCell<Option<MappedEl>>>
}


impl Core {
    pub fn new(mut options: LocomotiveOption) -> Rc<RefCell<Self>> {
        let window = window().unwrap();

        if Self::check_smooth_scroll(&options) {
            if window.history().unwrap().scroll_restoration().is_err() {
                window.history().unwrap().set_scroll_restoration(web_sys::ScrollRestoration::Manual).unwrap()
            }
            window.scroll_to_with_x_and_y(0., 0.);
        }

        let html = window.document().unwrap().document_element().unwrap();
        let window_height = window.inner_height().unwrap().as_f64().unwrap();
        let window_width = window.inner_width().unwrap().as_f64().unwrap();
        let window_middle = Position {
            y: window_height / 2.,
            x: window_width / 2.
        };
        let current_elements = CurrentElements::new();
        let mut instance = Instance::new(&html, current_elements.clone());

        let context = match (options.is_mobile, options.is_tablet) {
            (true, true) => {
                options.direction = options.tablet.as_ref().unwrap().direction.clone();
                "tablet".to_string()
            },
            (true, false) => {
                options.direction = options.smartphone.as_ref().unwrap().direction.clone();
                "smartphone".to_string()
            },
            (false, _) => "desktop".to_string(),
        };
        
        let direction_axis = if options.direction.as_str() == "horizontal" {
            'x'
        } else {
            'y'
        };

        if options.get_direction {
            instance.direction = None;
            instance.speed = Some(0.);
        }

        html.class_list().add_1(&options.init_class).unwrap();
        

        let html = Rc::new(RefCell::new(html));
        let instance = Rc::new(RefCell::new(instance));
        let direction_axis = Rc::new(RefCell::new(direction_axis));

        let core = Core {
            namespace: Rc::new(RefCell::new("locomotive".to_string())),
            html: html.clone(),
            window_height, 
            window_width,
            window_middle,
            els: Rc::new(RefCell::new(Els::new())),
            current_elements: Rc::new(RefCell::new(current_elements)), 
            listeners: Listeners::new(),
            has_scroll_ticking: Rc::new(RefCell::new(false)),
            has_scroll_bar: Rc::new(RefCell::new(None)),
            scroll_bar_limit: Rc::new(RefCell::new(Position::new(0.0, 0.0))),
            has_call_event_set: false, 
            check_scroll: Rc::new(RefCell::new(None)),
            check_resize: Rc::new(RefCell::new(None)),
            check_event: Rc::new(RefCell::new(None)), 
            set_scroll_to: Rc::new(RefCell::new(None)),
            instance: instance.clone(),
            context: Rc::new(RefCell::new(context)),
            direction_axis: direction_axis.clone(),
            resize_tick: Rc::new(RefCell::new(false)),
            scroll_to_els: None,
            scroll: Scroll::None,

            call_way: Rc::new(RefCell::new(String::new())),
            call_value: Rc::new(RefCell::new(vec![])),
            call_obj: Rc::new(RefCell::new(None)),
        };

        let core = Rc::new(RefCell::new(core));

        let scroll = Core::create_scroll(options.clone(),  core.clone());
        {
            web_sys::console::log_1(&"6".into());
            core.borrow_mut().scroll = scroll;
        }
        

        Core::check_scroll_callback(core.clone());
        Core::check_resize_callback(core.clone());
        Core::set_scroll_to_callback(core.clone(), &options);
      
        core

    }

    pub fn init(core: Rc<RefCell<Core>>) {
        //self.scroll.as_mut().unwrap().init(self.html.clone(), instance);
        let mut _smooth_scroll = false;
        let mut _options = None;

        {
            _smooth_scroll = core.borrow().scroll.is_smooth();
            _options = Some(core.borrow().scroll.get_option().clone());
        }

        match _smooth_scroll {
            true => {
                SmoothScroll::init(core.clone(), _options.as_ref().unwrap());
            },
            _ => {
                todo!()
            }
        }

        Core::init_events(core, _options.as_ref().unwrap());
    }

    pub fn init_events(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {

        {
            web_sys::console::log_1(&"7".into());
            core.borrow_mut().scroll_to_els = options.el.query_selector_all(&format!("[data-{}-to", options.name));
        }

        let core = core.clone();
        let core_ref = core.as_ref().borrow();

        if let Some(node_list) = core_ref.scroll_to_els.as_ref() {
            for i in 0 .. node_list.length() {
                let node = node_list.get(i).unwrap();
                let cb = core.as_ref().borrow();
                let cb = cb.set_scroll_to.borrow();
                let cb = cb.as_ref().unwrap();
                node.add_event_listener_with_callback_and_bool("click", cb.as_ref().unchecked_ref(), false).unwrap();
            }
        }
    }
}

impl Core {

    fn create_scroll(options: LocomotiveOption, core: Rc<RefCell<Core>>) -> Scroll {
    //Setup the `Scroll` Field

        if (options.smooth && !options.is_mobile)  || 
            (options.tablet.as_ref().unwrap().smooth && options.is_tablet) ||
            (options.smartphone.as_ref().unwrap().smooth && options.is_mobile && !options.is_tablet) 
            {
                let scroll = SmoothScroll::new(options, core);
                Scroll::Smooth(scroll)
        } else {
                //let scroll = NativeScroll::new(options, window, check_scroll_cb);
                //Scroll::Native(scroll)
                todo!()
        }
    }

    pub(crate) fn check_context(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        if !options.reload_on_context_change {
            return;
        }
        let mut _is_smooth = false;
        {
            _is_smooth = core.borrow().scroll.is_smooth();
        }

        let is_mobile = options.check_mobile_bool();
        {   
            web_sys::console::log_1(&"1".into());
            match _is_smooth {
                true => core.borrow_mut().scroll.get_mut_smooth().options.is_mobile = is_mobile,
                false => todo!()
            }
        }
        let is_tablet = options.check_tablet_bool();
        {
            web_sys::console::log_1(&"2".into());
            match _is_smooth {
                true => core.borrow_mut().scroll.get_mut_smooth().options.is_tablet = is_tablet,
                false => todo!()
            }
        }

        let old_context = core.borrow().context.as_ref().clone().into_inner();
        match (options.is_mobile, options.is_tablet) {
            (true, true) => *core.borrow().context.borrow_mut() = "tablet".to_string(),
            (true, false) => *core.borrow().context.borrow_mut() = "smartphone".to_string(),
            _ => *core.borrow().context.borrow_mut() = "desktop".to_string(),
        }

        if old_context.as_str() != core.borrow().context.borrow().as_str() {
            let old_smooth = match old_context.as_str() {
                "desktop" => options.smooth,
                "tablet" => options.tablet.as_ref().unwrap().smooth,
                "smartphone" => options.tablet.as_ref().unwrap().smooth,
                _ => panic!("device not supported") 
            };
            let new_smooth = match core.borrow().context.borrow().as_str() {
                "desktop" => options.smooth,
                "tablet" => options.tablet.as_ref().unwrap().smooth,
                "smartphone" => options.smartphone.as_ref().unwrap().smooth,
                _ => panic!("device not supported")
            };

            if old_smooth != new_smooth {
                window().unwrap().location().reload().unwrap();
            }
        }
    }

}

//INIT FUNCTIONS
impl Core {
    pub fn scroll_to(scroll: Scroll, target: ScrollToTarget, scroll_to_option: ScrollToOption, core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        //self.scroll.as_ref().unwrap().scroll_to(target_el, attr, None);
        match scroll {
            Scroll::Native(_) => {
                //NativeScroll::scroll_to(target, scroll_to_option, html, instance);
            },
            Scroll::Smooth(_) => {
                SmoothScroll::scroll_to(target, scroll_to_option, core.clone(), options)
            },
            _ => todo!()
        }
    }
}


impl Core {

    pub fn detect_elements(has_call_event_set: Option<bool> , cores: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        {
            let core = cores.clone();
            let core = core.as_ref().borrow();
            let scroll_top = core.instance.as_ref().borrow().scroll.y;
            let scroll_bottom = scroll_top + core.window_height;

            let scroll_left = core.instance.as_ref().borrow().scroll.x;
            let scroll_right = scroll_left + core.window_width;

            let els = core.els.as_ref().clone().into_inner().clone().data;
            let js_val = MappedEl::hash_to_js(&els);
            //web_sys::console::log_1(&js_val);
            for (id, el) in core.els.as_ref().clone().into_inner().clone().data {
                if !el.in_view.as_ref().unwrap() || has_call_event_set == Some(true) {

                    if options.direction.as_str() == "horizontal" {
                        if scroll_right >= el.left && scroll_left < el.right {
                            Core::set_in_view(&el, &id, cores.clone(), options)
                        }
                    } else {
                        if scroll_bottom >= el.top && scroll_top < el.bottom {
                            Core::set_in_view(&el, &id, cores.clone(), options)
                        }
                    }
                }

                if *el.in_view.as_ref().unwrap() {
                    if options.direction.as_str() == "horizontal" {
                        let width = el.right - el.left;
                        let scroll_x = core.instance.as_ref().borrow().scroll.x;
                        let new_progress = (scroll_x - (el.left - core.window_width)) / (width + core.window_width);
                        core.els.as_ref().borrow_mut().data.entry(id.clone()).and_modify(|data| {
                            data.progress = Some(new_progress)
                        });

                        if scroll_right < el.left || scroll_left > el.right {
                            Core::set_out_of_view(&el, &id, cores.clone(), options);
                        }
                    } else {
                        let height = el.bottom - el.top;
                        let scroll_y = core.instance.as_ref().borrow().scroll.y;
                        let new_progress = (scroll_y - (el.top - core.window_height)) / (height + core.window_height);
                        {
                            core.els.as_ref().borrow_mut().data.entry(id.clone()).and_modify(|data| {
                                data.progress = Some(new_progress)
                            });
                        }

                        if scroll_bottom < el.top || scroll_top > el.bottom {
                            Core::set_out_of_view(&el, &id, cores.clone(), options);
                        }
                    }
                }
            }
        }
    

        *cores.as_ref().borrow().has_scroll_ticking.borrow_mut() = false;

    }

    fn resize(core: Rc<RefCell<Core>>) {
        let is_smooth = core.borrow().scroll.is_smooth();
        let scroll = &core.borrow().scroll;
        let options = scroll.get_option();
        match is_smooth {
            true => SmoothScroll::resize(core.clone(), options),
            false => todo!()
        }
    }

    pub fn set_in_view(current: &MappedEl, id: &str, core: Rc<RefCell<Core>>, option: &LocomotiveOption) {
        
        {
            core.as_ref().borrow().els.as_ref().borrow_mut().data.entry(id.to_string()).and_modify(|data| data.in_view = Some(true)).or_insert(current.clone());
            current.el.as_ref().unwrap().class_list().add_1(&current.class).unwrap();
        }
        {
            core.as_ref().borrow().current_elements.as_ref().borrow_mut().data.entry(id.to_string()).and_modify(|el| {
                *el = current.clone();
            }).or_insert_with(|| current.clone());
        }


        if current.call.is_some() && core.as_ref().borrow().has_call_event_set {
            Core::dispatch_call(current, "enter", option, core.clone());

            if !*current.repeat.as_ref().unwrap() {
                core.as_ref().borrow().els.as_ref().borrow_mut().data.entry(id.to_string()).and_modify(|data| data.call = None);
            }
        }   
    }

    fn set_out_of_view(current: &MappedEl, id: &str, core: Rc<RefCell<Core>>, option: &LocomotiveOption) {
        core.as_ref().borrow().els.as_ref().borrow_mut().data.entry(id.to_string()).and_modify(|data| data.in_view = Some(false));
        
        core.as_ref().borrow().current_elements.as_ref().borrow_mut().data.remove(id).unwrap();

        if current.call.is_some() && core.as_ref().borrow().has_call_event_set {
            Core::dispatch_call(current, "exit", option, core.clone());
        }

        if *current.repeat.as_ref().unwrap() {
            let _ =current.el.as_ref().unwrap().class_list().remove_1(&current.class);
        }
    }

    fn dispatch_call(current: &MappedEl, way: &str, option: &LocomotiveOption, core: Rc<RefCell<Core>>) {
        {
            *core.as_ref().borrow().call_way.as_ref().borrow_mut() = way.to_string();
            *core.as_ref().borrow().call_value.as_ref().borrow_mut() = current.call.as_ref().unwrap().split(",").map(|s| s.trim().to_string()).collect::<Vec<String>>();
            *core.as_ref().borrow().call_obj.as_ref().borrow_mut() = Some(current.clone());
        }

        let call_event = Event::new(&format!("{}call", core.as_ref().borrow().namespace.as_ref().borrow())).unwrap();
        match &option.el {
            ElementType::Document(doc) => doc.dispatch_event(&call_event).unwrap(),
            ElementType::Element(el) => el.dispatch_event(&call_event).unwrap(),
        };
    }

    pub fn check_scroll(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        Core::dispatch_scroll(core.clone(), options);
    }

    pub fn dispatch_scroll(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let event = Event::new(&format!("{}scroll", core.as_ref().borrow().namespace.as_ref().borrow())).unwrap();
        options.el.dispatch_event(&event);
    }
}

/*
impl Core {

    fn update_elements(core: Rc<RefCell<Core>>) {
        {   
            let core_ref = core.as_ref().borrow();
            for (id, el) in core_ref.els.as_ref().clone().into_inner().clone().data {
                let top = el.target_el.as_ref().unwrap().get_bounding_client_rect().top() + core_ref.instance.as_ref().borrow().scroll.y;
                let html_el = el.target_el.as_ref().unwrap().dyn_ref::<HtmlElement>().unwrap();
                let bottom = top + html_el.offset_height() as f64;
                let relative_offset = Core::get_relative_offset(el.offset, core.as_ref().borrow().window_height);

                core_ref.els.as_ref().borrow_mut().data.entry(id).and_modify(|data| {
                    data.top = top + relative_offset[0];
                    data.bottom = bottom - relative_offset[1];
                });

            }
        }
        
        *core.as_ref().borrow().has_scroll_ticking.borrow_mut() = false;
        
    }
}


impl Core {
    fn get_relative_offset(offset: Vec<String>, window_height: f64) -> [f64; 2] {
        let mut res = [0.0, 0.0];

        if offset.len() > 2 {
            panic!("offset must be in type of 'x, y' only");
        }

        for (index, off_set) in offset.iter().enumerate() {
            let val = match off_set.contains("%") {
                true => {
                    let float = off_set.replace("%", "").parse::<f64>().unwrap();
                    (float * window_height) / 100.0
                },
                false => {
                    off_set.parse::<f64>().unwrap()
                }
            };
            res[index] = val
        }

        res
    }
}
*/


//GETTER
impl Core {
    pub fn get_check_resize(core: Rc<RefCell<Core>>) -> Rc<RefCell<Option<Closure<dyn FnMut()>>>>{
        core.as_ref().borrow().check_resize.clone()
    }

    pub fn get_html(core: Rc<RefCell<Core>>) -> Rc<RefCell<Element>> {
        core.as_ref().borrow().html.clone()
    }
}