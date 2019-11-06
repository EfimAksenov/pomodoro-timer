use std::time::{Duration};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

// todo: use state pattern
fn main() {
    let config = Config::new();

    let workflow = Workflow::new(config);

    for state in workflow {
        println!("{}", state.start_message);
        let receiver = state.execute();
        let Message::Timer() = receiver.recv().unwrap();
        print!("{}", state.end_message)
    }

}

struct Config {
    pomodoro_duration: Duration,
    short_break_duration: Duration,
    long_break_duration: Duration,
    pomodoro_start_msg: String,
    pomodoro_end_msg: String,
    short_break_start_msg: String,
    short_break_end_msg: String,
    long_break_start_msg: String,
    long_break_end_msg: String,
}

impl Config {
    fn new() -> Config {
        let pomodoro_duration = Duration::new(25 * 60, 0);
        let short_break_duration = Duration::new(5 * 60, 0);
        let long_break_duration = Duration::new(25 * 60, 0);
        Config {
            pomodoro_duration,
            short_break_duration,
            long_break_duration,
            pomodoro_start_msg: "Time to work!".to_string(),
            pomodoro_end_msg: "Time to rest.".to_string(),
            short_break_start_msg: "Short break.".to_string(),
            short_break_end_msg: "Break is over.".to_string(),
            long_break_start_msg: "Long break.".to_string(),
            long_break_end_msg: "Break is over.".to_string(),
        }
    }
}

struct Workflow {
    config: Config,
    was_break: bool,
    break_counter: usize,
}

impl Workflow {
    fn new(config: Config) -> Workflow {
        Workflow {
            config,
            was_break: true,
            break_counter: 0,
        }
    }
}

impl Iterator for Workflow {
    type Item = State;

    fn next(&mut self) -> Option<State> {
       let next_state = if self.was_break {
            State::new(self.config.pomodoro_duration, self.config.pomodoro_start_msg, self.config.pomodoro_end_msg)
       } else {
           if self.break_counter < 3 {
               self.break_counter += 1;
            State::new(self.config.short_break_duration, self.config.short_break_start_msg, self.config.short_break_end_msg)
           } else {
               self.break_counter = 0;
            State::new(self.config.long_break_duration, self.config.long_break_start_msg, self.config.long_break_end_msg)
           }
       };
        self.was_break = !self.was_break;
        Some(next_state)
    }
}

struct State {
    duration: Duration,
    start_message: String,
    end_message: String,
}

impl State {
    fn new(duration: Duration, start_message: String, end_message: String) -> State {
        State {duration, start_message, end_message}
    }

    fn execute(self) -> Receiver<Message> {
        let (sender, receiver) = mpsc::channel();

        let duration = self.duration.copy();

        thread::spawn(move || {
            thread::sleep(duration);
            sender.send(Message::Timer()).unwrap();
        });

        receiver
    }
}
