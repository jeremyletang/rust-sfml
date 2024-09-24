//! This is mainly an example to test linking solely against the SFML system module

use {
    sfml::{
        system::{Clock, Time},
        SfError,
    },
    std::io::Write,
};

fn main() -> Result<(), SfError> {
    let clock = Clock::start()?;
    while clock.elapsed_time().as_seconds() < 5.0 {
        print!("Elapsed time: {}\r", clock.elapsed_time().as_seconds());
        let _ = std::io::stdout().flush();
        sfml::system::sleep(Time::milliseconds(100));
    }
    Ok(())
}
