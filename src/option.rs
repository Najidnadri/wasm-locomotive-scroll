use convert_js::ToJs;
use serde::{Serialize, Deserialize};
use web_sys::window;

use crate::utils::element_type::ElementType;


#[derive(Clone, Debug, Serialize, Deserialize, ToJs)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0., y: 0. }
    }
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }

    pub fn get(&self, direction_axis: char) -> f64 {
        match direction_axis {
            'x' => self.x,
            'y' => self.y,
            _ => panic!("direction axis not supported")
        }
    }

    pub fn set(&mut self, new: f64, direction_axis: char) {
        match direction_axis {
            'x' => self.x = new,
            'y' => self.y = new,
            _ => panic!("direction axis not supported")
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToJs)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Tablet {
    pub smooth: bool,
    pub direction: String,
    pub gesture_direction: String,
    pub breakpoint: f64,
}

impl Default for Tablet {
    fn default() -> Self {
        Tablet { smooth: false, direction: "vertical".to_string(), gesture_direction: "vertical".to_string(), breakpoint: 1024. }
    }
}



#[derive(Clone, Debug, Serialize, Deserialize, ToJs)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Smartphone {
    pub smooth: bool,
    pub direction: String,
    pub gesture_direction: String,
}

impl Default for Smartphone {
    fn default() -> Self {
        Smartphone { smooth: false, direction: "vertical".to_string(), gesture_direction: "vertical".to_string() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct LocomotiveOption {
    #[serde(skip_serializing, skip_deserializing)]
    pub el: ElementType,
    pub query: String,
    pub name: String,
    pub offset: [f64; 2],
    pub repeat: bool,
    pub smooth: bool,
    pub init_position: Position,
    pub direction: String,
    pub gesture_direction: String,
    pub reload_on_context_change: bool,
    pub lerp: f64,
    pub class: String,
    pub scroll_bar_container: bool,
    pub scroll_bar_class: String,
    pub scrolling_class: String,
    pub dragging_class: String,
    pub smooth_class: String,
    pub init_class: String,
    pub get_speed: bool,
    pub get_direction: bool,
    pub scroll_from_anywhere: bool,
    pub multiplier: f64,
    pub firefox_multiplier: f64,
    pub touch_multiplier: f64,
    pub reset_native_scroll: bool,
    pub tablet: Option<Tablet>,
    pub is_tablet: bool,
    pub smartphone: Option<Smartphone>,
    pub is_mobile: bool,

    //SMOOTH OPTIONS
    pub inertia: Option<f64>,

    //NAMES
    pub names: Option<Names>,
}

impl Default for LocomotiveOption {
    fn default() -> Self {
        LocomotiveOption {
            el: ElementType::Document(window().unwrap().document().unwrap()),
            query: String::from("[data-scroll-container]"),
            name: "scroll".to_string(),
            offset: [0., 0.],
            repeat: false,
            smooth: false,
            init_position: Position::default(),
            direction: "vertical".to_string(),
            gesture_direction: "vertical".to_string(),
            reload_on_context_change: false,
            lerp: 0.1,
            class: "is_inview".to_string(),
            scroll_bar_container: false,
            scroll_bar_class: "c-scrollbar".to_string(),
            scrolling_class: "has-scroll-scrolling".to_string(),
            dragging_class: "has-scroll-dragging".to_string(),
            smooth_class: "has-scroll-smooth".to_string(),
            init_class: "has-scroll-init".to_string(),
            get_speed: false,
            get_direction: false,
            scroll_from_anywhere: false,
            multiplier: 1.,
            firefox_multiplier: 50.,
            touch_multiplier: 2.,
            reset_native_scroll: true,
            tablet: Some(Tablet::default()),
            is_tablet: false,
            smartphone: Some(Smartphone::default()),
            is_mobile: false,
            names: None,

            inertia: None,
        }
    }
}

impl LocomotiveOption {

    /* 
    Object.assign(this, defaults, options);
    this.smartphone = defaults.smartphone;
    if (options.smartphone) Object.assign(this.smartphone, options.smartphone);
    this.tablet = defaults.tablet;
    if (options.tablet) Object.assign(this.tablet, options.tablet);
    */
    pub fn _overwrite(&mut self, rhs: Self) {
        self.el = rhs.el;
        self.name = rhs.name;
        self.offset = rhs.offset;
        self.smooth = rhs.smooth;
        self.init_position = rhs.init_position;
        self.direction = rhs.direction;
        self.gesture_direction = rhs.gesture_direction;
        self.reload_on_context_change = rhs.reload_on_context_change;
        self.lerp = rhs.lerp;
        self.class = rhs.class;
        self.scroll_bar_container = rhs.scroll_bar_container;
        self.scroll_bar_class = rhs.scroll_bar_class;
        self.scrolling_class = rhs.scrolling_class;
        self.dragging_class = rhs.dragging_class;
        self.smooth_class = rhs.smooth_class;
        self.init_class = rhs.init_class;
        self.get_speed = rhs.get_speed;
        self.get_direction = rhs.get_direction;
        self.scroll_from_anywhere = rhs.scroll_from_anywhere;
        self.multiplier = rhs.multiplier;
        self.firefox_multiplier = rhs.firefox_multiplier;
        self.touch_multiplier = rhs.touch_multiplier;
        self.reset_native_scroll = rhs.reset_native_scroll;
        if self.smartphone.is_none() {
            self.smartphone = rhs.smartphone;
        }
        if self.tablet.is_none() {
            self.tablet = rhs.tablet;
        }
    }

    pub fn check_mobile(&mut self) {
        let navigator = window().unwrap().navigator();
        let inner_width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let reg_exp = js_sys::RegExp::new("/Android|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/", "i");

        let is_mobile = reg_exp.test(&navigator.user_agent().unwrap()) || 
            (navigator.platform().unwrap() == "MacIntel".to_string() && navigator.max_touch_points() > 1) || 
            inner_width < 1024.0;

        self.is_mobile = is_mobile;
    }

    pub fn check_tablet(&mut self) {
        let inner_width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let is_tablet = self.is_mobile && inner_width >= 1024.0;

        self.is_tablet = is_tablet;
    }

    pub(crate) fn check_mobile_bool(&self) -> bool {
        let navigator = window().unwrap().navigator();
        let inner_width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let reg_exp = js_sys::RegExp::new("/Android|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/", "i");

        let is_mobile = reg_exp.test(&navigator.user_agent().unwrap()) || 
            (navigator.platform().unwrap() == "MacIntel".to_string() && navigator.max_touch_points() > 1) || 
            inner_width < 1024.0;

        is_mobile
    }

    pub(crate) fn check_tablet_bool(&self) -> bool {
        let inner_width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let is_tablet = self.is_mobile && inner_width >= 1024.0;

        is_tablet
    }

    pub(crate) fn init(&mut self) {
        let names = Names::new(&self.name, &self.scroll_bar_class);
        self.names = Some(names);
        let el = window().unwrap().document().unwrap().query_selector(&self.query).unwrap().unwrap();
        self.el = ElementType::from_element(el);
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Names {
    pub speed: String,
    pub data_direction: String,
    pub thumb: String,
    pub data_section: String,
    pub id: String,
    pub persistent: String,
    pub data: String,
    pub class: String,
    pub repeat: String,
    pub call: String,
    pub position: String,
    pub delay: String,
    pub direction: String,
    pub sticky: String,
    pub offset: String,
    pub target: String,
    pub data_section_inview: String,
}

impl Names {
    pub fn new(name: &str, scrollbar_class: &str) -> Self {
        Names {
            speed: format!("{}Speed", name),
            data_direction: format!("data-{}-direction", name),
            thumb: format!("{}_thumb", scrollbar_class),
            data_section: format!("[data-{}-section]", name),
            id: format!("{}Id", name),
            persistent: format!("{}Persistent", name),  
            data: format!("[data-{}]", name),
            class: format!("{}Class", name),
            repeat: format!("{}Repeat", name),
            call: format!("{}Call", name),
            position: format!("{}Position", name),
            delay: format!("{}Delay", name),
            direction: format!("{}Direction", name),
            sticky: format!("{}Sticky", name),
            offset: format!("{}Offset", name),
            target: format!("{}Target", name),
            data_section_inview: format!("data-{}-section-inview", name),
        }
    }
}