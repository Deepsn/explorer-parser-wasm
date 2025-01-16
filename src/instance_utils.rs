use rbx_dom_weak::types::Ref;
use rbx_dom_weak::{Instance, WeakDom};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RBXInstance {
    name: String,
    class: String,
    reference: Ref,
    children: Vec<Box<RBXInstance>>,
}

impl RBXInstance {
    fn new(dom: &WeakDom, instance_ref: &Ref) -> Self {
        let instance = dom.get_by_ref(*instance_ref).expect("instance wasn't found using referent");

        RBXInstance {
            name: instance.name.clone(),
            class: instance.class.clone(),
            reference: instance.referent(),
            children: Vec::new(),
        }
    }
    
    fn to_instance<'a>(&self, dom: &'a WeakDom) -> &'a Instance {
        dom.get_by_ref(self.reference).expect("instance wasn't found using referent")
    }
    
    fn add_child(&mut self, child: RBXInstance) {
        self.children.push(Box::from(child));
    }
}

pub fn parse_instances(dom: WeakDom) -> RBXInstance {
    let root_instance = dom.root();
    let mut root = RBXInstance::new(&dom, &root_instance.referent());
    
    fn iterate(dom: &WeakDom, reference: &Ref, parent: &mut RBXInstance) {
        let mut rbx_instance = RBXInstance::new(dom, reference);
        let instance = rbx_instance.to_instance(dom);
        
        for instance_ref in instance.children() {
            iterate(dom, instance_ref, &mut rbx_instance);
        }
        
        parent.add_child(rbx_instance);
    }
    
    iterate(&dom, &root_instance.referent(), &mut root);
    
    root
}