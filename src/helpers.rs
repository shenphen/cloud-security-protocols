use std::time::Instant;

pub struct PerformanceTimer {
    instant: Instant,
    title: String,
    is_measuring: bool,
}

impl PerformanceTimer {
    pub fn new() -> Self {
        PerformanceTimer {
            instant: Instant::now(),
            title: "PerformanceTimerInit".to_owned(),
            is_measuring: false,
        }
    }

    pub fn start(&mut self, title: String) {
        if self.is_measuring {
            self.stop();
        }

        self.is_measuring = true;
        self.title = title;
        self.instant = Instant::now();
    }

    pub fn stop(&mut self) {
        if self.is_measuring {
            self.is_measuring = false;
            let elapsed = self.instant.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            println!("{} took {} seconds", self.title, sec);
        } else {
            println!("No active timer is run");
        }
    }
}
