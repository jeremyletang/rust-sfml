/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
e* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

//! A FTP client.

use std::mem;
use std::c_str::CString;
use libc::size_t;

use traits::Wrappable;
use network::IpAddress;
use system::Time;

use ffi::sfml_types::{SFTRUE, SFFALSE};
use ffi::network::ftp as ffi;

/// The differents FTP modes availables.
#[deriving(Clone, PartialEq, Eq, PartialOrd, Ord, Show)]
pub enum TransferMode {
    /// Ftp Binary Mod
    FtpBinary = 0,
    /// Ftp ASCII Mod
    FtpAscii = 1,
    /// Ftp Ebcdic Mod
    FtpEbcdic = 2
}

/// The status and commands id's for FTP.
#[deriving(Clone, PartialEq, Eq, PartialOrd, Ord, Show)]
pub enum Status {
    // 1xx: the requested action is being initiated,
    // expect another reply before proceeding with a new command

    /// Restart marker reply
    RestartMarkerReply          = ffi::RESTARTMARKERREPLY as int,
    /// Service ready in N minutes
    ServiceReadySoon            = ffi::SERVICEREADYSOON as int,
    /// Data connection already opened, transfer starting
    DataConnectionAlreadyOpened = ffi::DATACONNECTIONALREADYOPENED as int,
    /// File status ok, about to open data connection
    OpeningDataConnection       = ffi::OPENINGDATACONNECTION as int,

    // 2xx: the requested action has been successfully completed

    /// Command ok
    Ok                          = ffi::OK as int,
    /// Command not implemented
    PointlessCommand            = ffi::POINTLESSCOMMAND as int,
    /// System status, or system help reply
    SystemStatus                = ffi::SYSTEMSTATUS as int,
    /// Directory status
    DirectoryStatus             = ffi::DIRECTORYSTATUS as int,
    /// File status
    FileStatus                  = ffi::FILESTATUS as int,
    /// Help message
    HelpMessage                 = ffi::HELPMESSAGE as int,
    /// NAME system type, where NAME is an official system name from the list in the Assigned Numbers document
    SystemType                  = ffi::SYSTEMTYPE as int,
    /// Service ready for new user
    ServiceReady                = ffi::SERVICEREADY as int,
    /// Service closing control connection
    ClosingConnection           = ffi::CLOSINGCONNECTION as int,
    /// Data connection open, no transfer in progress
    DataConnectionOpened        = ffi::DATACONNECTIONOPENED as int,
    /// Closing data connection, requested file action successful
    ClosingDataConnection       = ffi::CLOSINGDATACONNECTION as int,
    /// Entering passive mode
    EnteringPassiveMode         = ffi::ENTERINGPASSIVEMODE as int,
    /// User logged in, proceed. Logged out if appropriate
    LoggedIn                    = ffi::LOGGEDIN as int,
    /// Requested file action ok
    FileActionOk                = ffi::FILEACTIONOK as int,
    /// PATHNAME created
    DirectoryOk                 = ffi::DIRECTORYOK as int,

    // 3xx: the command has been accepted, but the requested action
    // is dormant, pending receipt of further information
    /// User name ok, need password
    NeedPassword                = ffi::NEEDPASSWORD as int,
    /// Need account for login
    NeedAccountToLogIn          = ffi::NEEDACCOUNTTOLOGIN as int,
    /// Requested file action pending further information
    NeedInformation             = ffi::NEEDINFORMATION as int,

    // 4xx: the command was not accepted and the requested action did not take place,
    // but the error condition is temporary and the action may be requested again

    /// Service not available, closing control connection
    ServiceUnavailable          = ffi::SERVICEUNAVAILABLE as int,
    /// Can't open data connection
    DataConnectionUnavailable   = ffi::DATACONNECTIONUNAVAILABLE as int,
    /// Connection closed, transfer aborted
    TransferAborted             = ffi::TRANSFERABORTED as int,
    /// Requested file action not taken
    FileActionAborted           = ffi::FILEACTIONABORTED as int,
    /// Requested action aborted, local error in processing
    LocalError                  = ffi::LOCALERROR as int,
    /// Requested action not taken; insufficient storage space in system, file unavailable
    InsufficientStorageSpace    = ffi::INSUFFICIENTSTORAGESPACE as int,

    // 5xx: the command was not accepted and
    // the requested action did not take place
    /// Syntax error, command unrecognized
    CommandUnknown              = ffi::COMMANDUNKNOWN as int,
    /// Syntax error in parameters or arguments
    ParametersUnknown           = ffi::PARAMETERSUNKNOWN as int,
    /// Command not implemented
    CommandNotImplemented       = ffi::COMMANDNOTIMPLEMENTED as int,
    /// Bad sequence of commands
    BadCommandSequence          = ffi::BADCOMMANDSEQUENCE as int,
    /// Command not implemented for that parameter
    ParameterNotImplemented     = ffi::PARAMETERNOTIMPLEMENTED as int,
    /// Not logged in
    NotLoggedIn                 = ffi::NOTLOGGEDIN as int,
    /// Need account for storing files
    NeedAccountToStore          = ffi::NEEDACCOUNTTOSTORE as int,
    /// Requested action not taken, file unavailable
    FileUnavailable             = ffi::FILEUNAVAILABLE as int,
    /// Requested action aborted, page type unknown
    PageTypeUnknown             = ffi::PAGETYPEUNKNOWN as int,
    /// Requested file action aborted, exceeded storage allocation
    NotEnoughMemory             = ffi::NOTENOUGHMEMORY as int,
    /// Requested action not taken, file name not allowed
    FilenameNotAllowed          = ffi::FILENAMENOTALLOWED as int,

    // 10xx: SFML custom codes
    /// Response is not a valid FTP one
    InvalidResponse             = ffi::INVALIDRESPONSE as int,
    /// Connection with server failed
    ConnectionFailed            = ffi::CONNECTIONFAILED as int,
    /// Connection with server closed
    ConnectionClosed            = ffi::CONNECTIONCLOSED as int,
    /// Invalid file to upload / download
    InvalidFile                 = ffi::INVALIDFILE as int
}

/// The FTP client
pub struct Ftp {
    #[doc(hidden)]
    ftp: *mut ffi::sfFtp
}

/// Encapsulation of an Ftp Serveur response
pub struct Response {
    #[doc(hidden)]
    response: *mut ffi::sfFtpResponse
}

/// Encapsulation of a response returning a list of filename
pub struct ListingResponse{
    #[doc(hidden)]
    listing_response: *mut ffi::sfFtpListingResponse
}

/// Encapsulation of a response returning a directory
pub struct DirectoryResponse{
    #[doc(hidden)]
    directory_response: *mut ffi::sfFtpDirectoryResponse
}

impl ListingResponse {
    /// Check if a FTP listing response status code means a success
    ///
    /// This function is defined for convenience, it is
    /// equivalent to testing if the status code is < 400.
    ///
    /// Return true if the status is a success, false if it is a failure
    pub fn is_ok(&self) -> bool {
        match unsafe { ffi::sfFtpListingResponse_isOk(self.listing_response) } {
            SFFALSE => false,
            SFTRUE  => true
        }
    }

    /// Get the status code of a FTP listing response
    ///
    /// Return the status code
    pub fn get_status(&self) -> Status {
        unsafe {
            mem::transmute(ffi::sfFtpListingResponse_getStatus(self.listing_response) as i16)
        }
    }

    /// Get the full message contained in a FTP listing response
    ///
    /// Return the response message
    pub fn get_message(&self) -> String {
        unsafe {
            CString::new(ffi::sfFtpListingResponse_getMessage(self.listing_response),
                         false).as_str().unwrap().to_string()
        }
    }

    /// Return the number of directory/file names contained in a FTP listing response
    ///
    /// Return the total number of names available
    pub fn get_count(&self) -> u64 {
        unsafe {
            ffi::sfFtpListingResponse_getCount(self.listing_response) as u64
        }
    }

    /// Return a directory/file name contained in a FTP listing response
    ///
    /// # Arguments
    /// * index - Index of the name to get (in range [0 .. getCount])
    ///
    /// Return the requested name
    pub fn get_name(&self, index: u64) -> String {
        unsafe {
            CString::new(ffi::sfFtpListingResponse_getName(self.listing_response, index as size_t),
                         false).as_str().unwrap().to_string()
        }
    }
}

impl Drop for ListingResponse {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfFtpListingResponse_destroy(self.listing_response)
        }
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
        match unsafe { ffi::sfFtpDirectoryResponse_isOk(self.directory_response) } {
            SFFALSE => false,
            SFTRUE => true
        }
    }

    /// Get the status code of a FTP directory response
    ///
    /// Return the status code
    pub fn get_status(&self) -> Status {
        unsafe {
            mem::transmute(ffi::sfFtpDirectoryResponse_getStatus(self.directory_response) as i16)
        }
    }

    /// Get the full message contained in a FTP directory response
    ///
    /// Return the response message
    pub fn get_message(&self) -> String {
        unsafe {
            CString::new(ffi::sfFtpDirectoryResponse_getMessage(self.directory_response),
                         false).as_str().unwrap().to_string()
        }
    }

    /// Get the directory returned in a FTP directory response
    ///
    /// Return the directory name
    pub fn get_directory(&self) -> String {
        unsafe {
            CString::new(ffi::sfFtpDirectoryResponse_getDirectory(self.directory_response),
                         false).as_str().unwrap().to_string()
        }
    }
}

impl Drop for DirectoryResponse {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfFtpDirectoryResponse_destroy(self.directory_response)
        }
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
        match unsafe { ffi::sfFtpResponse_isOk(self.response) } {
            SFFALSE => false,
            SFTRUE => true
        }
    }

    /// Get the status code of a FTP response
    ///
    /// Return Status code
    pub fn get_status(&self) -> Status {
        unsafe {
            mem::transmute(ffi::sfFtpResponse_getStatus(self.response) as i16)
        }
    }

    /// Get the full message contained in a FTP response
    ///
    /// Return the response message
    pub fn get_message(&self) -> String {
        unsafe {
            CString::new(ffi::sfFtpResponse_getMessage(self.response),
                         false).as_str().unwrap().to_string()
        }
    }
}

impl Drop for Response {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfFtpResponse_destroy(self.response)
        }
    }
}

impl Ftp {
    /// Create a new Ftp object
    ///
    /// Return Some(Ftp) or None
    pub fn new() -> Option<Ftp> {
        let ptr = unsafe { ffi::sfFtp_create() };
        if ptr.is_null() {
            None
        } else {
            Some(Ftp {
                ftp: ptr
            })
        }
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
            response: unsafe { ffi::sfFtp_connect(self.ftp, server.unwrap(), port, timeout.unwrap()) }
        }
    }

    /// Log in using an anonymous account
    ///
    /// Logging in is mandatory after connecting to the server.
    /// Users that are not logged in cannot perform any operation.
    ///
    /// Return the server response to the request
    pub fn login_anonymous(&self) -> Response {
        Response {
            response: unsafe { ffi::sfFtp_loginAnonymous(self.ftp) }
        }
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
        let c_user_name = user_name.to_c_str();
        let c_password = password.to_c_str();
        Response {
            response: unsafe { ffi::sfFtp_login(self.ftp,
                                                 c_user_name.into_inner(),
                                                 c_password.into_inner()) }
        }
    }

    /// Close the connection with the server
    ///
    /// Return the server response to the request
    pub fn disconnect(&self) -> Response {
        Response {
            response: unsafe { ffi::sfFtp_disconnect(self.ftp) }
        }
    }

    /// Send a null command to keep the connection alive
    ///
    /// This command is useful because the server may close the
    /// connection automatically if no command is sent.
    ///
    /// Return the server response to the request
    pub fn keep_alive(&self) -> Response {
        Response {
            response: unsafe { ffi::sfFtp_keepAlive(self.ftp) }
        }
    }

    /// Get the current working directory
    ///
    /// The working directory is the root path for subsequent
    /// operations involving directories and/or filenames.
    ///
    /// Return the server response to the request
    pub fn get_working_directory(&self) -> DirectoryResponse {
        DirectoryResponse {
            directory_response: unsafe { ffi::sfFtp_getWorkingDirectory(self.ftp) }
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
    pub fn get_directory_listing(&self, directory: &str) -> ListingResponse {
        let c_directory = directory.to_c_str();
        ListingResponse {
            listing_response: unsafe { ffi::sfFtp_getDirectoryListing(self.ftp,
                                                                       c_directory.into_inner()) }
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
        let c_directory = directory.to_c_str();
        Response {
            response: unsafe { ffi::sfFtp_changeDirectory(self.ftp,
                                                           c_directory.into_inner()) }
        }
    }

    /// Go to the parent directory of the current one
    ///
    /// Return the server response to the request
    pub fn parent_directory(&self) -> Response {
        Response {
            response: unsafe { ffi::sfFtp_parentDirectory(self.ftp) }
        }
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
        let c_name = name.to_c_str();
        Response {
            response: unsafe { ffi::sfFtp_createDirectory(self.ftp,
                                                           c_name.into_inner()) }
        }
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
        let c_name = name.to_c_str();
        Response {
            response: unsafe { ffi::sfFtp_deleteDirectory(self.ftp,
                                                           c_name.into_inner()) }
        }
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
        let c_name = name.to_c_str();
        let c_new_name = new_name.to_c_str();
        Response {
            response: unsafe { ffi::sfFtp_renameFile(self.ftp,
                                                      c_name.into_inner(),
                                                      c_new_name.into_inner()) }
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
        let c_name = name.to_c_str();
        Response {
            response: unsafe { ffi::sfFtp_deleteFile(self.ftp,
                                                      c_name.into_inner()) }
        }
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
        let c_distant_file = distant_file.to_c_str();
        let c_dest_path = dest_path.to_c_str();
        Response {
            response: unsafe { ffi::sfFtp_download(self.ftp,
                                                    c_distant_file.into_inner(),
                                                    c_dest_path.into_inner(),
                                                    mode as ffi::TransferMode) }
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
        let c_local_file = local_file.to_c_str();
        let c_dest_path = dest_path.to_c_str();
        Response {
            response: unsafe { ffi::sfFtp_upload(self.ftp,
                                                  c_local_file.into_inner(),
                                                  c_dest_path.into_inner(),
                                                  mode as ffi::TransferMode) }
        }
    }
}

impl Drop for Ftp {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfFtp_destroy(self.ftp)
        }
    }
}
