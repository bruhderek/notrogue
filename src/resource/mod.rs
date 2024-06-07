use std::{cell::RefCell, collections::HashMap};

use notcurses::Visual;

use self::util::configure_visual;

mod util;
pub mod resources;

thread_local! {
    pub static RESOURCES: RefCell<HashMap<&'static str, Box<dyn LoadableResource<Visual>>>> = RefCell::new(HashMap::new());
}

pub fn static_resource(name: &'static str) -> &'static RefCell<Visual> {
    get_resource(name.to_string())
}

pub fn get_resource(name: String) -> &'static RefCell<Visual> {
    RESOURCES.with_borrow_mut(|resources| {
        if let Some(x) = resources.get_mut(name.as_str()) {
            return x.get();
        }
        panic!("resource does not exist: {name}");
    })
}

trait LoadableResource<T> {
    fn load(&mut self) -> Result<(), String>;
    fn get(&mut self) -> &'static RefCell<T>;
}

pub struct LoadableVisual {
    image: String,
    visual: Option<&'static RefCell<Visual>>,
}

impl LoadableVisual {
    fn new(image: &'static str) -> Self {
        LoadableVisual {
            image: image.to_string(),
            visual: None,
        }
    }
}

impl LoadableResource<Visual> for LoadableVisual {
    fn load(&mut self) -> Result<(), String> {
        if self.visual.is_some() {
            return Ok(());
        }
        match Visual::builder().build_from_file(&self.image) {
            Ok(mut v) => {
                configure_visual(&mut v);
                self.visual = Some(Box::leak(Box::new(RefCell::new(v))));
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    fn get(&mut self) -> &'static RefCell<Visual> {
        self.load().expect("unable to load resource");
        match &self.visual {
            Some(v) => v,
            None => {
                panic!("resource not loaded")
            }
        }
    }
}
