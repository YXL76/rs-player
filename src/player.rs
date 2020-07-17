use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};

enum Status {
    Playing(Instant, Duration),
    Stopped(Duration),
}

impl Status {
    fn elapsed(&self) -> Duration {
        match *self {
            Status::Stopped(d) => d,
            Status::Playing(start, extra) => start.elapsed() + extra,
        }
    }

    fn stop(&mut self) {
        if let Status::Playing(start, extra) = *self {
            *self = Status::Stopped(start.elapsed() + extra)
        }
    }

    fn play(&mut self) {
        if let Status::Stopped(duration) = *self {
            *self = Status::Playing(Instant::now(), duration)
        }
    }

    fn reset(&mut self) {
        *self = Status::Stopped(Duration::from_nanos(0));
    }
}

pub struct Player {
    device: rodio::Device,
    status: Status,
    sink: rodio::Sink,
}

impl Player {
    pub fn new() -> Player {
        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);
        sink.pause();

        Player {
            device,
            status: Status::Stopped(Duration::from_nanos(0)),
            sink,
        }
    }

    pub fn load(&mut self, url: &str) -> bool {
        match File::open(url) {
            Ok(file) => match rodio::Decoder::new(BufReader::new(file)) {
                Ok(source) => {
                    self.stop();
                    self.sink = rodio::Sink::new(&self.device);
                    self.sink.append(source);
                    self.play();
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub fn play(&mut self) {
        self.sink.play();
        self.status.play()
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.status.stop()
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.status.reset();
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn position(&self) -> u128 {
        self.status.elapsed().as_millis()
    }
}
