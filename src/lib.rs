use std::time;

pub struct Testmark { name: &'static str, start_time: u128, end_time: u128}

pub trait Timer {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
}

impl Testmark {
    pub fn start(&mut self) -> u128 {
        self.start_time = time::UNIX_EPOCH.elapsed().unwrap().as_micros();
        self.start_time
    }
    pub fn stop(&mut self) -> u128 {
        self.end_time = time::UNIX_EPOCH.elapsed().unwrap().as_micros();
        self.end_time
    }  
    pub fn time(&mut self) -> u128 {
        if self.end_time == 0 {
            time::UNIX_EPOCH.elapsed().unwrap().as_micros() - self.start_time
        } else {
            self.end_time - self.start_time
        }
    }
    pub fn print(&mut self) {
        println!("{0} took {1:.3} ms", self.name(), self.time() as f32 / 1000.00)
    }
}

impl Timer for Testmark {
    fn new(name: &'static str) -> Testmark {
        Testmark { name: name, start_time: 0, end_time: 0 }
    }
    fn name(&self) -> &'static str {
        self.name
    }
}
