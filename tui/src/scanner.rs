use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;

use crate::checks::Report;
use crate::env::EnvSnapshot;
use crate::probe::Probes;

pub struct Scanner {
    reports: Receiver<Report>,
    wake: Sender<()>,
}

impl Scanner {
    pub fn spawn(interval: Duration) -> Self {
        let (sender, reports) = mpsc::channel();
        let (wake, requests) = mpsc::channel();

        thread::spawn(move || {
            let mut probes = Probes::new();

            loop {
                let env = EnvSnapshot::read();

                if sender.send(Report::run(&env, &mut probes)).is_err() {
                    return;
                }

                match requests.recv_timeout(interval) {
                    Ok(()) | Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => return,
                }
            }
        });

        Scanner { reports, wake }
    }

    pub fn latest(&self) -> Option<Report> {
        let mut newest = None;

        while let Ok(report) = self.reports.try_recv() {
            newest = Some(report);
        }

        newest
    }

    pub fn request(&self) {
        let _ = self.wake.send(());
    }
}
