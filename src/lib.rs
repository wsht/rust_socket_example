#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate mio;

pub mod multi_thread_server;
pub mod client;
pub mod resource_server;
pub mod server_mio;
pub mod mio_example;
pub mod mio_token_test;