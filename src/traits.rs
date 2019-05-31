pub trait Protocol {
    fn new() -> Self;
    fn protocol(&self);
    fn run(&self) {
       self.protocol();
    }
}