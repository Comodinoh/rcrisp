use std::time;

pub struct ElapsedTime
{
    secs: f64
}

impl ElapsedTime
{
    pub fn new() -> Self
    {
        Self { secs: 0.0 }
    }
    pub fn update(&mut self, duration: &time::Duration)
    {
        let secs = duration.as_secs_f64();
        let nanos = (duration.subsec_nanos() as f64) / 1e9f64;

        self.secs = secs + nanos;
    }
}

pub struct Timer
{
    pub start_time: time::Instant
}

impl Timer
{
    pub fn new() -> Timer
    {
        Timer{start_time: time::Instant::now()}
    }

    pub fn process(&mut self, elapsed_time: &mut ElapsedTime)
    {
        elapsed_time.update(&self.start_time.elapsed());
        self.start_time = time::Instant::now();
    }
}

pub struct ScopedTimer<'a>
{
    pub start_time: time::Instant,
    pub elapsed: &'a mut time::Duration
}

impl<'a> ScopedTimer<'a>
{
    pub fn new(elapsed: &'a mut time::Duration) -> ScopedTimer<'a>
    {
        ScopedTimer{start_time: time::Instant::now(), elapsed}
    }
}

impl<'a> Drop for ScopedTimer<'a>
{
    fn drop(&mut self)
    {
        *self.elapsed = self.start_time.elapsed();
    }
}

