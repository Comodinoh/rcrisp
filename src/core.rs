use crate::timings;

//
// struct Core
// {
//
// }
//
// impl Core
// {
//     pub fn new() -> Self
//     {
//
//     }
//
//     pub fn init(&self);
// }

pub trait Run
{
    fn init(&mut self);
    fn shutdown(&mut self);
    fn update(&mut self);
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
        self.run.init();
    }

    pub fn shutdown(&mut self)
    {
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




