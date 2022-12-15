mod callback;
mod utils;

use std::{rc::Rc, cell::RefCell};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, KeyboardEvent, window, WheelEvent, HtmlElement, Node};

use crate::{option::LocomotiveOption, virtual_scroll::{VirtualScroll, VsOption},core::Core, utils::instance::Instance};

use self::utils::Section;

#[derive(Debug, Clone)]
pub struct ParallaxElements {

}

impl ParallaxElements {
    pub fn new() -> Self {
        ParallaxElements {  }
    }
}

#[derive(Debug, Clone)]
pub struct SmoothScroll {
    pub options: LocomotiveOption,
    pub is_scrolling: Rc<RefCell<bool>>,
    pub is_dragging_scrollbar: Rc<RefCell<bool>>,
    pub is_ticking: bool,
    pub has_scroll_ticking: Rc<RefCell<bool>>,
    pub parallax_elements: ParallaxElements,
    pub stop: Rc<RefCell<bool>>,
    pub scrollbar_container: bool,
    pub check_key: Rc<RefCell<Option<Closure<dyn FnMut(KeyboardEvent)>>>>,

    pub virtual_scroll: Option<VirtualScroll>,
    pub animating_scroll: Rc<RefCell<bool>>,

    //stop scrolling
    pub check_scroll_raf: Rc<RefCell<Option<i32>>>,
    pub start_scroll_ts: Rc<RefCell<Option<f64>>>,
    pub scroll_to_raf: Rc<RefCell<Option<i32>>>,

    //checkScroll
    pub sections: Vec<Rc<RefCell<Section>>>,

    //scrollbar
    pub scrollbar: Option<Element>,
    pub scrollbar_thumb: Option<Element>,
    pub get_scrollbar: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
    pub release_scrollbar: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
    pub move_scrollbar: Rc<RefCell<Option<Closure<dyn Fn()>>>>,


    //Closures / Function
    check_key_cb_1: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
    pub check_key_cb_2: Rc<RefCell<Option<Closure<dyn Fn()>>>>,
    pub vs_cb_1: Rc<RefCell<Option<Closure<dyn Fn(WheelEvent)>>>>
}

impl SmoothScroll {
    pub fn new(mut options: LocomotiveOption, html: Rc<RefCell<Element>>, instance: Rc<RefCell<Instance>>, direction_axis: Rc<RefCell<char>>, core: Rc<RefCell<Core>>) -> Self {

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
            parallax_elements: ParallaxElements::new(),
            stop: Rc::new(RefCell::new(false)),
            scrollbar_container,
            check_key: Rc::new(RefCell::new(None)),

            virtual_scroll: None,
            animating_scroll: Rc::new(RefCell::new(false)),

            check_scroll_raf: Rc::new(RefCell::new(None)),
            start_scroll_ts: Rc::new(RefCell::new(None)),
            scroll_to_raf: Rc::new(RefCell::new(None)),
            sections: vec![],

            scrollbar: None,
            scrollbar_thumb: None,
            get_scrollbar: Rc::new(RefCell::new(None)),
            release_scrollbar: Rc::new(RefCell::new(None)),
            move_scrollbar: Rc::new(RefCell::new(None)),

            check_key_cb_1: Rc::new(RefCell::new(None)),
            check_key_cb_2: Rc::new(RefCell::new(None)),
            vs_cb_1: Rc::new(RefCell::new(None)),
        };

        smooth.check_key_cb_1(core.clone());
        smooth.check_key_cb_2(core.clone());
        smooth.check_key_callback(core.clone());
        smooth.vs_cb_1(core.clone(), options);
        

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
            scroll.scroll.set_scrollbar(scrollbar, scrollbar_thumb);
        }
    }
}

