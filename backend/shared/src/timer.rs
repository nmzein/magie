use std::time::Instant;

pub struct Timer {
    start: Instant,
    lap: Instant,
}

impl Timer {
    pub fn new(message: &str) -> Self {
        println!("TIMER Started {message}");
        let time = Instant::now();
        Self {
            start: time,
            lap: time,
        }
    }

    pub fn lap(&mut self, message: &str) {
        println!("TIMER Lap     {:?} {message}", self.lap.elapsed());
        self.lap = Instant::now();
    }

    pub fn end(&mut self, message: &str) {
        println!("TIMER Lap     {:?} {message}", self.lap.elapsed());
        println!("TIMER Ended   {:?}", self.start.elapsed());
    }
}
