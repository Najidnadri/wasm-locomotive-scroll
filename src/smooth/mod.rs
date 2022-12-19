mod callback;
mod utils;

use std::{rc::Rc, cell::RefCell};

use js_sys::{Date, Function};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, KeyboardEvent, window, WheelEvent, HtmlElement, Node, MouseEvent, DomRect};

use crate::{option::{LocomotiveOption, Position}, virtual_scroll::{VirtualScroll, VsOption},core::Core, utils::{instance::Instance, lerp, get_translate, get_parents, els::{MappedEl, ScrollToTarget, ScrollToOption}}, bezier_easing::bezier2};

pub use self::utils::{Sections, Section, ParallaxElements};



#[derive(Debug, Clone)]
pub struct SmoothScroll {
    pub options: LocomotiveOption,
    pub is_scrolling: Rc<RefCell<bool>>,
    pub is_dragging_scrollbar: Rc<RefCell<bool>>,
    pub is_ticking: bool,
    pub has_scroll_ticking: Rc<RefCell<bool>>,
    pub parallax_elements: Rc<RefCell<ParallaxElements>>,
    pub stop: Rc<RefCell<bool>>,
    pub scrollbar_container: bool,
    pub check_key: Rc<RefCell<Option<Closure<dyn FnMut(KeyboardEvent)>>>>,

    pub virtual_scroll: Option<VirtualScroll>,
    pub animating_scroll: Rc<RefCell<bool>>,

    //stop scrolling
    pub check_scroll_raf: Rc<RefCell<Option<i32>>>,
    pub start_scroll_ts: Rc<RefCell<Option<f64>>>,
    pub speed_ts: Rc<RefCell<Option<f64>>>,
    pub scroll_to_raf: Rc<RefCell<Option<i32>>>,

    //checkScroll
    pub sections: Rc<RefCell<Sections>>,

    //scrollbar
    pub scrollbar: Option<Element>,
    pub scrollbar_thumb: Option<Element>,
    pub get_scrollbar: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
    pub release_scrollbar: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
    pub move_scrollbar: Rc<RefCell<Option<Closure<dyn Fn(MouseEvent)>>>>,
    pub mouse_event: Rc<RefCell<Option<MouseEvent>>>,
    pub scrollbar_bcr: Rc<RefCell<Option<DomRect>>>,
    pub scrollbar_width: Rc<RefCell<Option<f64>>>,
    pub scrollbar_height: Rc<RefCell<Option<f64>>>,
    pub scrollbar_thumb_bcr: Rc<RefCell<Option<DomRect>>>,


    //Closures / Function
    check_key_cb_1: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
    pub check_key_cb_2: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
    pub vs_cb_1: Rc<RefCell<Option<Closure<dyn Fn(WheelEvent)>>>>,
    pub check_scroll_cb: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
    pub move_scrollbar_cb_2: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
    pub loop_cb: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
}

impl SmoothScroll {
    pub fn new(mut options: LocomotiveOption, core: Rc<RefCell<Core>>) -> Self {

        //1
        if let Some(inertia) = options.inertia {
            options.lerp = inertia * 0.1;
        }

        let scrollbar_container = options.scroll_bar_container;

        let mut smooth = Self {
            options: options.clone(),
            is_scrolling: Rc::new(RefCell::new(false)),
            is_dragging_scrollbar: Rc::new(RefCell::new(false)),
            is_ticking: false,
            has_scroll_ticking: Rc::new(RefCell::new(false)),
            parallax_elements: Rc::new(RefCell::new(ParallaxElements::new())),
            stop: Rc::new(RefCell::new(false)),
            scrollbar_container,
            check_key: Rc::new(RefCell::new(None)),

            virtual_scroll: None,
            animating_scroll: Rc::new(RefCell::new(false)),

            check_scroll_raf: Rc::new(RefCell::new(None)),
            start_scroll_ts: Rc::new(RefCell::new(None)),
            speed_ts: Rc::new(RefCell::new(None)),
            scroll_to_raf: Rc::new(RefCell::new(None)),
            sections: Rc::new(RefCell::new(Sections::new())),

            scrollbar: None,
            scrollbar_thumb: None,
            get_scrollbar: Rc::new(RefCell::new(None)),
            release_scrollbar: Rc::new(RefCell::new(None)),
            move_scrollbar: Rc::new(RefCell::new(None)),
            mouse_event: Rc::new(RefCell::new(None)),
            scrollbar_bcr: Rc::new(RefCell::new(None)),
            scrollbar_width: Rc::new(RefCell::new(None)),
            scrollbar_height: Rc::new(RefCell::new(None)),
            scrollbar_thumb_bcr: Rc::new(RefCell::new(None)),

            check_key_cb_1: Rc::new(RefCell::new(None)),
            check_key_cb_2: Rc::new(RefCell::new(None)),
            vs_cb_1: Rc::new(RefCell::new(None)),
            check_scroll_cb: Rc::new(RefCell::new(None)),
            move_scrollbar_cb_2: Rc::new(RefCell::new(None)),
            loop_cb: Rc::new(RefCell::new(None)),
        };

        smooth.check_key_cb_1(core.clone());
        smooth.check_key_cb_2(core.clone());
        smooth.check_key_callback(core.clone());
        smooth.vs_cb_1(core.clone(), &options);
        smooth.check_scroll_cb(core.clone(), &options);
        smooth.get_scrollbar(core.clone(), &options);
        smooth.release_scrollbar_cb(core.clone(), &options);
        smooth.move_scrollbar_cb(core.clone());
        

        smooth
    }


    pub fn init(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        {
            let _ = core.as_ref().borrow().html.as_ref().borrow().class_list().add_1(&options.smooth_class);
            let _ = core.as_ref().borrow().html.as_ref().borrow().set_attribute(&format!("data-{}-direction", options.name), &options.direction);
        }
        {  
            {
                core.as_ref().borrow().instance.as_ref().borrow_mut().set_delta(options.init_position.clone());
            }
            {
                core.as_ref().borrow().instance.as_ref().borrow_mut().set_scroll(options.init_position.clone());
            }
        }
        {
            let el = if options.scroll_from_anywhere {
                window().unwrap().document().unwrap().dyn_into::<JsValue>().unwrap()
            } else {
                options.el.clone().to_js()
            };
            let mouse_multiplier = if window().unwrap().navigator().platform().unwrap().contains("Win") {1.0} else {0.4};
            let vs_option = VsOption {
                el,
                mouse_multiplier,
                firefox_multiplier: options.firefox_multiplier,
                touch_multiplier: options.touch_multiplier,
                use_keyboard: false,
                passive: true,
            };
            core.as_ref().borrow_mut().scroll.set_virtual_scroll(vs_option);
        }
        {
            core.as_ref().borrow().scroll.set_vs_event_listener()
        }
        {
            Self::set_scroll_limit(core.clone(), options);
        }
        {
            Self::init_scroll_bar(core.clone(), options);
        }
        {
            Self::add_sections(core.clone(), options);
        }
        {
            Self::add_elements(core.clone(), options);
        }

        {
            Self::check_scroll(Some(true), core.clone(), options.clone());
        }
        {
            Self::transform_elements(Some(true), Some(true), core.clone(), options)
        }

    }
}



impl SmoothScroll {
    fn set_scroll_limit(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let ref_core = core.as_ref().borrow();
        let window_width = ref_core.window_width;
        let mut instance = ref_core.instance.as_ref().borrow_mut();

        if options.direction.as_str() == "horizontal" {
            let mut total_width = 0;
            let nodes = options.el.children();
            for i in 0 .. nodes.length() {
                let el = nodes.get_with_index(i).unwrap().dyn_into::<HtmlElement>().unwrap();
                let offset = el.offset_width();
                total_width += offset;
            }

            instance.limit.x = total_width as f64 - window_width;
        }
    }

    fn init_scroll_bar(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let window = window().unwrap();
        let doc = window.document().unwrap();
        let core = core.clone();
        let core_ref = core.as_ref().borrow();
        let scroll = core_ref.scroll.get_smooth();
        let instance = core_ref.instance.clone();

        let scrollbar = doc.create_element("span").unwrap();
        let scrollbar_thumb = doc.create_element("span").unwrap();
        scrollbar.class_list().add_1(&options.scroll_bar_class).unwrap();
        scrollbar_thumb.class_list().add_1(&format!("{}_thumb", &options.scroll_bar_class)).unwrap();

        scrollbar.append_with_node_1(scrollbar_thumb.dyn_ref::<Node>().unwrap()).unwrap();

        //todo..append to scrollbar container if exist
        //...

        doc.body().unwrap().append_with_node_1(scrollbar.dyn_ref::<Node>().unwrap()).unwrap();
        {
            let mut scroll = core.as_ref().borrow_mut();
            scroll.scroll.set_scrollbar(scrollbar, scrollbar_thumb.clone());
        }

        //scrollbar events
        scrollbar_thumb.add_event_listener_with_callback("mousedown", scroll.get_scrollbar.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
        window.add_event_listener_with_callback("mouseup", scroll.release_scrollbar.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
        window.add_event_listener_with_callback("mousemove", scroll.move_scrollbar.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();

        //set scroll bar values
        {
            *core_ref.has_scroll_bar.borrow_mut().as_mut().unwrap() = false;
        }
        if options.direction.as_str() == "horizontal" {
            if instance.borrow().limit.x + core.borrow().window_width <= core.borrow().window_width {
                return;
            }
        } else {
            if instance.borrow().limit.y + core.borrow().window_height <= core.borrow().window_height {
                return;
            }
        }
        {
            *core_ref.has_scroll_bar.borrow_mut().as_mut().unwrap() = true;
        }

        {
            *scroll.scrollbar_bcr.borrow_mut() = Some(scroll.scrollbar.as_ref().unwrap().get_bounding_client_rect());
        }
        {
            *scroll.scrollbar_height.borrow_mut() = Some(scroll.scrollbar_bcr.borrow().as_ref().unwrap().height());
            *scroll.scrollbar_width.borrow_mut() = Some(scroll.scrollbar_bcr.borrow().as_ref().unwrap().width());
        }
        {
            if options.direction.as_str() == "horizontal" {
                let style = scroll.scrollbar_thumb.as_ref().unwrap().dyn_ref::<HtmlElement>().unwrap().style();
                let scrollbar_width = scroll.scrollbar_width.borrow();
                let scrollbar_width = scrollbar_width.as_ref().unwrap();
                let width = (scrollbar_width * scrollbar_width) / instance.borrow().limit.x + scrollbar_width;
                style.set_property("width", &format!("{:?}px", width)).unwrap();
            } else {
                let style = scroll.scrollbar_thumb.as_ref().unwrap().dyn_ref::<HtmlElement>().unwrap().style();
                let scrollbar_height = scroll.scrollbar_height.borrow();
                let scrollbar_height = scrollbar_height.as_ref().unwrap();
                let width = (scrollbar_height * scrollbar_height) / instance.borrow().limit.y + scrollbar_height;
                style.set_property("height", &format!("{:?}px", width)).unwrap();
            }
        }

        {
            *scroll.scrollbar_thumb_bcr.borrow_mut() = Some(scroll.scrollbar_thumb.as_ref().unwrap().get_bounding_client_rect());
        }
        {
            let x = scroll.scrollbar_width.borrow().as_ref().unwrap() - scroll.scrollbar_thumb_bcr.borrow().as_ref().unwrap().width();
            let y = scroll.scrollbar_height.borrow().as_ref().unwrap() - scroll.scrollbar_thumb_bcr.borrow().as_ref().unwrap().height();
            *core.borrow().scroll_bar_limit.borrow_mut() = Position::new(x, y);
        }

    }

    fn add_sections(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let mut core_ref = core.as_ref().borrow_mut();
        let scroll = core_ref.scroll.get_mut_smooth();
        {
            scroll.sections.borrow_mut().clear();
        }

        let sections = options.el.query_selector_all(&format!("[data-{}-section]", options.name)).unwrap();
        let sections = if sections.length() == 0 {
            vec![options.el.get_element().clone()]
        } else {
            let mut res = vec![];
            for i in 0 .. sections.length() {
                let node = sections.get(i).unwrap();
                res.push(node.dyn_into::<Element>().unwrap());
            }
            res
        };
    
        for (index, section) in sections.into_iter().enumerate() {
            let html_element = section.dyn_ref::<HtmlElement>().unwrap();
            let id = if let Some(id) = html_element.dataset().get(format!("{}Id", options.name).as_str()) {
                id.to_string()
            } else {
                format!("section{}", index)
            };
            let section_bcr = section.get_bounding_client_rect();
            let offset = Position {
                x: section_bcr.left() - window().unwrap().inner_width().unwrap().as_f64().unwrap() * 1.5 - get_translate(&section).unwrap().x,
                y: section_bcr.top() - window().unwrap().inner_height().unwrap().as_f64().unwrap() * 1.5 - get_translate(&section).unwrap().y,
            };
            let limit = Position {
                x: offset.x + section_bcr.width() + window().unwrap().inner_width().unwrap().as_f64().unwrap() * 2.0,
                y: offset.y + section_bcr.height() + window().unwrap().inner_height().unwrap().as_f64().unwrap() * 2.0,
            };
            let persistent = if let Some(persistent) = html_element.dataset().get(format!("{}Persistent", options.name).as_str()) {
                persistent == "string"
            } else {
                false
            };
            section.set_attribute("data-scroll-section-id", id.as_str()).unwrap();
    
            let mapped_section = Section {
                el: section,
                offset,
                limit,
                in_view: false,
                persistent: Some(persistent),
                id: id.clone(),
            };
    
            scroll.sections.borrow_mut().data.entry(id).or_insert_with(|| Rc::new(RefCell::new(mapped_section)));
        }
    }

     
    fn add_elements(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let core_ref = core.as_ref().borrow();
        let scroll = core_ref.scroll.get_smooth();
        let sections = scroll.sections.clone();
        let instance = core_ref.instance.clone();
        let direction_axis = core_ref.direction_axis.clone().as_ref().clone().into_inner();
        {
            core_ref.els.borrow_mut().data.clear();
            scroll.parallax_elements.borrow_mut().data.clear();
        }

        let els = options.el.query_selector_all(&format!("[data-{}]", options.name)).unwrap();

        for index in 0 .. els.length() {
            let node = els.get(index).unwrap();
            let el = node.dyn_ref::<Element>().unwrap();
            let html_el = el.dyn_ref::<HtmlElement>().unwrap();
            let dataset = html_el.dataset();
            let parents = get_parents(el.clone());
            let sections = sections.clone();
            let sections = sections.borrow();

            let section = sections
                .data
                .values()
                .find(|section| parents.contains(&section.borrow().el));

            let cl = dataset.get(&format!("{}Class", options.name)).unwrap_or(options.class.clone());
            let id = if let Some(el_id) = dataset.get(&format!("{}Id", options.name)) {
                el_id
            } else {
                format!("el{}", index.to_string())
            };
            let repeat = match dataset.get(&format!("{}Repeat", options.name)) {
                Some(val) => {
                    match val.as_str().trim() == "false" {
                        true => false,
                        false => true
                    }
                },
                None => options.repeat
            };
            let call = dataset.get(&format!("{}Call", options.name));
            let position = dataset.get(&format!("{}Position", options.name));
            let delay = dataset.get(&format!("{}Delay", options.name));
            let direction = dataset.get(&format!("{}Direction", options.name));
            let sticky = dataset.get(&format!("{}Sticky", options.name));
            let speed = if let Some(val) = dataset.get(&format!("{}Speed", options.name)) {
                Some(val.parse::<f64>().unwrap() / 10.0)
            } else {
                None
            };
            let offset = if let Some(val) = dataset.get(&format!("{}Offset", options.name)) {
                val.split(",").map(|s| s.trim().to_string()).collect::<Vec<String>>()
            } else {
                vec![options.offset[0].to_string(), options.offset[1].to_string()]
            };
            let target = dataset.get(&format!("{}Target", options.name));
            let target_el = match target {
                Some(val) => window().unwrap().document().unwrap().query_selector(&val).unwrap().unwrap(),
                None => el.clone()
            };
            let target_el_html = target_el.dyn_ref::<HtmlElement>().unwrap();

            let target_el_bcr = target_el.get_bounding_client_rect();
            let (mut top, mut left) = match section {
                Some(sect) => {
                    match !sect.borrow().in_view.clone() {
                        true => {
                            let top = target_el_bcr.top() - get_translate(&sect.borrow().el).unwrap().y + get_translate(&target_el).unwrap().y;
                            let left = target_el_bcr.left() - get_translate(&sect.borrow().el).unwrap().x + get_translate(&target_el).unwrap().x;
                            (top, left)
                        },
                        false => {
                            let top = target_el_bcr.top() + instance.borrow().scroll.y - get_translate(&target_el).unwrap().y;
                            let left = target_el_bcr.left() + instance.borrow().scroll.x - get_translate(&target_el).unwrap().x;
                            (top, left)
                        }
                    }
                }, 
                None => {
                    let top = target_el_bcr.top() + instance.borrow().scroll.y - get_translate(&target_el).unwrap().y;
                    let left = target_el_bcr.left() + instance.borrow().scroll.x - get_translate(&target_el).unwrap().x;
                    (top, left)
                }
            };

            let mut bottom = top + target_el_html.offset_height() as f64;
            let mut right = left + target_el_html.offset_width() as f64;
            let mut middle = Position {
                x: (right - left) / 2.0 + left,
                y: (bottom - top) / 2.0 + top
            };

            if sticky.is_some() {
                let el_bcr = el.get_bounding_client_rect();
                let el_top = el_bcr.top();
                let el_left = el_bcr.left();

                let el_distance = Position {
                    x: el_left - left,
                    y: el_top - top,
                };

                top += window().unwrap().inner_height().unwrap().as_f64().unwrap();
                left += window().unwrap().inner_width().unwrap().as_f64().unwrap();
                bottom = el_top + target_el_html.offset_height() as f64 - html_el.offset_height() as f64 - el_distance.get(direction_axis);
                right = el_left + target_el_html.offset_width() as f64 - html_el.offset_width() as f64 - el_distance.get(direction_axis);
                middle = Position {
                    x: (right - left) / 2.0 + left,
                    y: (bottom - top) / 2.0 + top,
                };

            }

            let mut relative_offset = [0.0, 0.0];
            if options.direction.as_str() == "horizontal" {
                for (index, val) in offset.iter().enumerate() {
                    let val_num = if val.contains("%") {
                        (val.replace("%", "").parse::<f64>().unwrap() * core_ref.window_width) / 100.0
                    } else {
                        val.parse::<f64>().unwrap()
                    };
                    relative_offset[index] = val_num;
                } 
                left = left + relative_offset[0];
                right = right - relative_offset[1];
            } else {
                for (index, val) in offset.iter().enumerate() {
                    let val_num = if val.contains("%") {
                        (val.replace("%", "").parse::<f64>().unwrap() * core_ref.window_width) / 100.0
                    } else {
                        val.parse::<f64>().unwrap()
                    };
                    relative_offset[index] = val_num;
                } 
                top = top + relative_offset[0];
                bottom = bottom - relative_offset[1]; 
            }

            let mapped_el = MappedEl {
                el: Some(el.clone()),
                target_el: Some(target_el),
                id: id.clone(),
                class: cl.clone(),
                top,
                bottom,
                left,
                right,
                offset,
                progress: Some(0.0),
                repeat: Some(repeat),
                in_view: Some(false),
                call,
                section: Some(section.unwrap().clone()),
                middle: Some(middle),
                speed,
                delay,
                position,
                direction,
                sticky: sticky.clone(),
            };
            {
                core_ref.els.clone().borrow_mut().data.entry(id.clone()).or_insert(mapped_el.clone());
            }
            if el.class_list().contains(&cl) {
                Core::set_in_view(&mapped_el, &id, core.clone(), options);
            }
            if speed != None || sticky.is_some() {
                {
                    scroll.parallax_elements.borrow_mut().data.entry(id).and_modify(|data| data.overwrite(mapped_el.clone())).or_insert(mapped_el);
                }
            }
        }
    }
    
}





impl SmoothScroll {
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

    pub fn check_scroll(forced: Option<bool>, core: Rc<RefCell<Core>>, option: LocomotiveOption) {
        let forced = forced.unwrap_or(false);
        let ref_core = core.clone();
        let ref_core = &ref_core.as_ref().borrow().scroll;
        let scroll = ref_core.get_smooth();
        let is_scrolling = scroll.is_scrolling.clone();
        let is_dragging_scrollbar = scroll.is_dragging_scrollbar.clone();
        let has_scroll_ticking = scroll.has_scroll_ticking.clone();
        let instance = core.as_ref().borrow().instance.clone();
        let animating_scroll = scroll.animating_scroll.clone();
        let check_scroll_raf = scroll.check_scroll_raf.clone();
        let scroll_to_raf = scroll.scroll_to_raf.clone();
        let direction_axis = *core.as_ref().borrow().direction_axis.borrow();
        let start_scroll_ts = scroll.start_scroll_ts.clone();

        if forced || *is_scrolling.borrow() || *is_dragging_scrollbar.borrow() {
            {
                if !has_scroll_ticking.as_ref().clone().into_inner() {
                    let scroll_raf = window().unwrap().request_animation_frame(scroll.check_key_cb_1.as_ref().borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
                    *check_scroll_raf.borrow_mut() = Some(scroll_raf);
    
                    *has_scroll_ticking.borrow_mut() = true;
                }
            }

            SmoothScroll::update_scroll(core.clone(),  option.lerp);

            let (scroll_val, _limit, delta) = match direction_axis {
                'x' => (instance.borrow().scroll.x, instance.borrow().limit.x, instance.borrow().delta.as_ref().unwrap().x),
                'y' => (instance.borrow().scroll.y, instance.borrow().limit.y, instance.borrow().delta.as_ref().unwrap().y),
                _ => panic!()
            };
            let distance = (delta - scroll_val).abs();
            let time_since_start = Date::now() - start_scroll_ts.as_ref().clone().into_inner().as_ref().unwrap_or(&0.0);

            if !*animating_scroll.borrow() && time_since_start > 100.0 && 
            ((distance < 0.5 && delta != 0.0) || (distance < 0.5 && delta == 0.0)) {
                SmoothScroll::stop_scrolling(core.clone(), start_scroll_ts.clone(), check_scroll_raf.clone(), scroll_to_raf.clone(), is_scrolling.clone(), option.scrolling_class.clone());
            }

            for (_id, section) in &scroll.sections.as_ref().borrow().data {
                let mut section = section.as_ref().borrow_mut();
                let (offset, section_limit) = match direction_axis {
                    'x' => (section.offset.x, section.limit.x),
                    'y' => (section.offset.y, section.limit.y),
                    _ => panic!()
                };

                if section.persistent.is_some() || (scroll_val > offset && scroll_val < section_limit) {
                    match option.direction.as_str() {
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
                        section.el.set_attribute(&format!("data-{}-section-inview", &option.name), "").unwrap()
                    }
                } else {
                    if section.in_view || forced {
                        section.in_view = true;
                        let style = section.el.dyn_ref::<HtmlElement>().unwrap().style();
                        style.set_property("opacity", "0").unwrap();
                        style.set_property("pointerEvents", "none").unwrap();
                        section.el.remove_attribute(&format!("data-{}-section-inview", &option.name)).unwrap()
                    }

                    SmoothScroll::transform(section.el.clone(), Some(0.0), Some(0.0), None);
                }
            }
            
            if option.get_direction {
                SmoothScroll::add_direction(core.clone());
            }

            if option.get_speed {
                SmoothScroll::add_speed(core.clone(), scroll.speed_ts.clone(), direction_axis);
                {
                    *scroll.speed_ts.as_ref().borrow_mut() = Some(Date::now());
                }
            }

            Core::detect_elements(None, core.clone(), &option);
            SmoothScroll::transform_elements(None, None, core.clone(), &option);


            if core.as_ref().borrow().has_scroll_bar.borrow().is_some() {
                let (scroll_val, limit, scrollbar_limit) = match core.as_ref().borrow().direction_axis.as_ref().clone().into_inner() {
                    'x' => (instance.as_ref().borrow().scroll.x, instance.borrow().limit.x, core.as_ref().borrow().scroll_bar_limit.borrow().x),
                    'y' => (instance.as_ref().borrow().scroll.y, instance.borrow().limit.y, core.as_ref().borrow().scroll_bar_limit.borrow().y),
                    _ => panic!()
                };
                let scroll_bar_translation = ( scroll_val / limit ) * scrollbar_limit;
                if option.direction.as_str() == "horizontal" {
                    SmoothScroll::transform(scroll.scrollbar_thumb.as_ref().unwrap().clone(), Some(scroll_bar_translation), Some(0.0), None);
                } else {
                    SmoothScroll::transform(scroll.scrollbar_thumb.as_ref().unwrap().clone(), Some(0.0), Some(scroll_bar_translation), None);
                }
            }


            Core::check_scroll(core.clone(), &option);

            {
                *core.as_ref().borrow().has_scroll_ticking.borrow_mut() = false;
            }

        }
    }

    pub(crate) fn reinit_scrollbar(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let scroll = &core.borrow().scroll;
        let scroll = scroll.get_smooth();
        let instance = core.borrow().instance.clone();
        let limit = instance.borrow().limit.clone();
        {
            *core.borrow().has_scroll_bar.borrow_mut() = Some(false);
        }
        if options.direction.as_str() == "horizontal" {
            if limit.x + core.borrow().window_width <= core.borrow().window_width {
                return;
            }
        } else {
            if limit.y + core.borrow().window_height <= core.borrow().window_height {
                return;
            }
        }

        {
            *core.borrow().has_scroll_bar.borrow_mut() = Some(true);
        }

        let scrollbar_bcr = scroll.scrollbar.as_ref().unwrap().get_bounding_client_rect();
        {
            *scroll.scrollbar_bcr.borrow_mut() = Some(scrollbar_bcr.clone());
            *scroll.scrollbar_height.borrow_mut() = Some(scrollbar_bcr.height());
            *scroll.scrollbar_width.borrow_mut() = Some(scrollbar_bcr.width());
        }

        if options.direction.as_str() == "horizontal" {
            let html_scrollbar_thumb = scroll.scrollbar_thumb.as_ref().unwrap().dyn_ref::<HtmlElement>().unwrap();
            let style = html_scrollbar_thumb.style();
            let scrollbar_width = scroll.scrollbar_width.borrow();
            let scrollbar_width = scrollbar_width.as_ref().unwrap();
            let width = format!("{:?}px", (scrollbar_width * scrollbar_width) / (limit.x + scrollbar_width));
            style.set_property("width", &width).unwrap();
        } else {
            let html_scrollbar_thumb = scroll.scrollbar_thumb.as_ref().unwrap().dyn_ref::<HtmlElement>().unwrap();
            let style = html_scrollbar_thumb.style();
            let scrollbar_height = scroll.scrollbar_height.borrow();
            let scrollbar_height = scrollbar_height.as_ref().unwrap();
            let height = format!("{:?}px", (scrollbar_height * scrollbar_height) / (limit.y + scrollbar_height));
            style.set_property("width", &height).unwrap();
        }

        let scrollbar_thumb_bcr = scroll.scrollbar_thumb.as_ref().unwrap().get_bounding_client_rect();
        {
            *scroll.scrollbar_thumb_bcr.borrow_mut() = Some(scrollbar_thumb_bcr.clone());
        }
        {   
            let scrollbar_height = scroll.scrollbar_height.clone();
            let scrollbar_width = scroll.scrollbar_width.clone();
            *core.borrow().scroll_bar_limit.borrow_mut() = Position {
                x: scrollbar_width.borrow().as_ref().unwrap() - scrollbar_thumb_bcr.width(),
                y: scrollbar_height.borrow().as_ref().unwrap() - scrollbar_thumb_bcr.height()
            };
        }
    }

    pub fn add_direction(core: Rc<RefCell<Core>>) {
        let binding = core.as_ref().borrow();
        let mut instance = binding.instance.borrow_mut();
        let (down, up, left, right) = (String::from("down"), String::from("up"), String::from("left"), String::from("right"));
        
        if instance.delta.as_ref().unwrap().y > instance.scroll.y {
            if instance.direction != Some(down.clone()) {
                instance.direction = Some(down);
            } 
        } else if instance.delta.as_ref().unwrap().y < instance.scroll.y {
            if instance.direction != Some(up.clone()) {
                instance.direction = Some(up);
            }
        }

        if instance.delta.as_ref().unwrap().x > instance.scroll.x {
            if instance.direction != Some(right.clone()) {
                instance.direction = Some(right);
            } 
        } else if instance.delta.as_ref().unwrap().x < instance.scroll.x {
            if instance.direction != Some(left.clone()) {
                instance.direction = Some(left);
            }
        }
    }

    pub fn add_speed(core: Rc<RefCell<Core>>, speed_ts: Rc<RefCell<Option<f64>>>, direction_axis: char) {
        let binding = core.as_ref().borrow();
        let mut instance = binding.instance.borrow_mut();
        let (delta, scroll) = match direction_axis {
            'x' => (instance.delta.as_ref().unwrap().x, instance.scroll.x),
            'y' => (instance.delta.as_ref().unwrap().y, instance.scroll.y),
            _ => panic!()
        };

        if delta != scroll {
            let val = (delta - scroll) / (f64::max(1.0, Date::now() - *speed_ts.as_ref().borrow().as_ref().unwrap()));
            instance.speed = Some(val);
        } else {
            instance.speed = Some(0.0);
        }
    }

    pub fn transform_elements(is_forced: Option<bool>, set_all_elements: Option<bool>, core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let core = core.as_ref().borrow();
        let scroll = core.scroll.get_smooth();
        let direction_axis = core.direction_axis.clone().as_ref().clone().into_inner();
        let window_width = core.window_width;
        let instance = core.instance.as_ref().borrow_mut();
        let (scroll_val, limit, scroll_x, scroll_y) = match direction_axis {
            'x' => (instance.scroll.x, instance.limit.x, instance.scroll.x, instance.scroll.y),
            'y' => (instance.scroll.y, instance.limit.y, instance.scroll.x, instance.scroll.y),
            _ => panic!("direction axis not supported")
        };
        let parallax_elements = scroll.parallax_elements.clone();

        let scroll_right = instance.scroll.x + window_width;
        let scroll_bottom = instance.scroll.y + core.window_height;
        let scroll_middle = Position::new(instance.scroll.x + core.window_middle.x, instance.scroll.y + core.window_middle.y);

        for (_id, current) in parallax_elements.borrow().data.iter() {
            let mut _transform_distance = None;
            //let current = current.borrow_mut();

            if is_forced.is_some() {
                _transform_distance = Some(0.0);
            }

            if *current.in_view.as_ref().unwrap() || set_all_elements.is_some() {
                let speed = current.speed.unwrap_or(0.0);
                match current.position.as_ref().unwrap().as_str() {
                    "top" => {
                        _transform_distance = Some(scroll_val * (-speed));
                    },
                    "elementTop" => {
                        _transform_distance = Some((scroll_bottom - current.top) * speed);
                    },
                    "bottom" => {
                        _transform_distance = Some((limit - scroll_bottom + core.window_height) * speed);
                    },
                    "left" => {
                        _transform_distance = Some(scroll_val * -speed);
                    },
                    "elementLeft" => {
                        _transform_distance = Some((scroll_right - current.left) * -speed);
                    },
                    "right" => {
                        _transform_distance = Some((limit - scroll_right + core.window_height) * speed);
                    },
                    _ => {
                        let (current_middle, scroll_middle) = match direction_axis {
                            'x' => (current.middle.as_ref().unwrap().x, scroll_middle.x),
                            'y' => (current.middle.as_ref().unwrap().y, scroll_middle.y),
                            _ => panic!()
                        };

                        _transform_distance = Some((scroll_middle - current_middle) * -speed);
                    }
                }
            }

            if current.sticky.as_ref().is_some() {
                let window = window().unwrap();
                let inner_height = window.inner_height().unwrap().as_f64().unwrap();
                let inner_width = window.inner_width().unwrap().as_f64().unwrap();
                if *current.in_view.as_ref().unwrap() {
                    if options.direction.as_str() == "horizontal" {
                        _transform_distance = Some(scroll_x - current.left + inner_width);
                    } else {
                        _transform_distance = Some(scroll_y - current.top + inner_height);
                    }
                } else {
                    if options.direction.as_str() == "horizontal" {
                        if scroll_x < current.left - inner_width && scroll_x < current.left - inner_width / 2.0 {
                            _transform_distance = Some(0.0)
                        } else if scroll_x > current.right && scroll_x > current.right + 100.0 {
                            _transform_distance = Some(current.right - current.left + inner_width);
                        } else {
                            _transform_distance = None;
                        }
                    } else {
                        if scroll_y < current.top - inner_height && scroll_y < current.top - inner_height / 2.0 {
                            _transform_distance = Some(0.0)
                        } else if scroll_y > current.bottom && scroll_y > current.bottom + 100.0 {
                            _transform_distance = Some(current.bottom - current.top + inner_height);
                        } else {
                            _transform_distance = None;
                        } 
                    }
                }
            }

            if let Some(val) = _transform_distance {
                if current.direction.as_ref().unwrap().as_str() == "horizontal" || (options.direction.as_str() == "horizontal" && current.direction.as_ref().unwrap().as_str() != "vertical") {
                    let delay = if is_forced.is_some() {
                        if let Some(val) = current.delay.as_ref() {
                            Some(val.parse::<f64>().unwrap())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    SmoothScroll::transform(current.el.as_ref().unwrap().clone(), Some(val), Some(0.0), delay)
                } else {
                    let delay = if is_forced.is_some() {
                        if let Some(val) = current.delay.as_ref() {
                            Some(val.parse::<f64>().unwrap())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    SmoothScroll::transform(current.el.as_ref().unwrap().clone(), Some(0.0), Some(val), delay)
                }
            }
        }
    }

    pub fn resize(core:Rc<RefCell<Core>>, options: &LocomotiveOption) {
        {
            core.borrow_mut().window_height = window().unwrap().inner_height().unwrap().as_f64().unwrap();
        }
        {
            core.borrow_mut().window_width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        }

        Core::check_context(core.clone(), options);

        {   
            let window_height = core.borrow().window_height;
            let window_width = core.borrow().window_width;
            core.borrow_mut().window_middle = Position {
                x: window_width / 2.0,
                y: window_height / 2.0,
            }
        }
    }

    pub fn scroll_to(target: ScrollToTarget, scroll_to_option: ScrollToOption, core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        let core = core.clone();
        let direction_axis = core.borrow().direction_axis.clone().as_ref().clone().into_inner();
        let instance = core.borrow().instance.clone();
        let (limit, scroll_val, limit_x, limit_y) = (instance.borrow().limit.get(direction_axis), instance.borrow().scroll.get(direction_axis), instance.borrow().limit.x, instance.borrow().limit.y);
        let scroll = &core.borrow().scroll;
        let scroll = scroll.get_smooth();

        

        let offset = match &scroll_to_option.offset {
            Some(val) => val.parse::<f64>().unwrap_or(0.0).floor(),
            None => 0.0
        };
        let duration = match &scroll_to_option.duration {
            Some(val) => val.floor(),
            None => 1000.0,
        };
        let easing = match &scroll_to_option.easing {
            Some(val) => *val,
            None => [0.25, 0.0, 0.35, 1.0]
        };
        let disable_lerp = match &scroll_to_option.disable_lerp {
            Some(val) => *val,
            None => false
        };
        //todo
        //let callback = None;
        let easing = bezier2(easing[0], easing[1], easing[2], easing[3]);

        let target = match &target {
            ScrollToTarget::String(val) => {
                match val.as_str() {
                    "top" => ScrollToTarget::Num(0.0),
                    "bottom" => ScrollToTarget::Num(limit_y),
                    "left" => ScrollToTarget::Num(0.0),
                    "right" => ScrollToTarget::Num(limit_x),
                    val => {
                        let target = window().unwrap().document().unwrap().query_selector(val).unwrap();
                        if let Some(el) = target {
                            ScrollToTarget::Element(el)
                        } else {
                            return;
                        }
                    }
                }
            },
            ScrollToTarget::Num(num) => ScrollToTarget::Num(num.floor()),
            ScrollToTarget::Element(el) => ScrollToTarget::Element(el.clone()),
        };

        let offset = match target {
            ScrollToTarget::Num(num) => num + offset,
            ScrollToTarget::Element(el) => {
                let target_parents = get_parents(el.clone());
                let target_in_scope = target_parents.contains(options.el.try_get_element().unwrap());
                if !target_in_scope {
                    return;
                }

                let target_bcr = el.get_bounding_client_rect();
                let offset_top = target_bcr.top();
                let offset_left = target_bcr.left();

                let parent_section = target_parents.iter().find(|&candidate| {
                    scroll.sections.borrow().data.values().any(|section| section.borrow().el == candidate.clone())
                });
                let parent_section_offset = match parent_section {
                    Some(el) => get_translate(el).unwrap().get(direction_axis),
                    None => -scroll_val,
                };

                if options.direction.as_str() == "horizontal" {
                    offset_left + offset - parent_section_offset
                } else {
                    offset_top + offset - parent_section_offset
                }

            },
            _ => panic!()
        };


        //ACTUAL SCROLLTO
        let scroll_start = limit;
        let scroll_target = 0.0f64.max(offset.min(limit));
        let scroll_diff = scroll_target - scroll_start;
        let direction = options.direction.clone();

        let render = move |p: f64| {
            let instance = instance.clone();
            if disable_lerp {
                if direction.as_str() == "horizontal" {
                    SmoothScroll::set_scroll(instance.clone(), scroll_start + scroll_diff * p, instance.borrow().delta.as_ref().unwrap().y);
                } else {
                    SmoothScroll::set_scroll(instance.clone(), instance.borrow().delta.as_ref().unwrap().x, scroll_start + scroll_diff * p);
                }
            } else {
                match direction_axis {
                    'x' => instance.borrow_mut().delta.as_mut().unwrap().x = scroll_start + scroll_diff * p,
                    'y' => instance.borrow_mut().delta.as_mut().unwrap().y = scroll_start + scroll_diff * p,
                    _ => panic!()
                }
            }
        };
        let render = Box::new(render);

        {
            *scroll.animating_scroll.borrow_mut() = true;
        }
        {
            SmoothScroll::stop_scrolling(core.clone(), scroll.start_scroll_ts.clone(), scroll.check_scroll_raf.clone(), scroll.scroll_to_raf.clone(), scroll.is_scrolling.clone(), options.scrolling_class.clone());
        }
        {
            SmoothScroll::start_scrolling(core.clone(), options.clone());
        }

        let core_1 = core.clone();
        let options_1 = options.clone();
        let start = Date::now();
        let loop_event = Closure::wrap(Box::new(move || {

            let p = ( Date::now() - start ) / duration;
            let options = options_1.clone();
            let core = core_1.clone();
            let scroll = &core.borrow().scroll;
            let scroll = scroll.get_smooth();

            if p > 1.0 {
                render(1.0);
                *scroll.animating_scroll.clone().borrow_mut() = false;

                if duration == 0.0 {
                    SmoothScroll::update(core.clone(), &options);
                }
            
            } else {
               *scroll.scroll_to_raf.borrow_mut() = Some(window().unwrap().request_animation_frame(scroll.loop_cb.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap());
               render(easing(p));
            }
        }) as Box<dyn Fn()>);
        {
            *scroll.loop_cb.borrow_mut() = Some(loop_event);
        }
        scroll.loop_cb.borrow().as_ref().unwrap().as_ref().unchecked_ref::<Function>().call0(&"".into()).unwrap();


    }

    pub(crate) fn update(core: Rc<RefCell<Core>>, options: &LocomotiveOption) {
        SmoothScroll::set_scroll_limit(core.clone(), options);
        SmoothScroll::add_sections(core.clone(), options);
        SmoothScroll::add_elements(core.clone(), options);
        Core::detect_elements(None, core.clone(), options);
        SmoothScroll::update_scroll(core.clone(), 0.0);
        SmoothScroll::transform_elements(Some(true), None, core.clone(), options);
        SmoothScroll::reinit_scrollbar(core.clone(), options);

        SmoothScroll::check_scroll(Some(true), core.clone(), options.clone());
    }
}
