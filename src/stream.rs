//! Provides a struct that implements logic to present a stream of disparate byte slices as a single stream of bytes

use embedded_usb_pd::PdError;

use crate::trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReadState {
    /// Total number of bytes to read
    pub total: usize,
    /// Number of bytes already read for the current read operation
    pub current: usize,
}

impl ReadState {
    /// Create a new read state with the given total length
    pub fn new(total: usize) -> Self {
        ReadState { total, current: 0 }
    }
}

/// Data stream state
enum State {
    /// Skip data until we reach the given byte
    Seek(usize),
    /// Consume the given number of bytes
    Read(ReadState),
}

/// Data stream implementation
pub struct Stream {
    /// Curent offset within the overall stream
    global_offset: usize,
    /// Current operation state
    state: State,
}

impl Stream {
    /// Create a new stream
    pub fn new(start_at: usize) -> Self {
        Stream {
            global_offset: 0,
            state: State::Seek(start_at),
        }
    }

    /// Returns if we're currently seeking in the stream
    pub fn is_seeking(&self) -> bool {
        matches!(self.state, State::Seek(_))
    }

    /// Returns if we're currently reading from the stream
    pub fn is_reading(&self) -> bool {
        matches!(self.state, State::Read(_))
    }

    /// Start a seek operation to the given byte
    pub fn start_seek(&mut self, offset: usize) -> Result<(), PdError> {
        if offset < self.global_offset {
            return Err(PdError::InvalidParams);
        }
        self.state = State::Seek(offset);
        Ok(())
    }

    /// Supply a byte slice to the stream
    /// Returns (remaining data, bytes needed to reach the target)
    pub fn seek_bytes<'a>(&mut self, data: &'a [u8]) -> Result<(&'a [u8], usize), PdError> {
        let target = match self.state {
            State::Seek(target) => target,
            State::Read(_) => Err(PdError::InvalidMode)?,
        };

        if self.global_offset + data.len() < target {
            trace!(
                "Still waiting for target byte: {:#x} current: {:#x}",
                target,
                self.global_offset
            );
            // Still waiting for a particular byte
            self.global_offset += data.len();
            return Ok((&data[0..0], target - self.global_offset));
        }

        // Drop any bytes before the target
        trace!("Seeked to {} {}", target, self.global_offset);
        let skip = target - self.global_offset;
        trace!(
            "Data length: {} current: {} target: {}, skipping {} bytes",
            data.len(),
            self.global_offset,
            target,
            skip
        );
        self.global_offset = target;
        Ok((&data[skip..], 0))
    }

    /// Start a read operation of the given length
    pub fn start_read(&mut self, length: usize) {
        self.state = State::Read(ReadState::new(length));
    }

    /// Supply a byte slice to the stream
    /// Returns a slice containing available data and the current read state
    pub fn read_bytes<'a>(&mut self, data: &'a [u8]) -> Result<(&'a [u8], &'a [u8], ReadState), PdError> {
        let read_state = match self.state {
            State::Seek(_) => Err(PdError::InvalidMode)?,
            State::Read(read_state) => read_state,
        };

        trace!("Starting read at {:#x}", self.global_offset);
        let required = read_state.total - read_state.current;
        let bytes_to_take = required.min(data.len());
        let to_take = &data[..bytes_to_take];
        let remaining = &data[bytes_to_take..];
        let current = read_state.current;

        // This will get overwritten by the state function if needed
        self.state = State::Read(ReadState {
            total: read_state.total,
            current: current + bytes_to_take,
        });
        self.global_offset += bytes_to_take;
        trace!(
            "total: {}, required: {}, {} bytes to take, current: {}, data: {}",
            read_state.total,
            required,
            bytes_to_take,
            current,
            data.len()
        );

        Ok((
            remaining,
            to_take,
            ReadState {
                total: read_state.total,
                current,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test that operations in the expected order succeed
    #[test]
    fn test_modes_success() {
        let mut stream = Stream::new(0);
        assert!(stream.start_seek(0).is_ok());
        assert!(stream.seek_bytes(&[0, 1, 2, 3]).is_ok());
        stream.start_read(4);
        assert!(stream.read_bytes(&[0, 1, 2, 3]).is_ok());
    }

    /// Test that attempting to read in seek mode fails
    #[test]
    fn test_modes_read_failure() {
        let mut stream = Stream::new(0);
        assert!(stream.start_seek(0).is_ok());
        assert!(stream.read_bytes(&[0, 1, 2, 3]).is_err());
    }

    /// Test that attempting to seek in read mode fails
    #[test]
    fn test_modes_seek_failure() {
        let mut stream = Stream::new(0);
        stream.start_read(4);
        assert!(stream.seek_bytes(&[0, 1, 2, 3]).is_err());
    }

    /// Test an incomplete seek
    #[test]
    fn test_seek_incomplete() {
        let mut stream = Stream::new(0);
        assert!(stream.start_seek(8).is_ok());
        let (remaining_data, required) = stream.seek_bytes(&[0, 1, 2, 3]).unwrap();
        assert!(remaining_data.is_empty());
        assert_eq!(required, 4);
    }

    /// Test an exact seek
    #[test]
    fn test_seek_exact() {
        let mut stream = Stream::new(0);
        assert!(stream.start_seek(4).is_ok());
        let (remaining_data, required) = stream.seek_bytes(&[0, 1, 2, 3]).unwrap();
        assert!(remaining_data.is_empty());
        assert_eq!(required, 0);
    }

    /// Test that a seek does not consume extra bytes
    #[test]
    fn test_seek_remaining() {
        let mut stream = Stream::new(0);
        assert!(stream.start_seek(2).is_ok());
        let (remaining_data, required) = stream.seek_bytes(&[0, 1, 2, 3, 4]).unwrap();
        assert_eq!(remaining_data, &[2, 3, 4]);
        assert_eq!(required, 0);
    }

    /// Test that a seek backwards fails
    #[test]
    fn test_seek_backwards() {
        let mut stream = Stream::new(0);
        assert!(stream.start_seek(4).is_ok());
        stream.seek_bytes(&[0, 1, 2, 3]).unwrap();
        assert!(stream.start_seek(2).is_err());
    }

    /// Read that an exact read
    #[test]
    fn test_read_exact() {
        let mut stream = Stream::new(0);
        stream.start_read(4);
        let (remaining_data, read_data, read_state) = stream.read_bytes(&[0, 1, 2, 3]).unwrap();
        assert!(remaining_data.is_empty());
        assert_eq!(read_state.current, 0);
        assert_eq!(read_state.total, 4);
        assert_eq!(read_data, &[0, 1, 2, 3]);
    }

    /// Test that a read does not consume extra bytes
    #[test]
    fn test_read_remaining() {
        let mut stream = Stream::new(0);
        stream.start_read(4);
        let (remaining_data, read_data, read_state) = stream.read_bytes(&[0, 1, 2, 3, 4]).unwrap();
        assert_eq!(remaining_data, &[4]);
        assert_eq!(read_state.current, 0);
        assert_eq!(read_state.total, 4);
        assert_eq!(read_data, &[0, 1, 2, 3]);
    }

    /// Test a read split across two calls, with an exact second read
    #[test]
    fn test_read_split_exact() {
        let mut stream = Stream::new(0);
        stream.start_read(7);
        let (remaining_data, read_data, read_state) = stream.read_bytes(&[0, 1, 2, 3]).unwrap();
        assert!(remaining_data.is_empty());
        assert_eq!(read_state.current, 0);
        assert_eq!(read_state.total, 7);
        assert_eq!(read_data, &[0, 1, 2, 3]);

        let (remaining_data, read_data, read_state) = stream.read_bytes(&[4, 5, 6]).unwrap();
        assert!(remaining_data.is_empty());
        assert_eq!(read_state.current, 4);
        assert_eq!(read_state.total, 7);
        assert_eq!(read_data, &[4, 5, 6]);
    }

    /// Test a read split across two calls, with remaining data in the second read
    #[test]
    fn test_read_split_remaining() {
        let mut stream = Stream::new(0);
        stream.start_read(7);
        let (remaining_data, read_data, read_state) = stream.read_bytes(&[0, 1, 2, 3]).unwrap();
        assert!(remaining_data.is_empty());
        assert_eq!(read_state.current, 0);
        assert_eq!(read_state.total, 7);
        assert_eq!(read_data, &[0, 1, 2, 3]);

        let (remaining_data, read_data, read_state) = stream.read_bytes(&[4, 5, 6, 7]).unwrap();
        assert_eq!(remaining_data, &[7]);
        assert_eq!(read_state.current, 4);
        assert_eq!(read_state.total, 7);
        assert_eq!(read_data, &[4, 5, 6]);
    }
}
