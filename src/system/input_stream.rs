use {
    crate::{
        cpp::FBox,
        ffi::system::{sfInputStreamHelper, sfInputStreamHelper_del},
    },
    std::{
        io::{Read, Seek, SeekFrom},
        marker::PhantomData,
        os::raw::{c_longlong, c_void},
        ptr,
    },
};

unsafe extern "C" fn read<T: Read + Seek>(
    data: *mut c_void,
    size: c_longlong,
    user_data: *mut c_void,
) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data.cast::<T>()) };
    if size == 0 {
        return 0;
    }
    let Ok(size) = usize::try_from(size) else {
        return -1;
    };
    let mut chunk = stream.take(size as u64);
    let mut buf = vec![];
    match chunk.read_to_end(&mut buf) {
        Ok(bytes_read) => {
            unsafe { ptr::copy_nonoverlapping(buf.as_ptr(), data.cast(), bytes_read) };
            c_longlong::try_from(bytes_read).unwrap_or(-1)
        }
        Err(_) => -1,
    }
}

unsafe extern "C" fn get_size<T: Read + Seek>(user_data: *mut c_void) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data.cast()) };
    let Ok(pos) = stream.stream_position() else {
        return -1;
    };
    let Ok(size) = stream.seek(SeekFrom::End(0)) else {
        return -1;
    };
    if stream.seek(SeekFrom::Start(pos)).is_err() {
        return -1;
    }
    c_longlong::try_from(size).unwrap_or(-1)
}

unsafe extern "C" fn tell<T: Read + Seek>(user_data: *mut c_void) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data.cast()) };
    match stream.stream_position() {
        Ok(position) => c_longlong::try_from(position).unwrap_or(-1),
        Err(_) => -1,
    }
}

unsafe extern "C" fn seek<T: Read + Seek>(
    position: c_longlong,
    user_data: *mut c_void,
) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data.cast()) };
    let Ok(position) = u64::try_from(position) else {
        return -1;
    };
    match stream.seek(SeekFrom::Start(position)) {
        Ok(n) => c_longlong::try_from(n).unwrap_or(-1),
        Err(_) => -1,
    }
}

/// Type that allows you to create an SFML input stream from a `Read + Seek` source.
#[derive(Debug)]
pub struct InputStream<'src, T> {
    #[cfg_attr(not(any(feature = "audio", feature = "graphics")), expect(dead_code))]
    pub(crate) stream: FBox<sfInputStreamHelper>,
    _source: Option<Box<T>>,
    _marker: PhantomData<&'src mut T>,
}

impl<'src, T: Read + Seek> InputStream<'src, T> {
    /// Create a new input stream from a `Read + Seek` source.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io::Cursor;
    /// # use sfml::system::InputStream;
    /// let mut cursor = Cursor::new(vec![1u8, 2, 3]);
    /// let stream = InputStream::new(&mut cursor);
    /// let _ = stream;
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if a new `InputStream` can't be created for some reason.
    pub fn new(stream: &'src mut T) -> InputStream<'src, T> {
        let user_data: *mut T = stream;
        unsafe {
            let new = crate::ffi::system::sfInputStreamHelper_new(
                Some(read::<T>),
                Some(seek::<T>),
                Some(tell::<T>),
                Some(get_size::<T>),
                user_data.cast(),
            );
            Self {
                stream: FBox::new(new).expect("Failed to create InputStream"),
                _source: None,
                _marker: PhantomData,
            }
        }
    }

    /// Create a new input stream from a `Read + Seek` source, with ownership.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io::Cursor;
    /// # use sfml::system::InputStream;
    /// let stream = InputStream::new_owned(Cursor::new(vec![1u8, 2, 3]));
    /// let _ = stream;
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if a new `InputStream` can't be created for some reason.
    pub fn new_owned(stream: T) -> InputStream<'src, T> {
        let mut stream = Box::new(stream);
        let user_data: *mut T = stream.as_mut();

        unsafe {
            let new = crate::ffi::system::sfInputStreamHelper_new(
                Some(read::<T>),
                Some(seek::<T>),
                Some(tell::<T>),
                Some(get_size::<T>),
                user_data.cast(),
            );
            Self {
                stream: FBox::new(new).expect("Failed to create InputStream"),
                _source: Some(stream),
                _marker: PhantomData,
            }
        }
    }
}

impl Drop for sfInputStreamHelper {
    fn drop(&mut self) {
        unsafe {
            sfInputStreamHelper_del(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_size, read, seek, tell};
    use std::{
        io::{self, Read, Seek, SeekFrom},
        os::raw::c_void,
    };

    /// A source whose `Read` side works but whose seek operations fail.
    struct FailingSeek;

    impl Read for FailingSeek {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Ok(0)
        }
    }

    impl Seek for FailingSeek {
        fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {
            Err(io::Error::other("seek failed"))
        }
    }

    #[test]
    fn failing_seek_callbacks_return_minus_one() {
        let mut stream = FailingSeek;
        let user_data: *mut FailingSeek = &mut stream;
        let user_data = user_data.cast::<c_void>();

        // SAFETY: `user_data` points to a live `FailingSeek`.
        unsafe {
            assert_eq!(get_size::<FailingSeek>(user_data), -1);
            assert_eq!(tell::<FailingSeek>(user_data), -1);
            assert_eq!(seek::<FailingSeek>(0, user_data), -1);
        }
    }

    #[test]
    fn read_with_negative_size_returns_minus_one() {
        let mut stream = FailingSeek;
        let user_data: *mut FailingSeek = &mut stream;
        let user_data = user_data.cast::<c_void>();
        let mut buf = [0u8; 4];

        // SAFETY: `user_data` points to a live `FailingSeek`, and `buf` is
        // large enough because the negative size is rejected before writing.
        unsafe {
            assert_eq!(
                read::<FailingSeek>(buf.as_mut_ptr().cast(), -1, user_data),
                -1
            );
        }
    }
}
