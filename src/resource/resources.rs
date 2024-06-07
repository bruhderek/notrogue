use std::collections::HashMap;

use notcurses::Visual;

use super::{LoadableResource, LoadableVisual, RESOURCES};

fn insert_resource(resources: &mut HashMap<&str, Box<dyn LoadableResource<Visual>>>, k: &'static str, v: &'static str) {
    resources.insert(k, Box::new(LoadableVisual::new(v)));
}

pub fn add_resources() {
    RESOURCES.with_borrow_mut(|resources| {
        insert_resource(resources, "arch", "resources/arch.png");
        insert_resource(resources, "knight", "resources/knight.png");
    });
}