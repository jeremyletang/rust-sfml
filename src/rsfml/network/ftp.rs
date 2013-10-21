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

/*!
* A FTP client.
*
*
*
*/

use std::libc::{size_t};
use std::str;

use traits::wrappable::Wrappable;
use network::ip_address::*;
use system::time::Time;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_void, c_char, size_t};

    use network::ip_address::ffi::*;
    use sfml_types::SfBool;
    use network::ftp::Status;
    use network::ftp::TransferMode;
    use system::time;

    pub struct sfFtp {
        This : *c_void
    }

    pub struct sfFtpDirectoryResponse {
        This : *c_void
    }

    pub struct sfFtpListingResponse {
        This : *c_void
    }

    pub struct sfFtpResponse {
        This : *c_void
    }
    
    extern "C" {
        pub fn sfFtpListingResponse_destroy(ftpListingResponse : *sfFtpListingResponse) -> ();
        pub fn sfFtpListingResponse_isOk(ftpListingResponse : *sfFtpListingResponse) -> SfBool;
        pub fn sfFtpListingResponse_getStatus(ftpListingResponse : *sfFtpListingResponse) -> Status;
        pub fn sfFtpListingResponse_getMessage(ftpListingResponse : *sfFtpListingResponse) -> *c_char;
        pub fn sfFtpListingResponse_getCount(ftpListingResponse : *sfFtpListingResponse) -> size_t;
        pub fn sfFtpListingResponse_getName(ftpListingResponse : *sfFtpListingResponse, index : size_t) -> *c_char;
        pub fn sfFtpDirectoryResponse_destroy(ftpDirectoryResponse : *sfFtpDirectoryResponse) -> ();
        pub fn sfFtpDirectoryResponse_isOk(ftpDirectoryResponse : *sfFtpDirectoryResponse) -> SfBool;
        pub fn sfFtpDirectoryResponse_getStatus(ftpDirectoryResponse : *sfFtpDirectoryResponse) -> Status;
        pub fn sfFtpDirectoryResponse_getMessage(ftpDirectoryResponse : *sfFtpDirectoryResponse) -> *c_char;
        pub fn sfFtpDirectoryResponse_getDirectory(ftpDirectoryResponse : *sfFtpDirectoryResponse) -> *c_char;
        pub fn sfFtpResponse_destroy(ftpResponse : *sfFtpResponse) -> ();
        pub fn sfFtpResponse_isOk(ftpResponse : *sfFtpResponse) -> SfBool;
        pub fn sfFtpResponse_getStatus(ftpResponse : *sfFtpResponse) -> Status;
        pub fn sfFtpResponse_getMessage(ftpResponse : *sfFtpResponse) -> *c_char;
        pub fn sfFtp_create() -> *sfFtp;
        pub fn sfFtp_destroy(ftp : *sfFtp) -> ();
        pub fn sfFtp_connect(ftp : *sfFtp, server : sfIpAddress, port : u16, timeout : time::ffi::sfTime) -> *sfFtpResponse;
        pub fn sfFtp_loginAnonymous(ftp : *sfFtp) -> *sfFtpResponse;
        pub fn sfFtp_login(ftp : *sfFtp, userName : *c_char, password : *c_char) -> *sfFtpResponse;
        pub fn sfFtp_disconnect(ftp : *sfFtp) -> *sfFtpResponse;
        pub fn sfFtp_keepAlive(ftp : *sfFtp) -> *sfFtpResponse;
        pub fn sfFtp_getWorkingDirectory(ftp : *sfFtp) -> *sfFtpDirectoryResponse;
        pub fn sfFtp_getDirectoryListing(ftp : *sfFtp, directory : *c_char) -> *sfFtpListingResponse;
        pub fn sfFtp_changeDirectory(ftp : *sfFtp, directory : *c_char) -> *sfFtpResponse;
        pub fn sfFtp_parentDirectory(ftp : *sfFtp) -> *sfFtpResponse;
        pub fn sfFtp_createDirectory(ftp : *sfFtp, name : *c_char) -> *sfFtpResponse;
        pub fn sfFtp_deleteDirectory(ftp : *sfFtp, name : *c_char) -> *sfFtpResponse;
        pub fn sfFtp_renameFile(ftp : *sfFtp, file : *c_char, newName : *c_char) -> *sfFtpResponse;
        pub fn sfFtp_deleteFile(ftp : *sfFtp, name : *c_char) -> *sfFtpResponse;
        pub fn sfFtp_download(ftp : *sfFtp, distantFile : *c_char, destPath : *c_char, mode : TransferMode) -> *sfFtpResponse;
        pub fn sfFtp_upload(ftp : *sfFtp, localFile : *c_char, destPath : *c_char, mode : TransferMode) -> *sfFtpResponse;
    }
}

/// The differents FTP modes availables.
pub enum TransferMode {
    /// Ftp Binary Mod
    FtpBinary = 0,
    /// Ftp ASCII Mod
    FtpAscii = 1,
    /// Ftp Ebcdic Mod
    FtpEbcdic = 2
}

/// The status and commands id's for FTP.
pub enum Status {
    // 1xx: the requested action is being initiated,
    // expect another reply before proceeding with a new command

    /// Restart marker reply
    RestartMarkerReply          = 110, 
    /// Service ready in N minutes
    ServiceReadySoon            = 120, 
    /// Data connection already opened, transfer starting
    DataConnectionAlreadyOpened = 125, 
    /// File status ok, about to open data connection
    OpeningDataConnection       = 150, 

    // 2xx: the requested action has been successfully completed

    /// Command ok
    Ok                    = 200, 
    /// Command not implemented
    PointlessCommand      = 202, 
    /// System status, or system help reply
    SystemStatus          = 211, 
    /// Directory status
    DirectoryStatus       = 212, 
    /// File status
    FileStatus            = 213, 
    /// Help message
    HelpMessage           = 214, 
    /// NAME system type, where NAME is an official system name from the list in the Assigned Numbers document
    SystemType            = 215, 
    /// Service ready for new user
    ServiceReady          = 220, 
    /// Service closing control connection
    ClosingConnection     = 221, 
    /// Data connection open, no transfer in progress
    DataConnectionOpened  = 225, 
    /// Closing data connection, requested file action successful
    ClosingDataConnection = 226, 
    /// Entering passive mode
    EnteringPassiveMode   = 227, 
    /// User logged in, proceed. Logged out if appropriate
    LoggedIn              = 230, 
    /// Requested file action ok
    FileActionOk          = 250, 
    /// PATHNAME created
    DirectoryOk           = 257, 

    // 3xx: the command has been accepted, but the requested action
    // is dormant, pending receipt of further information
    /// User name ok, need password
    NeedPassword       = 331, 
    /// Need account for login
    NeedAccountToLogIn = 332, 
    /// Requested file action pending further information
    NeedInformation    = 350, 

    // 4xx: the command was not accepted and the requested action did not take place,
    // but the error condition is temporary and the action may be requested again

    /// Service not available, closing control connection
    ServiceUnavailable        = 421, 
    /// Can't open data connection
    DataConnectionUnavailable = 425, 
    /// Connection closed, transfer aborted
    TransferAborted           = 426, 
    /// Requested file action not taken
    FileActionAborted         = 450, 
    /// Requested action aborted, local error in processing
    LocalError                = 451, 
    /// Requested action not taken; insufficient storage space in system, file unavailable
    InsufficientStorageSpace  = 452, 

    // 5xx: the command was not accepted and
    // the requested action did not take place
    /// Syntax error, command unrecognized
    CommandUnknown          = 500, 
    /// Syntax error in parameters or arguments
    ParametersUnknown       = 501, 
    /// Command not implemented
    CommandNotImplemented   = 502, 
    /// Bad sequence of commands
    BadCommandSequence      = 503, 
    /// Command not implemented for that parameter
    ParameterNotImplemented = 504, 
    /// Not logged in
    NotLoggedIn             = 530, 
    /// Need account for storing files
    NeedAccountToStore      = 532, 
    /// Requested action not taken, file unavailable
    FileUnavailable         = 550, 
    /// Requested action aborted, page type unknown
    PageTypeUnknown         = 551, 
    /// Requested file action aborted, exceeded storage allocation
    NotEnoughMemory         = 552, 
    /// Requested action not taken, file name not allowed
    FilenameNotAllowed      = 553, 

    // 10xx: SFML custom codes
    /// Response is not a valid FTP one
    InvalidResponse  = 1000, 
    /// Connection with server failed
    ConnectionFailed = 1001, 
    /// Connection with server closed
    ConnectionClosed = 1002, 
    /// Invalid file to upload / download
    InvalidFile      = 1003  
}

pub struct Ftp {
    #[doc(hidden)]
    priv ftp : *ffi::sfFtp
}


pub struct Response{
    #[doc(hidden)]
    priv response : *ffi::sfFtpResponse
}

pub struct ListingResponse{
    #[doc(hidden)]
    priv listing_response : *ffi::sfFtpListingResponse
}


pub struct DirectoryResponse{
    #[doc(hidden)]
    priv directory_response : *ffi::sfFtpDirectoryResponse
}

impl ListingResponse {
    /**
    * Check if a FTP listing response status code means a success
    *
    * This function is defined for convenience, it is
    * equivalent to testing if the status code is < 400.
    *
    * Return true if the status is a success, false if it is a failure
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_ok(&self) -> bool {
        match unsafe { ffi::sfFtpListingResponse_isOk(self.listing_response) } {
            0 => false,
            _ => true
        }
    }
    
    /**
    * Get the status code of a FTP listing response
    *
    * Return the status code
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_status(&self) -> Status {
        unsafe {
            ffi::sfFtpListingResponse_getStatus(self.listing_response)
        }
    }

    /**
    * Get the full message contained in a FTP listing response
    *
    * Return the response message
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_message(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(ffi::sfFtpListingResponse_getMessage(self.listing_response))
        }
    }

    /**
    * Return the number of directory/file names contained in a FTP listing response
    *
    * Return the total number of names available
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_count(&self) -> u64 {
        unsafe {
            ffi::sfFtpListingResponse_getCount(self.listing_response) as u64
        }
    }

    /**
    * Return a directory/file name contained in a FTP listing response
    *
    * # Arguments
    * * index - Index of the name to get (in range [0 .. getCount])
    *
    * Return the requested name
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_name(&self, index : u64) -> ~str {
        unsafe {
            str::raw::from_c_str(ffi::sfFtpListingResponse_getName(self.listing_response, index as size_t))            
        }
    }
}

impl Drop for ListingResponse {
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfFtpListingResponse_destroy(self.listing_response)
        }
    }
}

impl DirectoryResponse {
    /**
    * Check if a FTP directory response status code means a success
    *
    * This function is defined for convenience, it is
    * equivalent to testing if the status code is < 400.
    *
    * Return true if the status is a success, false if it is a failure
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_ok(&self) -> bool {
        match unsafe { ffi::sfFtpDirectoryResponse_isOk(self.directory_response) } {
            0 => false,
            _ => true
        }
    }
    
    /**
    * Get the status code of a FTP directory response
    *
    * Return the status code
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_status(&self) -> Status {
        unsafe {
            ffi::sfFtpDirectoryResponse_getStatus(self.directory_response)
        }
    }
    
    /**
    * Get the full message contained in a FTP directory response
    *
    * Return the response message
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_message(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(ffi::sfFtpDirectoryResponse_getMessage(self.directory_response))
        }
    }
    
    /**
    * Get the directory returned in a FTP directory response
    *
    * Return the directory name
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_directory(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(ffi::sfFtpDirectoryResponse_getDirectory(self.directory_response))
        }
    }
}

impl Drop for DirectoryResponse {
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfFtpDirectoryResponse_destroy(self.directory_response)
        }
    }
}

impl Response {
    /**
    * Check if a FTP response status code means a success
    *
    * This function is defined for convenience, it is
    * equivalent to testing if the status code is < 400.
    *
    * Return true if the status is a success, false if it is a failure
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_ok(&self) -> bool {
        match unsafe { ffi::sfFtpResponse_isOk(self.response) } {
            0 => false,
            _ => true
        }
    }

    /**
    * Get the status code of a FTP response
    *
    * Return Status code
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_status(&self) -> Status {
        unsafe {
            ffi::sfFtpResponse_getStatus(self.response)
        }
    }

    /**
    * Get the full message contained in a FTP response
    *
    * Return the response message
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_message(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(ffi::sfFtpResponse_getMessage(self.response))
        }
    }
}

impl Drop for Response {
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfFtpResponse_destroy(self.response)
        }
    }
}

impl Ftp {
    /**
    * Create a new Ftp object
    *
    * Return a new option to Ftp object or None
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new() -> Ftp {
        Ftp {
            ftp : unsafe { ffi::sfFtp_create() }
        }
    }

    /**
    * Connect to the specified FTP server
    *
    * The port should be 21, which is the standard
    * port used by the FTP protocol. You shouldn't use a different
    * value, unless you really know what you do.
    * This function tries to connect to the server so it may take
    * a while to complete, especially if the server is not
    * reachable. To avoid blocking your application for too long,
    * you can use a timeout. Using 0 means that the
    * system timeout will be used (which is usually pretty long).
    *
    * # Arguments
    * * server - Name or address of the FTP server to connect to
    * * port - Port used for the connection
    * * timeout - Maximum time to wait
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn connect(&self, server : &IpAddress, port : u16, timeout : &Time) -> Response {
        Response {
            response : unsafe { ffi::sfFtp_connect(self.ftp, server.unwrap(), port, timeout.unwrap()) }
        }
    }

    /**
    * Log in using an anonymous account
    *
    * Logging in is mandatory after connecting to the server.
    * Users that are not logged in cannot perform any operation.
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn login_anonymous(&self) -> Response {
        Response {
            response : unsafe { ffi::sfFtp_loginAnonymous(self.ftp) }
        }
    }

    /**
    * Log in using a username and a password
    *
    * Logging in is mandatory after connecting to the server.
    * Users that are not logged in cannot perform any operation.
    *
    * # Arguments
    * * name - User name
    * * password - Password
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn login(&self, user_name : ~str, password : ~str) -> Response {
        let c_user_name = user_name.to_c_str();
        let c_password = password.to_c_str();
        Response {
            response : unsafe { ffi::sfFtp_login(self.ftp, c_user_name.unwrap(), c_password.unwrap()) }
        }
    }
    
    /**
    * Close the connection with the server
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn disconnect(&self) -> Response {
        Response {
            response : unsafe { ffi::sfFtp_disconnect(self.ftp) }
        }
    }

    /**
    * Send a null command to keep the connection alive
    *
    * This command is useful because the server may close the
    * connection automatically if no command is sent.
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn keep_alive(&self) -> Response {
        Response {
            response : unsafe { ffi::sfFtp_keepAlive(self.ftp) }
        }
    }

    /**
    * Get the current working directory
    *
    * The working directory is the root path for subsequent
    * operations involving directories and/or filenames.
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_working_directory(&self) -> DirectoryResponse {
        DirectoryResponse {
            directory_response : unsafe { ffi::sfFtp_getWorkingDirectory(self.ftp) } 
        }
    }

    /**
    * Get the contents of the given directory
    *
    * This function retrieves the sub-directories and files
    * contained in the given directory. It is not recursive.
    * The directory parameter is relative to the current
    * working directory.
    *
    * # Arguments
    * * directory - Directory to list
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_directory_listing(&self, directory : ~str) -> ListingResponse {
        let c_directory = directory.to_c_str();
        ListingResponse {
            listing_response : unsafe { ffi::sfFtp_getDirectoryListing(self.ftp, c_directory.unwrap()) }
        }
    }

    /**
    * Change the current working directory
    *
    * The new directory must be relative to the current one.
    *
    * # Arguments
    * * directory - New working directory
    * 
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn change_directory(&self, directory : ~str) -> Response {
        let c_directory = directory.to_c_str();
        Response {
            response : unsafe { ffi::sfFtp_changeDirectory(self.ftp, c_directory.unwrap()) }
        }
    }

    /**
    * Go to the parent directory of the current one
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn parent_directory(&self) -> Response {
        Response {
            response : unsafe { ffi::sfFtp_parentDirectory(self.ftp) }
        }
    }

    /**
    * Create a new directory
    *
    * The new directory is created as a child of the current
    * working directory.
    *
    * # Arguments
    * * name - Name of the directory to create
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn create_directory(&self, name : ~str) -> Response {
        let c_name = name.to_c_str();
        Response { 
            response : unsafe { ffi::sfFtp_createDirectory(self.ftp, c_name.unwrap()) }
        }
    }

    /**
    * Remove an existing directory
    *
    * he directory to remove must be relative to the
    * current working directory.
    * Use this function with caution, the directory will
    * be removed permanently!
    *
    * # Arguments
    * * name - Name of the directory to remove
    * 
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn delete_directory(&self, name : ~str) -> Response {
        let c_name = name.to_c_str();
        Response {
            response : unsafe { ffi::sfFtp_deleteDirectory(self.ftp, c_name.unwrap()) }
        }
    }
    
    /**
    * Rename an existing file
    *
    * The filenames must be relative to the current working
    * directory.
    *
    * # Arguments
    * * file - File to rename
    * * newName - New name of the file
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn rename_file(&self, name : ~str, new_name : ~str) -> Response {
        let c_name = name.to_c_str();
        let c_new_name = new_name.to_c_str();
        Response {
            response : unsafe { ffi::sfFtp_renameFile(self.ftp, c_name.unwrap(), c_new_name.unwrap()) } 
        }
    }

    /**
    * Remove an existing file
    *
    * The file name must be relative to the current working
    * directory.
    * Use this function with caution, the file will be
    * removed permanently!
    *
    * # Arguments
    * * name File to remove
    * 
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn delete_file(&self, name : ~str) -> Response {
        let c_name = name.to_c_str();
        Response {
            response : unsafe { ffi::sfFtp_deleteFile(self.ftp, c_name.unwrap()) }
        }
    }

    /**
    * Download a file from a FTP server
    *
    * The filename of the distant file is relative to the
    * current working directory of the server, and the local
    * destination path is relative to the current directory
    * of your application.
    *
    * # Arguments
    * * remoteFile - Filename of the distant file to download
    * * localPath - Where to put to file on the local computer
    * * mode - Transfer mode
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn download(&self, distant_file : ~str, dest_path : ~str, mode : TransferMode) -> Response {
        let c_distant_file = distant_file.to_c_str();
        let c_dest_path = dest_path.to_c_str();
        Response { 
            response : unsafe { ffi::sfFtp_download(self.ftp, c_distant_file.unwrap(), c_dest_path.unwrap(), mode) }
        }
    }

    /**
    * Upload a file to a FTP server
    *
    * The name of the local file is relative to the current
    * working directory of your application, and the
    * remote path is relative to the current directory of the
    * FTP server.
    * 
    * # Arguments
    * * localFile - Path of the local file to upload
    * * remotePath - Where to put to file on the server
    * * mode - Transfer mode
    *
    * Return the server response to the request
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn upload(&self, local_file : ~str, dest_path : ~str, mode : TransferMode) -> Response {
        let c_local_file = local_file.to_c_str();
        let c_dest_path = dest_path.to_c_str();
        Response { 
            response : unsafe { ffi::sfFtp_upload(self.ftp, c_local_file.unwrap(), c_dest_path.unwrap(), mode) }
        }
    }

}

impl Drop for Ftp {
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfFtp_destroy(self.ftp)
        }
    }
}
