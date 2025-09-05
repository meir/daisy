use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

use super::{file::File, Context};

pub enum Resource {
    File(File),
    Styling(String, String, String),
    Other(String, String),
}

pub type ResourceList = HashMap<String, Rc<RefCell<Resource>>>;

pub trait ResourceListExt {
    fn load(&mut self, ctx: Context, location: PathBuf) -> Rc<RefCell<Resource>>;
    fn get_pages(&self) -> Vec<Rc<RefCell<Resource>>>;
    fn get(&self) -> Vec<Rc<RefCell<Resource>>>;
}

impl ResourceListExt for ResourceList {
    fn load(&mut self, ctx: Context, location: PathBuf) -> Rc<RefCell<Resource>> {
        let absolute_path = location.to_str().unwrap().to_string();
        if self.contains_key(&absolute_path) {
            return self.get(&absolute_path).unwrap().clone();
        }

        let resource = Rc::new(RefCell::new(Resource::File(File::new(
            ctx.clone(),
            &location,
        ))));
        self.insert(absolute_path, resource.clone());
        resource
    }

    fn get_pages(&self) -> Vec<Rc<RefCell<Resource>>> {
        self.values()
            .filter(|res| match &*res.borrow() {
                Resource::File(file) => file.is_page,
                _ => false,
            })
            .cloned()
            .collect()
    }

    fn get(&self) -> Vec<Rc<RefCell<Resource>>> {
        self.values().cloned().collect()
    }
}
