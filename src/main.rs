use std::time::{Duration};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

fn main() {
    let config = Config::new();

    let workflow = Workflow::new(config);

    for state in workflow {
        let message = match state {
            State::Pomodoro(_) => "Time to work!",
            State::ShortBreak(_) => "Short break.",
            State::LongBreak(_) => "Long break.",
        };
        println!("{}", message);
        let receiver = state.execute();
        let Message::Timer(message) = receiver.recv().unwrap();
        println!("{}", message)
    }

}

enum Message {
    Timer(String)
}

struct Config {
    pomodoro: Duration,
    short_break: Duration,
    long_break: Duration,
}

impl Config {
    fn new() -> Config {
        let pomodoro = Duration::new(25 * 60, 0);
        let short_break = Duration::new(5 * 60, 0);
        let long_break = Duration::new(25 * 60, 0);
        Config {
            pomodoro,
            short_break,
            long_break,
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
            State::Pomodoro(self.config.pomodoro)
       } else {
           if self.break_counter < 3 {
               self.break_counter += 1;
               State::ShortBreak(self.config.short_break)
           } else {
               self.break_counter = 0;
               State::LongBreak(self.config.long_break)
           }
       };
        self.was_break = !self.was_break;
        Some(next_state)
    }
}

trait Command {
    fn execute(self) -> Receiver<Message>;
}

enum State {
    Pomodoro(Duration),
    ShortBreak(Duration),
    LongBreak(Duration),
}

impl Command for State {
    fn execute(self) -> Receiver<Message> {
        let (sender, receiver) = mpsc::channel();

        let (duration, message) = match self {
            State::Pomodoro(duration) => (duration, "Time to rest."),
            State::ShortBreak(duration) => (duration, "Break is over."),
            State::LongBreak(duration) => (duration, "Break is over."),
        };

        thread::spawn(move || {
            thread::sleep(duration);
            sender.send(Message::Timer(message.to_string())).unwrap();
        });

        receiver
    }
}
