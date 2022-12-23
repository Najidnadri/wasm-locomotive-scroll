use wasm_bindgen::{JsValue, JsCast};
use web_sys::{window, Document, Element, NodeList, Event, HtmlCollection};

#[derive(Clone, Debug)]
pub enum ElementType {
    Document(Document),
    Element(Element),
}

impl Default for ElementType {
    fn default() -> Self {
        ElementType::Document(window().unwrap().document().unwrap())
    }
}



impl ElementType {
    pub fn _query_selector(&self, selectors: &str) -> Option<Element> {
        match self {
            ElementType::Document(doc) => doc.query_selector(selectors).unwrap(),
            ElementType::Element(el) => el.query_selector(selectors).unwrap()
        }
    }

    pub fn query_selector_all(&self, selectors: &str) -> Option<NodeList> {
        match self {
            ElementType::Document(doc) => Some(doc.query_selector_all(selectors).unwrap()),
            ElementType::Element(el) => Some(el.query_selector_all(selectors).unwrap())
        }
    }

    pub fn try_get_element(&self) -> Option<&Element> {
        match self {
            ElementType::Document(_) => None,
            ElementType::Element(el) => Some(el)
        }
    }

    pub fn _try_get_document(&self) -> Option<&Document> {
        match self {
            ElementType::Document(doc) => Some(doc),
            ElementType::Element(_) => None
        }
    }

    pub fn dispatch_event(&self, event: &Event) {
        match self {
            ElementType::Document(doc) => doc.dispatch_event(event).unwrap(),
            ElementType::Element(el) => el.dispatch_event(event).unwrap()
        };
    }

    pub fn from_element(element: Element) -> Self {
        Self::Element(element)
    }

    pub fn to_js(self) -> JsValue {
        match self {
            ElementType::Document(doc) => doc.dyn_into::<JsValue>().unwrap(),
            ElementType::Element(el) => el.dyn_into::<JsValue>().unwrap()
        }
    }

    pub fn children(&self) -> HtmlCollection {
        match self {
            ElementType::Document(doc) => doc.children(),
            ElementType::Element(el) => el.children()
        }
    }

    pub fn get_element(&self) -> &Element {
        match self {
            Self::Element(el) => el,
            _ => panic!("cannot get element proprty")
        }
    }


}

