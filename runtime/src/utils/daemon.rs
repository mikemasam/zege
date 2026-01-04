use std::fs::File;
use std::process::{self, Command, exit};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{env, fs};
use tokio::time::sleep;

use crate::lib::events::input::LogEventChannelMessage;

pub async fn wait_for_signal_impl(
    events_writer: Sender<LogEventChannelMessage>,
    writer_thread: JoinHandle<()>,
) {
    use tokio::signal::unix;
    use tokio::signal::unix::Signal;
    let mut signal_terminate = unix::signal(unix::SignalKind::terminate()).unwrap();
    let mut signal_quit = unix::signal(unix::SignalKind::quit()).unwrap();
    let mut signal_interrupt = unix::signal(unix::SignalKind::interrupt()).unwrap();

    //let mut signal_hangup = unix::signal(unix::SignalKind::hangup()).unwrap();
    //_ = signal_hangup.recv() => println!("Received HANGUP."), TODO: keep named pipe open, reload config and continue to run
    tokio::select! {
        _ = signal_terminate.recv() => {},
        _ = signal_quit.recv() => {},
        _ = signal_interrupt.recv() => {},
    };
    let _ = events_writer.send(LogEventChannelMessage::Shutdown);
    drop(events_writer);
    println!("  ...waiting for other threads for 10s");
    tokio::task::spawn(async {
        sleep(Duration::from_secs(10)).await;
        process::exit(0);
    });
    let _ = writer_thread.join();
}
