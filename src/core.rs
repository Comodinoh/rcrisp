use crate::timings;

pub trait Run
{
    fn init(&mut self);
    fn shutdown(&mut self);
    fn update(&mut self);
}

pub struct Core<T>
where T: Run
{
    running: bool,
    app: Application<T>
}

impl<T> Core<T>
where T: Run
{
    pub fn new(run: T) -> Self
    {
        Core{
            running: true,
            app: Application {
                run,
                timer: timings::Timer::new(),
                running: true,
            }
        }
    }

    pub fn run(&mut self)
    {
        self.init();

        while self.running
        {
            self.app.init();

            self.app.run();

            self.app.shutdown();
        }

        self.shutdown();
    }

    fn init(&self)
    {
        println!("Initializing core");
    }

    fn shutdown(&self)
    {
        println!("Shutting down core");
    }
}



pub struct Application<T>
where T: Run
{
    pub run: T,
    pub timer: timings::Timer,
    pub running: bool,
}

impl<T> Application<T>
where T: Run
{
    pub fn new(run: T) -> Application<T>
    {
        Application { run , timer: timings::Timer::new(), running: true }
    }

    pub fn init(&mut self)
    {
        println!("Initializing application");
        self.run.init();
    }

    pub fn shutdown(&mut self)
    {
        println!("Shutting down application");
        self.run.shutdown();
    }

    pub fn run(&mut self)
    {
        let mut elapsed: timings::ElapsedTime = timings::ElapsedTime::new();

        while self.running
        {
            self.timer.process(&mut elapsed);
        }
    }
}




