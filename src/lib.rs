#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

pub mod email_down ;
pub mod email_proc ;
pub mod constants ;
pub mod util ;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
