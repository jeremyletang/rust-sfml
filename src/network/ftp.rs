//! A FTP client.

use csfml_network_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;
use network::IpAddress;
use std::ffi::{CStr, CString};
use std::mem;
use std::str;
use system::Time;
use system::raw_conv::Raw;

/// The differents FTP modes availables.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
#[repr(u32)]
pub enum TransferMode {
    /// Binary mode (file is transfered as a sequence of bytes)
    Binary = 0,
    /// Text mode using ASCII encoding.
    Ascii = 1,
    /// Text mode using EBCDIC encoding.
    Ebcdic = 2,
}

/// The status and commands id's for FTP.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
#[repr(u32)]
#[cfg_attr(feature="clippy", allow(enum_variant_names))]
pub enum Status {
    /// Restart marker reply.
    RestartMarkerReply = 110,
    /// Service ready in N minutes.
    ServiceReadySoon = 120,
    /// Data connection already opened, transfer starting.
    DataConnectionAlreadyOpened = 125,
    /// File status ok, about to open data connection.
    OpeningDataConnection = 150,
    /// Command ok.
    Ok = 200,
    /// Command not implemented.
    PointlessCommand = 202,
    /// System status, or system help reply.
    SystemStatus = 211,
    /// Directory status.
    DirectoryStatus = 212,
    /// File status.
    FileStatus = 213,
    /// Help message.
    HelpMessage = 214,
    /// NAME system type, where NAME is an official system name from the
    /// list in the Assigned Numbers document.
    SystemType = 215,
    /// Service ready for new user.
    ServiceReady = 220,
    /// Service closing control connection.
    ClosingConnection = 221,
    /// Data connection open, no transfer in progress.
    DataConnectionOpened = 225,
    /// Closing data connection, requested file action successful.
    ClosingDataConnection = 226,
    /// Entering passive mode.
    EnteringPassiveMode = 227,
    /// User logged in, proceed. Logged out if appropriate.
    LoggedIn = 230,
    /// Requested file action ok.
    FileActionOk = 250,
    /// PATHNAME created.
    DirectoryOk = 257,
    /// User name ok, need password.
    NeedPassword = 331,
    /// Need account for login.
    NeedAccountToLogIn = 332,
    /// Requested file action pending further information.
    NeedInformation = 350,
    /// Service not available, closing control connection.
    ServiceUnavailable = 421,
    /// Can't open data connection.
    DataConnectionUnavailable = 425,
    /// Connection closed, transfer aborted.
    TransferAborted = 426,
    /// Requested file action not taken.
    FileActionAborted = 450,
    /// Requested file action not taken.
    LocalError = 451,
    /// Requested action not taken; insufficient storage space in system, file unavailable.
    InsufficientStorageSpace = 452,
    /// Syntax error, command unrecognized.
    CommandUnknown = 500,
    /// Syntax error in parameters or arguments.
    ParametersUnknown = 501,
    /// Command not implemented.
    CommandNotImplemented = 502,
    /// Bad sequence of commands.
    BadCommandSequence = 503,
    /// Command not implemented for that parameter.
    ParameterNotImplemented = 504,
    /// Not logged in.
    NotLoggedIn = 530,
    /// Need account for storing files.
    NeedAccountToStore = 532,
    /// Requested action not taken, file unavailable.
    FileUnavailable = 550,
    /// Requested action aborted, page type unknown.
    PageTypeUnknown = 551,
    /// Requested file action aborted, exceeded storage allocation.
    NotEnoughMemory = 552,
    /// Requested action not taken, file name not allowed.
    FilenameNotAllowed = 553,
    /// Not part of the FTP standard, generated by SFML when a received response cannot be parsed.
    InvalidResponse = 1000,
    /// Not part of the FTP standard, generated by SFML when the
    /// low-level socket connection with the server fails.
    ConnectionFailed = 1001,
    /// Not part of the FTP standard, generated by SFML when the
    /// low-level socket connection is unexpectedly closed.
    ConnectionClosed = 1002,
    /// Not part of the FTP standard, generated by SFML when a
    /// local file cannot be read or written.
    InvalidFile = 1003,
}

/// The FTP client
pub struct Ftp {
    ftp: *mut ffi::sfFtp,
}

/// Encapsulation of an Ftp Serveur response
pub struct Response {
    response: *mut ffi::sfFtpResponse,
}

/// Encapsulation of a response returning a list of filename
pub struct ListingResponse {
    listing_response: *mut ffi::sfFtpListingResponse,
}

/// Encapsulation of a response returning a directory
pub struct DirectoryResponse {
    directory_response: *mut ffi::sfFtpDirectoryResponse,
}

impl ListingResponse {
    /// Check if a FTP listing response status code means a success
    ///
    /// This function is defined for convenience, it is
    /// equivalent to testing if the status code is < 400.
    ///
    /// Return true if the status is a success, false if it is a failure
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::sfFtpListingResponse_isOk(self.listing_response) }.to_bool()
    }

    /// Get the status code of a FTP listing response
    ///
    /// Return the status code
    pub fn status(&self) -> Status {
        unsafe { mem::transmute(ffi::sfFtpListingResponse_getStatus(self.listing_response) as i32) }
    }

    /// Get the full message contained in a FTP listing response
    ///
    /// Return the response message
    pub fn message(&self) -> String {
        unsafe {
            let string = ffi::sfFtpListingResponse_getMessage(self.listing_response);
            str::from_utf8(CStr::from_ptr(string).to_bytes_with_nul()).unwrap().into()
        }
    }

    /// Return the number of directory/file names contained in a FTP listing response
    ///
    /// Return the total number of names available
    pub fn count(&self) -> u64 {
        unsafe { ffi::sfFtpListingResponse_getCount(self.listing_response) as u64 }
    }

    /// Return a directory/file name contained in a FTP listing response
    ///
    /// # Arguments
    /// * index - Index of the name to get (in range [0 .. getCount])
    ///
    /// Return the requested name
    pub fn name(&self, index: usize) -> String {
        unsafe {
            let string = ffi::sfFtpListingResponse_getName(self.listing_response, index);
            str::from_utf8(CStr::from_ptr(string).to_bytes_with_nul()).unwrap().into()
        }
    }
}

impl Drop for ListingResponse {
    fn drop(&mut self) {
        unsafe { ffi::sfFtpListingResponse_destroy(self.listing_response) }
    }
}

impl DirectoryResponse {
    /// Check if a FTP directory response status code means a success
    ///
    /// This function is defined for convenience, it is
    /// equivalent to testing if the status code is < 400.
    ///
    /// Return true if the status is a success, false if it is a failure
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::sfFtpDirectoryResponse_isOk(self.directory_response) }.to_bool()
    }

    /// Get the status code of a FTP directory response
    ///
    /// Return the status code
    pub fn status(&self) -> Status {
        unsafe {
            mem::transmute(ffi::sfFtpDirectoryResponse_getStatus(self.directory_response) as i32)
        }
    }

    /// Get the full message contained in a FTP directory response
    ///
    /// Return the response message
    pub fn message(&self) -> String {
        unsafe {
            let string = ffi::sfFtpDirectoryResponse_getMessage(self.directory_response);
            str::from_utf8(CStr::from_ptr(string).to_bytes_with_nul()).unwrap().into()
        }
    }

    /// Get the directory returned in a FTP directory response
    ///
    /// Return the directory name
    pub fn directory(&self) -> String {
        unsafe {
            let string = ffi::sfFtpDirectoryResponse_getDirectory(self.directory_response);
            str::from_utf8(CStr::from_ptr(string).to_bytes_with_nul()).unwrap().into()
        }
    }
}

impl Drop for DirectoryResponse {
    fn drop(&mut self) {
        unsafe { ffi::sfFtpDirectoryResponse_destroy(self.directory_response) }
    }
}

impl Response {
    /// Check if a FTP response status code means a success
    ///
    /// This function is defined for convenience, it is
    /// equivalent to testing if the status code is < 400.
    ///
    /// Return true if the status is a success, false if it is a failure
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::sfFtpResponse_isOk(self.response) }.to_bool()
    }

    /// Get the status code of a FTP response
    ///
    /// Return Status code
    pub fn status(&self) -> Status {
        unsafe { mem::transmute(ffi::sfFtpResponse_getStatus(self.response) as i32) }
    }

    /// Get the full message contained in a FTP response
    ///
    /// Return the response message
    pub fn message(&self) -> String {
        unsafe {
            let string = ffi::sfFtpResponse_getMessage(self.response);
            str::from_utf8(CStr::from_ptr(string).to_bytes_with_nul()).unwrap().into()
        }
    }
}

impl Drop for Response {
    fn drop(&mut self) {
        unsafe { ffi::sfFtpResponse_destroy(self.response) }
    }
}

impl Ftp {
    /// Create a new Ftp object
    ///
    /// Return Some(Ftp) or None
    pub fn new() -> Ftp {
        let ptr = unsafe { ffi::sfFtp_create() };
        assert!(!ptr.is_null(), "Failed to create Ftp");
        Ftp { ftp: ptr }
    }

    /// Connect to the specified FTP server
    ///
    /// The port should be 21, which is the standard
    /// port used by the FTP protocol. You shouldn't use a different
    /// value, unless you really know what you do.
    /// This function tries to connect to the server so it may take
    /// a while to complete, especially if the server is not
    /// reachable. To avoid blocking your application for too long,
    /// you can use a timeout. Using 0 means that the
    /// system timeout will be used (which is usually pretty long).
    ///
    /// # Arguments
    /// * server - Name or address of the FTP server to connect to
    /// * port - Port used for the connection
    /// * timeout - Maximum time to wait
    ///
    /// Return the server response to the request
    pub fn connect(&self, server: &IpAddress, port: u16, timeout: &Time) -> Response {
        Response {
            response: unsafe { ffi::sfFtp_connect(self.ftp, server.raw(), port, timeout.raw()) },
        }
    }

    /// Log in using an anonymous account
    ///
    /// Logging in is mandatory after connecting to the server.
    /// Users that are not logged in cannot perform any operation.
    ///
    /// Return the server response to the request
    pub fn login_anonymous(&self) -> Response {
        Response { response: unsafe { ffi::sfFtp_loginAnonymous(self.ftp) } }
    }

    /// Log in using a username and a password
    ///
    /// Logging in is mandatory after connecting to the server.
    /// Users that are not logged in cannot perform any operation.
    ///
    /// # Arguments
    /// * name - User name
    /// * password - Password
    ///
    /// Return the server response to the request
    pub fn login(&self, user_name: &str, password: &str) -> Response {
        let c_user_name = CString::new(user_name.as_bytes()).unwrap();
        let c_password = CString::new(password.as_bytes()).unwrap();
        Response {
            response: unsafe {
                ffi::sfFtp_login(self.ftp, c_user_name.as_ptr(), c_password.as_ptr())
            },
        }
    }

    /// Close the connection with the server
    ///
    /// Return the server response to the request
    pub fn disconnect(&self) -> Response {
        Response { response: unsafe { ffi::sfFtp_disconnect(self.ftp) } }
    }

    /// Send a null command to keep the connection alive
    ///
    /// This command is useful because the server may close the
    /// connection automatically if no command is sent.
    ///
    /// Return the server response to the request
    pub fn keep_alive(&self) -> Response {
        Response { response: unsafe { ffi::sfFtp_keepAlive(self.ftp) } }
    }

    /// Get the current working directory
    ///
    /// The working directory is the root path for subsequent
    /// operations involving directories and/or filenames.
    ///
    /// Return the server response to the request
    pub fn working_directory(&self) -> DirectoryResponse {
        DirectoryResponse {
            directory_response: unsafe { ffi::sfFtp_getWorkingDirectory(self.ftp) },
        }
    }

    /// Get the contents of the given directory
    ///
    /// This function retrieves the sub-directories and files
    /// contained in the given directory. It is not recursive.
    /// The directory parameter is relative to the current
    /// working directory.
    ///
    /// # Arguments
    /// * directory - Directory to list
    ///
    /// Return the server response to the request
    pub fn directory_listing(&self, directory: &str) -> ListingResponse {
        let c_directory = CString::new(directory.as_bytes()).unwrap();
        ListingResponse {
            listing_response: unsafe {
                ffi::sfFtp_getDirectoryListing(self.ftp, c_directory.as_ptr())
            },
        }
    }

    /// Change the current working directory
    ///
    /// The new directory must be relative to the current one.
    ///
    /// # Arguments
    /// * directory - New working directory
    ///
    /// Return the server response to the request
    pub fn change_directory(&self, directory: &str) -> Response {
        let c_directory = CString::new(directory.as_bytes()).unwrap();
        Response { response: unsafe { ffi::sfFtp_changeDirectory(self.ftp, c_directory.as_ptr()) } }
    }

    /// Go to the parent directory of the current one
    ///
    /// Return the server response to the request
    pub fn parent_directory(&self) -> Response {
        Response { response: unsafe { ffi::sfFtp_parentDirectory(self.ftp) } }
    }

    /// Create a new directory
    ///
    /// The new directory is created as a child of the current
    /// working directory.
    ///
    /// # Arguments
    /// * name - Name of the directory to create
    ///
    /// Return the server response to the request
    pub fn create_directory(&self, name: &str) -> Response {
        let c_name = CString::new(name.as_bytes()).unwrap();
        Response { response: unsafe { ffi::sfFtp_createDirectory(self.ftp, c_name.as_ptr()) } }
    }

    /// Remove an existing directory
    ///
    /// he directory to remove must be relative to the
    /// current working directory.
    /// Use this function with caution, the directory will
    /// be removed permanently!
    ///
    /// # Arguments
    /// * name - Name of the directory to remove
    ///
    /// Return the server response to the request
    pub fn delete_directory(&self, name: &str) -> Response {
        let c_name = CString::new(name.as_bytes()).unwrap();
        Response { response: unsafe { ffi::sfFtp_deleteDirectory(self.ftp, c_name.as_ptr()) } }
    }

    /// Rename an existing file
    ///
    /// The filenames must be relative to the current working
    /// directory.
    ///
    /// # Arguments
    /// * file - File to rename
    /// * newName - New name of the file
    ///
    /// Return the server response to the request
    pub fn rename_file(&self, name: &str, new_name: &str) -> Response {
        let c_name = CString::new(name.as_bytes()).unwrap();
        let c_new_name = CString::new(new_name.as_bytes()).unwrap();
        Response {
            response: unsafe {
                ffi::sfFtp_renameFile(self.ftp, c_name.as_ptr(), c_new_name.as_ptr())
            },
        }
    }

    /// Remove an existing file
    ///
    /// The file name must be relative to the current working
    /// directory.
    /// Use this function with caution, the file will be
    /// removed permanently!
    ///
    /// # Arguments
    /// * name File to remove
    ///
    /// Return the server response to the request
    pub fn delete_file(&self, name: &str) -> Response {
        let c_name = CString::new(name.as_bytes()).unwrap();
        Response { response: unsafe { ffi::sfFtp_deleteFile(self.ftp, c_name.as_ptr()) } }
    }

    /// Download a file from a FTP server
    ///
    /// The filename of the distant file is relative to the
    /// current working directory of the server, and the local
    /// destination path is relative to the current directory
    /// of your application.
    ///
    /// # Arguments
    /// * remoteFile - Filename of the distant file to download
    /// * localPath - Where to put to file on the local computer
    /// * mode - Transfer mode
    ///
    /// Return the server response to the request
    pub fn download(&self, distant_file: &str, dest_path: &str, mode: TransferMode) -> Response {
        let c_distant_file = CString::new(distant_file.as_bytes()).unwrap();
        let c_dest_path = CString::new(dest_path.as_bytes()).unwrap();
        Response {
            response: unsafe {
                ffi::sfFtp_download(self.ftp,
                                    c_distant_file.as_ptr(),
                                    c_dest_path.as_ptr(),
                                    ::std::mem::transmute(mode))
            },
        }
    }

    /// Upload a file to a FTP server
    ///
    /// The name of the local file is relative to the current
    /// working directory of your application, and the
    /// remote path is relative to the current directory of the
    /// FTP server.
    ///
    /// # Arguments
    /// * localFile - Path of the local file to upload
    /// * remotePath - Where to put to file on the server
    /// * mode - Transfer mode
    ///
    /// Return the server response to the request
    pub fn upload(&self, local_file: &str, dest_path: &str, mode: TransferMode) -> Response {
        let c_local_file = CString::new(local_file.as_bytes()).unwrap();
        let c_dest_path = CString::new(dest_path.as_bytes()).unwrap();
        Response {
            response: unsafe {
                ffi::sfFtp_upload(self.ftp,
                                  c_local_file.as_ptr(),
                                  c_dest_path.as_ptr(),
                                  ::std::mem::transmute(mode))
            },
        }
    }
}

impl Default for Ftp {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Ftp {
    fn drop(&mut self) {
        unsafe { ffi::sfFtp_destroy(self.ftp) }
    }
}
