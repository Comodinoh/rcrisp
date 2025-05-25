use rcrisp;
use rcrisp::timings;

struct TestApplication
{
}

impl rcrisp::core::Run for TestApplication
{
    fn init(&mut self) {


    }

    fn shutdown(&mut self) {


    }

    fn update(&mut self, delta: &timings::ElapsedTime) {
        println!("{}s passed since last loop", delta.secs);
    }
}
fn main() {
    rcrisp::core::Core::new(TestApplication{}).run();

}
