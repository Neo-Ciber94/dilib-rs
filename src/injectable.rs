use crate::Container;

pub trait Injectable {
    fn resolve(container: &Container) -> Self;
}