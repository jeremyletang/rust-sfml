use {
    crate::{ffi::system::sfInputStream, SfBox},
    std::{
        io::{Read, Seek, SeekFrom},
        marker::PhantomData,
        os::raw::{c_longlong, c_void},
        ptr,
    },
};

#[expect(clippy::comparison_chain)]
unsafe extern "C" fn read<T: Read + Seek>(
    data: *mut c_void,
    size: c_longlong,
    user_data: *mut c_void,
) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data.cast::<T>()) };
    if size == 0 {
        return 0;
    } else if size > 0 {
        #[expect(clippy::unwrap_used)]
        let mut chunk = stream.take(size.try_into().unwrap());
        let mut buf = vec![];
        let result = chunk.read_to_end(&mut buf);
        #[expect(clippy::unwrap_used)]
        if let Ok(bytes_read) = result {
            unsafe { ptr::copy_nonoverlapping(buf.as_ptr(), data.cast(), bytes_read) };
            return bytes_read.try_into().unwrap();
        }
    }
    -1
}

#[expect(clippy::unwrap_used)]
unsafe extern "C" fn get_size<T: Read + Seek>(user_data: *mut c_void) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data.cast()) };
    let pos = stream.stream_position().unwrap();
    let size = stream.seek(SeekFrom::End(0)).unwrap();
    let _ = stream.seek(SeekFrom::Start(pos));
    size.try_into().unwrap()
}

unsafe extern "C" fn tell<T: Read + Seek>(user_data: *mut c_void) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data.cast()) };
    #[expect(clippy::unwrap_used)]
    stream.stream_position().unwrap().try_into().unwrap()
}

unsafe extern "C" fn seek<T: Read + Seek>(
    position: c_longlong,
    user_data: *mut c_void,
) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data.cast()) };
    #[expect(clippy::unwrap_used)]
    match stream.seek(SeekFrom::Start(position.try_into().unwrap())) {
        Ok(n) => n.try_into().unwrap(),
        Err(_) => -1,
    }
}

/// Type that allows you to create an SFML input stream from a `Read + Seek` source.
#[derive(Debug)]
pub struct InputStream<'src, T> {
    pub(crate) stream: SfBox<sfInputStream>,
    _source: PhantomData<&'src mut T>,
}

impl Drop for sfInputStream {
    fn drop(&mut self) {
        unsafe { crate::ffi::system::sfInputStream_destroy(self) }
    }
}

impl<'src, T: Read + Seek> InputStream<'src, T> {
    /// Create a new input stream from a `Read + Seek` source.
    pub fn new(stream: &'src mut T) -> InputStream<'src, T> {
        let user_data: *mut T = stream;
        unsafe {
            let new = crate::ffi::system::sfInputStream_new(
                Some(read::<T>),
                Some(seek::<T>),
                Some(tell::<T>),
                Some(get_size::<T>),
                user_data.cast(),
            );
            Self {
                stream: SfBox::new(new).expect("Failed to create InputStream"),
                _source: PhantomData,
            }
        }
    }
}
