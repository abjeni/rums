
pub mod response {

    use crate::Node;
    
    use std::error::Error;

    pub struct Response<'a, T, NIDT> {
        pub response: Result<T, Box<dyn Error>>,
        pub node: &'a Node<NIDT>
    }
}