use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};

enum PlayerState {
    Stopped,
    Paused,
    Playing,
}

impl PlayerState {
    fn is_playing(&self) -> bool {
        match *self {
            PlayerState::Stopped | PlayerState::Paused => false,
            PlayerState::Playing => true,
        }
    }
}

enum Status {
    Playing(Instant, Duration),
    Stopped(Duration),
}

impl Status {
    fn elapsed(self) -> Duration {
        match self {
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
    state: PlayerState,
    status: Status,
    sink: rodio::Sink,
}

impl Player {
    pub fn new() -> Player {
        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);

        Player {
            state: PlayerState::Stopped,
            status: Status::Stopped(Duration::from_nanos(0)),
            sink,
            device,
        }
    }

    pub fn load(&mut self, url: &str) {
        let file = File::open(url).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        self.stop();
        self.sink.append(source);
        self.play();
    }

    pub fn play(&mut self) {
        self.sink.play();
        self.state = PlayerState::Playing;
        self.status.play()
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.state = PlayerState::Paused;
        self.status.stop()
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.state = PlayerState::Stopped;
        self.status.reset();
    }

    pub fn status(&self) -> bool {
        self.state.is_playing()
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }
}
