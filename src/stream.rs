//! Provides a struct that implements logic to present a stream of disparate byte slices as a single stream of bytes

use embedded_usb_pd::PdError;

use crate::trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReadOperation {
    /// Total number of bytes to read
    pub total: usize,
    /// Number of bytes already read for the current read operation
    pub current: usize,
}

impl ReadOperation {
    /// Create a new read state with the given total length
    pub fn new(total: usize) -> Self {
        ReadOperation { total, current: 0 }
    }

    /// Adnavce the read state by the given number of bytes
    pub fn advance(&mut self, bytes: usize) {
        self.current += bytes;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeekOperation(usize);

impl SeekOperation {
    /// Create a new seek operation
    pub const fn new(target: usize) -> Self {
        SeekOperation(target)
    }
}

/// A stream that is currently seeking data
#[derive(Clone, Copy)]
pub struct SeekingStream {
    /// Curent offset within the overall stream
    position: usize,
    /// Target offset
    target: usize,
}

impl SeekingStream {
    /// Create a new seeking stream
    pub const fn new(starting_offset: usize, target: usize) -> Self {
        SeekingStream {
            position: starting_offset,
            target,
        }
    }

    /// Starts the given operation
    pub fn start_read(&mut self, operation: ReadOperation) -> ReadingStream {
        ReadingStream {
            position: self.position,
            operation,
        }
    }

    /// Supply a byte slice to the stream
    /// Returns the seeked slice
    pub fn seek_bytes<'a>(&mut self, data: &'a [u8]) -> &'a [u8] {
        let skip = self.target - self.position;
        if let Some((_, remaining)) = data.split_at_checked(skip) {
            trace!(
                "Data length: {} current: {} target: {}, skipping {} bytes",
                data.len(),
                self.position,
                self.target,
                skip
            );

            self.position = self.target;
            remaining
        } else {
            trace!(
                "Still waiting for target byte: {:#x} current: {:#x}",
                self.target,
                self.position
            );
            // Still waiting for a particular byte
            self.position += data.len();
            &data[0..0]
        }
    }
}

/// Result of a read operation
pub struct ReadResult<'a> {
    /// Data remaining in the stream
    /// This is the data that was not consumed by the read operation
    pub remaining_data: &'a [u8],
    /// Data consumed by the read operation
    pub read_data: &'a [u8],
    /// Read operation state
    pub read_state: ReadOperation,
    /// Current position within the overall stream
    pub position: usize,
}

impl ReadResult<'_> {
    /// Returns the number of bytes still needed to complete the read operation
    pub fn remaining(&self) -> usize {
        self.read_state.total - (self.read_state.current + self.read_data.len())
    }

    /// Returns true if the read operation is complete
    pub fn is_complete(&self) -> bool {
        self.remaining() == 0
    }
}

/// A stream that is currently reading data
#[derive(Clone, Copy)]
pub struct ReadingStream {
    /// Current position within the overall stream
    position: usize,
    /// Operation state
    operation: ReadOperation,
}

impl ReadingStream {
    /// Start a seek operation to the given byte
    pub fn start_seek(self, operation: SeekOperation) -> Result<SeekingStream, PdError> {
        let target = operation.0;
        if target < self.position {
            Err(PdError::InvalidParams)
        } else {
            Ok(SeekingStream {
                position: self.position,
                target,
            })
        }
    }

    /// Supply a byte slice to the stream
    pub fn read_bytes<'a>(&mut self, data: &'a [u8]) -> ReadResult<'a> {
        trace!("Starting read at {:#x}", self.position);
        let required = self.operation.total - self.operation.current;
        let bytes_to_take = required.min(data.len());
        // SAFETY: bytes_to_take <= data.len()
        let (to_take, remaining) = data.split_at(bytes_to_take);

        let previous_state = self.operation;
        self.operation.advance(bytes_to_take);
        self.position += bytes_to_take;
        trace!(
            "total: {}, required: {}, {} bytes to take, current: {}, data: {}",
            self.operation.total,
            required,
            bytes_to_take,
            self.operation.current,
            data.len()
        );

        // Internally we've already updated our state, but give the previous state because the caller hasn't processed the read yet
        ReadResult {
            remaining_data: remaining,
            read_data: to_take,
            read_state: previous_state,
            position: self.position,
        }
    }
}

/// Wrapper for the two types of streams
#[derive(Clone, Copy)]
pub enum Stream {
    /// Reading stream
    Reading(ReadingStream),
    /// Seeking stream
    Seeking(SeekingStream),
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test an incomplete seek
    #[test]
    fn test_seek_incomplete() {
        let mut stream = SeekingStream::new(0, 8);
        let remaining_data = stream.seek_bytes(&[0, 1, 2, 3]);
        assert!(remaining_data.is_empty());
        assert_eq!(stream.position, 4);
    }

    /// Test an exact seek
    #[test]
    fn test_seek_exact() {
        let mut stream = SeekingStream::new(0, 4);
        let remaining_data = stream.seek_bytes(&[0, 1, 2, 3]);
        assert!(remaining_data.is_empty());
        assert_eq!(stream.position, 4);
    }

    /// Test that a seek does not consume extra bytes
    #[test]
    fn test_seek_remaining() {
        let mut stream = SeekingStream::new(0, 2);
        let remaining_data = stream.seek_bytes(&[0, 1, 2, 3, 4]);
        assert_eq!(remaining_data, &[2, 3, 4]);
        assert_eq!(stream.position, 2);
    }

    /// Test that a seek backwards fails
    #[test]
    fn test_seek_backwards() {
        let mut stream = SeekingStream::new(0, 0);
        let mut stream = stream.start_read(ReadOperation::new(4));
        let _ = stream.read_bytes(&[0, 1, 2, 3]);
        assert!(stream.start_seek(SeekOperation::new(2)).is_err());
    }

    /// Read an exact read
    #[test]
    fn test_read_exact() {
        let mut stream = SeekingStream::new(0, 0);
        let mut stream = stream.start_read(ReadOperation::new(4));
        let read_result = stream.read_bytes(&[0, 1, 2, 3]);
        assert!(read_result.remaining_data.is_empty());
        assert_eq!(read_result.read_state.current, 0);
        assert_eq!(read_result.read_state.total, 4);
        assert_eq!(read_result.read_data, &[0, 1, 2, 3]);
        assert_eq!(read_result.position, 4);
    }

    /// Test that a read does not consume extra bytes
    #[test]
    fn test_read_remaining() {
        let mut stream = SeekingStream::new(0, 0);
        let mut stream = stream.start_read(ReadOperation::new(4));
        let read_result = stream.read_bytes(&[0, 1, 2, 3, 4]);
        assert_eq!(read_result.remaining_data, &[4]);
        assert_eq!(read_result.read_state.current, 0);
        assert_eq!(read_result.read_state.total, 4);
        assert_eq!(read_result.read_data, &[0, 1, 2, 3]);
        assert_eq!(read_result.position, 4);
    }

    /// Test a read split across two calls, with an exact second read
    #[test]
    fn test_read_split_exact() {
        let mut stream = SeekingStream::new(0, 0);
        let mut stream = stream.start_read(ReadOperation::new(7));
        let read_result = stream.read_bytes(&[0, 1, 2, 3]);
        assert!(read_result.remaining_data.is_empty());
        assert_eq!(read_result.read_state.current, 0);
        assert_eq!(read_result.read_state.total, 7);
        assert_eq!(read_result.read_data, &[0, 1, 2, 3]);
        assert_eq!(read_result.position, 4);

        let read_result = stream.read_bytes(&[4, 5, 6]);
        assert!(read_result.remaining_data.is_empty());
        assert_eq!(read_result.read_state.current, 4);
        assert_eq!(read_result.read_state.total, 7);
        assert_eq!(read_result.read_data, &[4, 5, 6]);
        assert_eq!(read_result.position, 7);
    }

    /// Test a read split across two calls, with remaining data in the second read
    #[test]
    fn test_read_split_remaining() {
        let mut stream = SeekingStream::new(0, 0);
        let mut stream = stream.start_read(ReadOperation::new(7));
        let read_result = stream.read_bytes(&[0, 1, 2, 3]);
        assert!(read_result.remaining_data.is_empty());
        assert_eq!(read_result.read_state.current, 0);
        assert_eq!(read_result.read_state.total, 7);
        assert_eq!(read_result.read_data, &[0, 1, 2, 3]);
        assert_eq!(read_result.position, 4);

        let read_result = stream.read_bytes(&[4, 5, 6, 7]);
        assert_eq!(read_result.remaining_data, &[7]);
        assert_eq!(read_result.read_state.current, 4);
        assert_eq!(read_result.read_state.total, 7);
        assert_eq!(read_result.read_data, &[4, 5, 6]);
        assert_eq!(read_result.position, 7);
    }
}
