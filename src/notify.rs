//! Everything to do with ADS notifications.

use std::time::Duration;

use byteorder::{LE, ReadBytesExt};

use crate::{Error, Result};

/// A handle to the notification; this can be used to delete the notification later.
pub type Handle = u32;

/// Attributes for creating a notification.
pub struct Attributes {
    pub length: usize,
    pub trans_mode: TransmissionMode,
    pub max_delay: Duration,
    pub cycle_time: Duration,
}

impl Attributes {
    pub fn new(length: usize, trans_mode: TransmissionMode,
               max_delay: Duration, cycle_time: Duration) -> Self {
        Self { length, trans_mode, max_delay, cycle_time }
    }
}

/// When notifications should be generated.
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TransmissionMode {
    /// No transmission.
    NoTrans = 0,
    /// Notify each server cycle.
    ServerCycle = 3,
    /// Notify when the content changes.
    ServerOnChange = 4,
    // Other constants from the C++ library:
    // ClientCycle = 1,
    // ClientOnChange = 2,
    // ServerCycle2 = 5,
    // ServerOnChange2 = 6,
    // Client1Req = 10,
}

/// A notification message from the ADS server.
pub struct Notification {
    data: Vec<u8>,
    nstamps: u32,
}

impl std::fmt::Debug for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Notification [")?;
        for sample in self.samples() {
            writeln!(f, "    {:?}", sample)?;
        }
        write!(f, "]")
    }
}

impl Notification {
    pub fn new(data: Vec<u8>) -> Result<Self> {
        // Relevant data starts at byte 42 with the number of stamps.
        let mut ptr = &data[42..];
        let nstamps = ptr.read_u32::<LE>()?;
        for _ in 0..nstamps {
            let _timestamp = ptr.read_u64::<LE>()?;
            let nsamples = ptr.read_u32::<LE>()?;

            for _ in 0..nsamples {
                let _handle = ptr.read_u32::<LE>()?;
                let length = ptr.read_u32::<LE>()? as usize;
                if ptr.len() >= length {
                    ptr = &ptr[length..];
                } else {
                    return Err(Error::Io(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof, "notification too short")));
                }
            }
        }
        if ptr.is_empty() {
            Ok(Self { data, nstamps })
        } else {
            Err(Error::Communication("too many bytes in notification", ptr.len() as u32))
        }
    }

    pub fn samples(&self) -> SampleIter {
        SampleIter { data: &self.data[46..], cur_timestamp: 0,
                     stamps_left: self.nstamps, samples_left: 0 }
    }
}

/// A single sample in a notification message.
#[derive(Debug)]
pub struct Sample<'a> {
    /// The notification handle associated with the data.
    pub handle: Handle,
    /// Timestamp of generation (nanoseconds since 01/01/1601).
    pub timestamp: u64, // TODO: better dtype?
    /// Data of the handle at the specified time.
    pub data: &'a [u8],
}

/// An iterator over all samples within a notification message.
pub struct SampleIter<'a> {
    data: &'a [u8],
    cur_timestamp: u64,
    stamps_left: u32,
    samples_left: u32,
}

impl<'a> Iterator for SampleIter<'a> {
    type Item = Sample<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.samples_left > 0 {
            let handle = self.data.read_u32::<LE>().unwrap();
            let length = self.data.read_u32::<LE>().unwrap() as usize;
            let (data, rest) = self.data.split_at(length);
            self.data = rest;
            self.samples_left -= 1;
            Some(Sample { handle, data, timestamp: self.cur_timestamp })
        } else if self.stamps_left > 0 {
            self.cur_timestamp = self.data.read_u64::<LE>().unwrap();
            self.samples_left = self.data.read_u32::<LE>().unwrap();
            self.stamps_left -= 1;
            self.next()
        } else {
            None
        }
    }
}