use rcrisp;

struct TestApplication
{
}

impl rcrisp::core::Run for TestApplication
{
    fn init(&mut self) {
        
        
    }

    fn shutdown(&mut self) {
        
        
    }

    fn update(&mut self) {
        
    }
}
fn main() {
    rcrisp::core::Core::new(TestApplication{}).run();

}
