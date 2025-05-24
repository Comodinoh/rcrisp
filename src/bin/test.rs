use rcrisp;

struct TestApplication
{
}



impl rcrisp::core::Run for TestApplication
{
    fn init(&mut self) {
        println!("Initializing application");
    }

    fn shutdown(&mut self) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }
}
fn main() {
    let test_app = TestApplication{};


    let mut app = rcrisp::core::Application::new(
        test_app,
    );

    app.init();

}
