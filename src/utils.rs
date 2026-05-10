extern crate web_sys;
use web_sys::console;
pub struct Timer<'a> {
    name: &'a str,
}
impl<'a> Timer<'a> {
    pub fn start(name: &'a str) -> Self {
        console::time_with_label(name);
        Self { name }
    }
}
impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
