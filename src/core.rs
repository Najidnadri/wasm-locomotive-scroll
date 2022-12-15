use std::{rc::Rc, cell::RefCell};

use wasm_bindgen::{JsCast, prelude::Closure, UnwrapThrowExt};
use web_sys::{Element, NodeList, window, Window, Event, HtmlElement, console};

use crate::{option::{LocomotiveOption, Position}, native::NativeScroll, Scroll, smooth::SmoothScroll, utils::{current_elements::CurrentElements, listeners::Listeners, els::{Els, MappedEl, ScrollToTarget, ScrollToOption}, instance::Instance, element_type::ElementType}};





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
    pub has_scroll_ticking: bool,
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

        //1
        if Self::check_smooth_scroll(&options) {
            if window.history().unwrap().scroll_restoration().is_err() {
                window.history().unwrap().set_scroll_restoration(web_sys::ScrollRestoration::Manual).unwrap()
            }
            window.scroll_to_with_x_and_y(0., 0.);
        }

        //2
        let html = window.document().unwrap().document_element().unwrap();
        let window_height = window.inner_height().unwrap().as_f64().unwrap();
        let window_width = window.inner_width().unwrap().as_f64().unwrap();
        let window_middle = Position {
            y: window_height / 2.,
            x: window_width / 2.
        };
        let current_elements = CurrentElements::new();
        let mut instance = Instance::new(&html, current_elements.clone());

        //3
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
        
        //4
        let direction_axis = if options.direction.as_str() == "horizontal" {
            'x'
        } else {
            'y'
        };

        //5
        if options.get_direction {
            instance.direction = None;
            instance.speed = Some(0.);
        }

        //6 create callback for checkScroll
        let element = options.el.clone();
        let check_scroll_cb = Closure::wrap(Box::new(move || {
            let element = element.clone();
            let event = Event::new("locomotivescroll").unwrap();
            match element {
                ElementType::Document(doc) => {
                    let _ = doc.dispatch_event(&event).unwrap();
                },
                ElementType::Element(el) => {
                    let _ = el.dispatch_event(&event).unwrap();
                }
            }
        }) as Box<dyn Fn()> );

        let html = Rc::new(RefCell::new(html));
        let instance = Rc::new(RefCell::new(instance));
        let direction_axis = Rc::new(RefCell::new(direction_axis));
       // html.class_list().add_1(&options.init_class).unwrap();

        let core = Core {
            namespace: Rc::new(RefCell::new("locomotive".to_string())),
            html: html.clone(),
            window_height, 
            window_width,
            window_middle,
            els: Rc::new(RefCell::new(Els::new())),
            current_elements: Rc::new(RefCell::new(current_elements)), 
            listeners: Listeners::new(),
            has_scroll_ticking: false,
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

        let scroll = Core::create_scroll(options.clone(), &window, check_scroll_cb, html.clone(), instance.clone(), direction_axis.clone(), core.clone());

        core.as_ref().borrow_mut().scroll = scroll;
        /* move to Smooth.rs
        core.check_scroll_callback(&options);
        core.check_resize_callback();
        core.check_event_callback();

        let check_resize_cb = core.check_resize.clone();
        let check_resize_cb = check_resize_cb.borrow();
        let check_resize_cb = check_resize_cb.as_ref().unwrap();
        window.add_event_listener_with_callback_and_bool("resize", check_resize_cb.as_ref().unchecked_ref(), false).unwrap();  
        */      
        core

    }
}

impl Core {
    pub fn init(core: Rc<RefCell<Core>>) {
        //self.scroll.as_mut().unwrap().init(self.html.clone(), instance);
        let scroll = core.as_ref().clone().into_inner().clone().scroll;
        let options = scroll.get_option();
        match &scroll {
            Scroll::Native(_) => Core::init_native(core.clone(), &scroll),
            Scroll::Smooth(_scroll) => {
                SmoothScroll::init(core.clone(), &options);
            },
            _ => todo!()
        }

        Core::init_events(core, &scroll);
    }


    fn create_scroll(options: LocomotiveOption, window: &Window, check_scroll_cb: Closure<dyn Fn()>, html: Rc<RefCell<Element>>, instance: Rc<RefCell<Instance>>, direction_axis: Rc<RefCell<char>>, core: Rc<RefCell<Core>>) -> Scroll {
    //Setup the `Scroll` Field
        console::log_1(&format!("{:?}", options).into());
        if (options.smooth && !options.is_mobile)  || 
            (options.tablet.as_ref().unwrap().smooth && options.is_tablet) ||
            (options.smartphone.as_ref().unwrap().smooth && options.is_mobile && !options.is_tablet) 
            {
                let scroll = SmoothScroll::new(options, html, instance, direction_axis, core);
                Scroll::Smooth(scroll)
        } else {
                let scroll = NativeScroll::new(options, window, check_scroll_cb);
                Scroll::Native(scroll)
        }
    }

    fn check_smooth_scroll(options: &LocomotiveOption) -> bool {
        if (options.smooth && !options.is_mobile)  || 
        (options.tablet.as_ref().unwrap().smooth && options.is_tablet) ||
        (options.smartphone.as_ref().unwrap().smooth && options.is_mobile && !options.is_tablet) {
            true
        } else {
            false
        }
    }

}

//INIT FUNCTIONS
impl Core {
    pub fn scroll_to(scroll: Scroll, target: ScrollToTarget, scroll_to_option: ScrollToOption, html: &Element, instance: Rc<RefCell<Instance>>) {
        //self.scroll.as_ref().unwrap().scroll_to(target_el, attr, None);
        match scroll {
            Scroll::Native(_) => {
                NativeScroll::scroll_to(target, scroll_to_option, html, instance);
            },
            Scroll::Smooth(_) => (),
            _ => todo!()
        }
    }

    pub fn init_events(core: Rc<RefCell<Core>>, scroll: &Scroll) {
        let option = scroll.get_option();
        {
            core.as_ref().borrow_mut().scroll_to_els = option.el.query_selector_all(&format!("[data-{}-to", option.name));
        }
        
        //self.set_scroll_to = 
        Core::set_scroll_to_callback(core.clone(), &option);

        if let Some(node_list) = core.as_ref().borrow().scroll_to_els.as_ref() {
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



//EVENT LISTENERS INIT
impl Core {
    pub fn check_scroll_callback(core: Rc<RefCell<Core>>) {
        
        let core2 = core.clone();

        let callback: Rc<RefCell<Option<Closure<dyn FnMut() >>>> = Rc::new(RefCell::new(None));
        *callback.borrow_mut() = Some(Closure::new(move || {
            let core = core.clone();
            let option = core.as_ref().borrow().scroll.get_option();
            let element = option.el.clone();
            let namespace = &core.as_ref().borrow().namespace;


            let scroll_event = Event::new(&format!("{}scroll", namespace.as_ref().borrow())).unwrap();
            element.dispatch_event(&scroll_event);
        }));
        core2.as_ref().borrow_mut().check_scroll = callback;

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
            console::log_1(&"update elements".into());
            if !_resize_tick {
                {
                    *core.as_ref().borrow().resize_tick.as_ref().borrow_mut() = true;
                }
                let callback = Closure::wrap(Box::new(move || {
                    //if core.as_ref().borrow()
                    Core::resize(core.clone());
                    *core.as_ref().borrow().resize_tick.as_ref().borrow_mut() = false;
                }) as Box<dyn Fn()>);
                window().unwrap().request_animation_frame(callback.as_ref().unchecked_ref()).unwrap();
                callback.forget();
            }   
        }));

        core2.as_ref().borrow_mut().check_resize = callback;
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
        core2.as_ref().borrow_mut().check_event = callback;
    }

    fn set_scroll_to_callback(core: Rc<RefCell<Core>>, option: &LocomotiveOption) {
        let callback: Rc<RefCell<Option<Closure<dyn FnMut(Event) >>>> = Rc::new(RefCell::new(None));
        let name = option.name.clone();
        let core = core.clone();
        let core2 = core.clone();


        *callback.borrow_mut() = Some(Closure::new(move |event: Event| {
            let core = core.clone();
            let html = core.as_ref().borrow();
            let html = &html.html;
            let html = html.clone();
            let html = html.as_ref().borrow();
            let instance = core.as_ref().borrow().instance.clone();
            let scroll = core.as_ref().borrow().scroll.clone();
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
                duration: None
            };

            Core::scroll_to(scroll.clone(), target, option, &html, instance);
        }));

        core2.as_ref().borrow_mut().set_scroll_to = callback;
    }
}







//NATIVE SCROLL
impl Core {
    fn init_native(core: Rc<RefCell<Core>>, scroll: &Scroll) {
        let option = scroll.get_option();
        let window = window().unwrap();
        {
            core.as_ref().borrow_mut().instance.as_ref().borrow_mut().scroll.y = window.page_y_offset().unwrap();
        }


        //add_element
        Core::add_element(&option, core.clone(), &option);
        //self.add_elements(instance, window_height, els, current_elements, has_call_event_set, call_way, call_value, call_object, namespace)

        //detect element
        Core::detect_elements(None, &option, core.clone());

        //super.init() AKA init core


    }
}


impl Core {
    fn add_element(options: &LocomotiveOption, core: Rc<RefCell<Core>>, option: &LocomotiveOption) {
        
        //let options = self.scroll.as_ref().unwrap().get_option();
        {
            core.as_ref().borrow().els.as_ref().borrow_mut().data.clear();
        }

        let elements = match &options.el {
            ElementType::Document(doc) => doc.query_selector_all(&format!("[data-{}]", options.name)).unwrap(),
            ElementType::Element(el) => el.query_selector_all(&format!("[data-{}]", options.name)).unwrap()
        };
        for i in 0 .. elements.length() {
            let element = elements.get(i).unwrap();
            let html_element = element.dyn_ref::<HtmlElement>().unwrap();
            let element = element.dyn_ref::<Element>().unwrap();
            let dataset = html_element.dataset();

            let _rect = element.get_bounding_client_rect();
            let class = dataset.get(&format!("{}Class", options.name)).unwrap_or(options.class.clone());
            let id = dataset.get(&format!("{}Id", options.name)).unwrap_or(i.to_string());
            let offset = match dataset.get(&format!("{}Offset", options.name)) {
                Some(attr) => attr.split(",").map(|str| str.to_string()).collect::<Vec<String>>(),
                None => options.offset.iter().map(|float| float.to_string()).collect::<Vec<String>>(),
            };
            let repeat = dataset.get(&format!("{}Repeat", options.name));
            let call = dataset.get(&format!("{}Call", options.name));

            let target = dataset.get(&format!("{}Target", options.name));
            let target_el = match &target {
                Some(attr) => {
                    window().unwrap().document().unwrap().query_selector(&attr).unwrap().expect_throw("cannot find target element, WasmLocomotiveScroll")
                },
                None => element.clone()
            };
            let target_html_el = target_el.dyn_ref::<HtmlElement>().unwrap();
            
            let target_e1bcr = target_el.get_bounding_client_rect();
            let top = target_e1bcr.top() + core.as_ref().borrow().instance.as_ref().borrow().scroll.y;
            let left = target_e1bcr.left() + core.as_ref().borrow().instance.as_ref().borrow().scroll.x;
            let bottom = top + target_html_el.offset_height() as f64;
            let right = left + target_html_el.offset_width() as f64;

            let repeat = match &repeat {
                Some(attr) => if attr == "false" {false} else {options.repeat},
                None => true,
            };
            
            let relative_offset = Core::get_relative_offset(offset.clone(), core.as_ref().borrow().window_height);
            let top = top + relative_offset[0];
            let bottom = bottom - relative_offset[1];

            let mapped_el = MappedEl {
                el: element.clone(),
                target_el,
                id: id.clone(),
                class: class.clone(),
                top,
                bottom,
                left,
                right,
                offset,
                progress: 0.0,
                repeat,
                in_view: false,
                call
            };
            {
                core.as_ref().borrow().els.as_ref().borrow_mut().data.entry(id.clone()).or_insert_with(|| mapped_el.clone());
            }

            if element.class_list().contains(&class) {
                Core::set_in_view(&mapped_el, &id, core.clone(), option);
            }
        }
    }

    fn detect_elements(has_call_event_set: Option<bool>, options: &LocomotiveOption, cores: Rc<RefCell<Core>>) {
        {
            let core = cores.clone();
            let core = core.as_ref().borrow();
            let scroll_top = core.instance.as_ref().borrow().scroll.y;
            let scroll_bottom = scroll_top + core.window_height;

            let scroll_left = core.instance.as_ref().borrow().scroll.x;
            let scroll_right = scroll_left + core.window_width;

            for (id, el) in core.els.as_ref().clone().into_inner().clone().data {
                if !el.in_view || has_call_event_set == Some(true) {

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

                if el.in_view {
                    if options.direction.as_str() == "horizontal" {
                        let width = el.right - el.left;
                        let scroll_x = core.instance.as_ref().borrow().scroll.x;
                        let new_progress = (scroll_x - (el.left - core.window_width)) / (width / core.window_width);
                        core.els.as_ref().borrow_mut().data.entry(id.clone()).and_modify(|data| {
                            data.progress = new_progress
                        });

                        if scroll_right < el.left || scroll_left > el.right {
                            Core::set_out_of_view(&el, &id, cores.clone(), options);
                        }
                    } else {
                        let height = el.bottom - el.top;
                        let scroll_y = core.instance.as_ref().borrow().scroll.y;
                        let new_progress = (scroll_y - (el.top - core.window_height)) / (height / core.window_height);
                        core.els.as_ref().borrow_mut().data.entry(id.clone()).and_modify(|data| {
                            data.progress = new_progress
                        });

                        if scroll_bottom < el.top || scroll_top > el.bottom {
                            Core::set_out_of_view(&el, &id, cores.clone(), options);
                        }
                    }
                }
            }
        }
    

        cores.as_ref().borrow_mut().has_scroll_ticking = false;

    }

    fn resize(core: Rc<RefCell<Core>>) {
        let mut _empty = true;
        {
            _empty = core.as_ref().borrow().els.as_ref().borrow().data.is_empty();
        }
        if !_empty {
            {
                core.as_ref().borrow_mut().window_height = window().unwrap().inner_height().unwrap().as_f64().unwrap();
            }
            Core::update_elements(core.clone())
        }
    }
}


impl Core {
    fn set_in_view(current: &MappedEl, id: &str, core: Rc<RefCell<Core>>, option: &LocomotiveOption) {
        
        {
            core.as_ref().borrow().els.as_ref().borrow_mut().data.entry(id.to_string()).and_modify(|data| data.in_view = true).or_insert(current.clone());
            current.el.class_list().add_1(&current.class).unwrap();
        }
        {
            core.as_ref().borrow().current_elements.as_ref().borrow_mut().data.entry(id.to_string()).and_modify(|el| {
                *el = current.clone();
            }).or_insert_with(|| current.clone());
        }


        if current.call.is_some() && core.as_ref().borrow().has_call_event_set {
            Core::dispatch_call(current, "enter", option, core.clone());

            if !current.repeat {
                core.as_ref().borrow().els.as_ref().borrow_mut().data.entry(id.to_string()).and_modify(|data| data.call = None);
            }
        }   
    }

    fn set_out_of_view(current: &MappedEl, id: &str, core: Rc<RefCell<Core>>, option: &LocomotiveOption) {
        core.as_ref().borrow().els.as_ref().borrow_mut().data.entry(id.to_string()).and_modify(|data| data.in_view = false);
        
        core.as_ref().borrow().current_elements.as_ref().borrow_mut().data.remove(id).unwrap();

        if current.call.is_some() && core.as_ref().borrow().has_call_event_set {
            Core::dispatch_call(current, "exit", option, core.clone());
        }

        if current.repeat {
            let _ =current.el.class_list().remove_1(&current.class);
        }
    }

    fn update_elements(core: Rc<RefCell<Core>>) {
        {   
            let core_ref = core.as_ref().borrow();
            for (id, el) in core_ref.els.as_ref().clone().into_inner().clone().data {
                let top = el.target_el.get_bounding_client_rect().top() + core_ref.instance.as_ref().borrow().scroll.y;
                let html_el = el.target_el.dyn_ref::<HtmlElement>().unwrap();
                let bottom = top + html_el.offset_height() as f64;
                let relative_offset = Core::get_relative_offset(el.offset, core.as_ref().borrow().window_height);

                core_ref.els.as_ref().borrow_mut().data.entry(id).and_modify(|data| {
                    data.top = top + relative_offset[0];
                    data.bottom = bottom - relative_offset[1];
                });

            }
        }
        
        core.as_ref().borrow_mut().has_scroll_ticking = false;
        
    }
}


impl Core {
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




// SMOOTH SCROLL
impl Core {
    fn init_smooth(core: Rc<RefCell<Core>>, scroll: &Scroll) {
        let option = scroll.get_option();
        let window = window().unwrap();
        {
            core.as_ref().borrow().html.as_ref().borrow().class_list().add_1(&option.smooth_class).unwrap();
            core.as_ref().borrow().html.as_ref().borrow().set_attribute(&format!("data-{}-direction", &option.name), &option.direction).unwrap();
        }

        {
            {
                core.as_ref().borrow().instance.as_ref().borrow_mut().set_delta(option.init_position.clone());
            }
            {
                core.as_ref().borrow().instance.as_ref().borrow_mut().set_scroll(option.init_position.clone());
            }
        }

        {

        }
    }
}


//GETTER
impl Core {
    pub fn get_check_resize(core: Rc<RefCell<Core>>) -> Rc<RefCell<Option<Closure<dyn FnMut()>>>>{
        core.as_ref().borrow().check_resize.clone()
    }

    pub fn get_html(core: Rc<RefCell<Core>>) -> Rc<RefCell<Element>> {
        core.as_ref().borrow().html.clone()
    }
}