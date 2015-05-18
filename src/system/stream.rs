use std::io::{Read, Seek, SeekFrom};

use libc::c_void;
use std::mem::transmute;
use std::slice;
use std::marker::PhantomData;

/// Wrapper for custom file input streams.
///
/// It is usually only necessary to construct one of these when using `Music`,
/// since in all other cases the target type handles the creation of the
/// `InputStream` internally.
///
/// # Example
/// ```ignore
/// /*A*/ let mut source = Cursor::new(&some_bytes);
/// /*B*/ let mut source = File::open("path/to/file.ogg").unwrap();
/// let mut stream = InputStream::new(&mut source);
/// let mut song = Music::new_from_stream(&mut stream).unwrap();
/// song.play();
/// ```
#[repr(C)]
pub struct InputStream<'a> {
	read_func: unsafe extern fn(dest: *mut c_void, size: i64, user: *mut c_void) -> i64,
	seek_func: unsafe extern fn(pos: i64, user: *mut c_void) -> i64,
	tell_func: unsafe extern fn(user: *mut c_void) -> i64,
	get_size_func: unsafe extern fn(user: *mut c_void) -> i64,
	stream: *mut c_void,
	phantom: PhantomData<&'a mut ()>
}

impl<'a> InputStream<'a> {
	/// Wrap a stream implementing `Read + Seek` in an `InputStream`.
	pub fn new<T: Read + Seek + 'a>(stream: &'a mut T) -> InputStream<'a> {
		InputStream {
			read_func: read_impl::<T>,
			seek_func: seek_impl::<T>,
			tell_func: tell_impl::<T>,
			get_size_func: get_size_impl::<T>,
			stream: unsafe { transmute(stream) },
			phantom: PhantomData
		}
	}
}

unsafe extern fn read_impl<T: Read>(dest: *mut c_void, size: i64, user: *mut c_void) -> i64 {
	let stream: &mut T = transmute(user);
	let buffer = slice::from_raw_parts_mut(transmute(dest), size as usize);
	match stream.read(buffer) {
		Ok(val) => val as i64,
		Err(_) => -1
	}
}

unsafe extern fn seek_impl<T: Seek>(pos: i64, user: *mut c_void) -> i64 {
	let stream: &mut T = transmute(user);
	match stream.seek(SeekFrom::Start(pos as u64)) {
		Ok(val) => val as i64,
		Err(_) => -1
	}
}

unsafe extern fn tell_impl<T: Seek>(user: *mut c_void) -> i64 {
	let stream: &mut T = transmute(user);
	match stream.seek(SeekFrom::Current(0)) {
		Ok(val) => val as i64,
		Err(_) => -1
	}
}

unsafe extern fn get_size_impl<T: Seek>(user: *mut c_void) -> i64 {
	let stream: &mut T = transmute(user);
	match stream.seek(SeekFrom::End(0)) {
		Ok(val) => val as i64,
		Err(_) => -1,
	}
}
