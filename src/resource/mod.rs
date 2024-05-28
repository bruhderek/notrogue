use std::{cell::RefCell, collections::HashMap};

use notcurses::Visual;

use self::util::configure_visual;

mod util;

thread_local! {
    pub static RESOURCES: RefCell<HashMap<String, Box<dyn LoadableResource<Visual>>>> = RefCell::new(HashMap::new());
}

pub fn add_resources() {
    RESOURCES.with_borrow_mut(|resources| {
        resources.insert("arch".to_string(), Box::new(LoadableVisual::new("resources/arch.png".to_string())));
    });
}

pub fn get_resource(name: String) -> &'static RefCell<Visual> {
    RESOURCES.with_borrow_mut(|resources| {
        if let Some(x) = resources.get_mut(&name) {
            return x.get()
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
    visual: Option<&'static RefCell<Visual>>
}

impl LoadableVisual {
    fn new(image: String) -> Self {
        LoadableVisual { image, visual: None }
    }
}

impl LoadableResource<Visual> for LoadableVisual {
    fn load(&mut self) -> Result<(), String> {
        if self.visual.is_some() { return Ok(()) }
        match Visual::builder().build_from_file(&self.image) {
            Ok(mut v) => {
                configure_visual(&mut v);
                self.visual = Some(Box::leak(Box::new(RefCell::new(v))));
                Ok(())
            },
            Err(e) => Err(e.to_string())
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