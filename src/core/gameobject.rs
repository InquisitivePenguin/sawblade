pub trait GameObject {
    fn get_id(&self) -> u64;
}

pub trait HasComponents: GameObject {
    fn component_list(&self) -> Vec<String>;
}