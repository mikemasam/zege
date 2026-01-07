use std::fs::File;
use std::process::{self, Command, exit};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{env, fs};
use tokio::time::sleep;

use crate::lib::events::input::LogEventChannelMessage;

pub async fn wait_for_signal_impl(events_writer: Sender<LogEventChannelMessage>) {
    use tokio::signal::unix;
    use tokio::signal::unix::Signal;
    let mut signal_terminate = unix::signal(unix::SignalKind::terminate()).unwrap();

    //let mut signal_hangup = unix::signal(unix::SignalKind::hangup()).unwrap();
    //_ = signal_hangup.recv() => println!("Received HANGUP."), TODO: keep named pipe open, reload config and continue to run

    loop {
        signal_terminate.recv().await;
        println!("SIGTERM received — sending flush request");

        // Send a flush message instead of closing channel
        let _ = events_writer.send(LogEventChannelMessage::Flush);
    }
}
