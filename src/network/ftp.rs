/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
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
*
*
*
*
*/

use std::libc::{size_t};
use std::str;

use network::ip_address::*;
use system::time;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_void, c_char, size_t};

    use network::ip_address::csfml::*;
    use rsfml::sfTypes::sfBool;
    use network::ftp::FtpStatus;
    use network::ftp::FtpTransferMode;
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
    
    pub extern "C" {
        fn sfFtpListingResponse_destroy(ftpListingResponse : *sfFtpListingResponse) -> ();
        fn sfFtpListingResponse_isOk(ftpListingResponse : *sfFtpListingResponse) -> sfBool;
        fn sfFtpListingResponse_getStatus(ftpListingResponse : *sfFtpListingResponse) -> FtpStatus;
        fn sfFtpListingResponse_getMessage(ftpListingResponse : *sfFtpListingResponse) -> *c_char;
        fn sfFtpListingResponse_getCount(ftpListingResponse : *sfFtpListingResponse) -> size_t;
        fn sfFtpListingResponse_getName(ftpListingResponse : *sfFtpListingResponse, index : size_t) -> *c_char;
        fn sfFtpDirectoryResponse_destroy(ftpDirectoryResponse : *sfFtpDirectoryResponse) -> ();
        fn sfFtpDirectoryResponse_isOk(ftpDirectoryResponse : *sfFtpDirectoryResponse) -> sfBool;
        fn sfFtpDirectoryResponse_getStatus(ftpDirectoryResponse : *sfFtpDirectoryResponse) -> FtpStatus;
        fn sfFtpDirectoryResponse_getMessage(ftpDirectoryResponse : *sfFtpDirectoryResponse) -> *c_char;
        fn sfFtpDirectoryResponse_getDirectory(ftpDirectoryResponse : *sfFtpDirectoryResponse) -> *c_char;
        fn sfFtpResponse_destroy(ftpResponse : *sfFtpResponse) -> ();
        fn sfFtpResponse_isOk(ftpResponse : *sfFtpResponse) -> sfBool;
        fn sfFtpResponse_getStatus(ftpResponse : *sfFtpResponse) -> FtpStatus;
        fn sfFtpResponse_getMessage(ftpResponse : *sfFtpResponse) -> *c_char;
        fn sfFtp_create() -> *sfFtp;
        fn sfFtp_destroy(ftp : *sfFtp) -> ();
        fn sfFtp_connect(ftp : *sfFtp, server : sfIpAddress, port : u16, timeout : time::csfml::sfTime) -> *sfFtpResponse;
        fn sfFtp_loginAnonymous(ftp : *sfFtp) -> *sfFtpResponse;
        fn sfFtp_login(ftp : *sfFtp, userName : *c_char, password : *c_char) -> *sfFtpResponse;
        fn sfFtp_disconnect(ftp : *sfFtp) -> *sfFtpResponse;
        fn sfFtp_keepAlive(ftp : *sfFtp) -> *sfFtpResponse;
        fn sfFtp_getWorkingDirectory(ftp : *sfFtp) -> *sfFtpDirectoryResponse;
        fn sfFtp_getDirectoryListing(ftp : *sfFtp, directory : *c_char) -> sfFtpListingResponse;
        fn sfFtp_changeDirectory(ftp : *sfFtp, directory : *c_char) -> *sfFtpResponse;
        fn sfFtp_parentDirectory(ftp : *sfFtp) -> *sfFtpResponse;
        fn sfFtp_createDirectory(ftp : *sfFtp, name : *c_char) -> *sfFtpResponse;
        fn sfFtp_deleteDirectory(ftp : *sfFtp, name : *c_char) -> *sfFtpResponse;
        fn sfFtp_renameFile(ftp : *sfFtp, file : *c_char, newName : *c_char) -> *sfFtpResponse;
        fn sfFtp_deleteFile(ftp : *sfFtp, name : *c_char) -> *sfFtpResponse;
        fn sfFtp_download(ftp : *sfFtp, distantFile : *c_char, destPath : *c_char, mode : FtpTransferMode) -> *sfFtpResponse;
        fn sfFtp_upload(ftp : *sfFtp, localFile : *c_char, destPath : *c_char, mode : FtpTransferMode) -> *sfFtpResponse;
    }
}

pub enum FtpTransferMode {
    FtpBinary = 0,
    FtpAscii = 1,
    FtpEbcdic = 2
}

pub enum FtpStatus {
    // 1xx: the requested action is being initiated,
    // expect another reply before proceeding with a new command
    sfFtpRestartMarkerReply          = 110, ///< Restart marker reply
    sfFtpServiceReadySoon            = 120, ///< Service ready in N minutes
    sfFtpDataConnectionAlreadyOpened = 125, ///< Data connection already opened, transfer starting
    sfFtpOpeningDataConnection       = 150, ///< File status ok, about to open data connection

    // 2xx: the requested action has been successfully completed
    sfFtpOk                    = 200, ///< Command ok
    sfFtpPointlessCommand      = 202, ///< Command not implemented
    sfFtpSystemStatus          = 211, ///< System status, or system help reply
    sfFtpDirectoryStatus       = 212, ///< Directory status
    sfFtpFileStatus            = 213, ///< File status
    sfFtpHelpMessage           = 214, ///< Help message
    sfFtpSystemType            = 215, ///< NAME system type, where NAME is an official system name from the list in the Assigned Numbers document
    sfFtpServiceReady          = 220, ///< Service ready for new user
    sfFtpClosingConnection     = 221, ///< Service closing control connection
    sfFtpDataConnectionOpened  = 225, ///< Data connection open, no transfer in progress
    sfFtpClosingDataConnection = 226, ///< Closing data connection, requested file action successful
    sfFtpEnteringPassiveMode   = 227, ///< Entering passive mode
    sfFtpLoggedIn              = 230, ///< User logged in, proceed. Logged out if appropriate
    sfFtpFileActionOk          = 250, ///< Requested file action ok
    sfFtpDirectoryOk           = 257, ///< PATHNAME created

    // 3xx: the command has been accepted, but the requested action
    // is dormant, pending receipt of further information
    sfFtpNeedPassword       = 331, ///< User name ok, need password
    sfFtpNeedAccountToLogIn = 332, ///< Need account for login
    sfFtpNeedInformation    = 350, ///< Requested file action pending further information

    // 4xx: the command was not accepted and the requested action did not take place,
    // but the error condition is temporary and the action may be requested again
    sfFtpServiceUnavailable        = 421, ///< Service not available, closing control connection
    sfFtpDataConnectionUnavailable = 425, ///< Can't open data connection
    sfFtpTransferAborted           = 426, ///< Connection closed, transfer aborted
    sfFtpFileActionAborted         = 450, ///< Requested file action not taken
    sfFtpLocalError                = 451, ///< Requested action aborted, local error in processing
    sfFtpInsufficientStorageSpace  = 452, ///< Requested action not taken; insufficient storage space in system, file unavailable

    // 5xx: the command was not accepted and
    // the requested action did not take place
    sfFtpCommandUnknown          = 500, ///< Syntax error, command unrecognized
    sfFtpParametersUnknown       = 501, ///< Syntax error in parameters or arguments
    sfFtpCommandNotImplemented   = 502, ///< Command not implemented
    sfFtpBadCommandSequence      = 503, ///< Bad sequence of commands
    sfFtpParameterNotImplemented = 504, ///< Command not implemented for that parameter
    sfFtpNotLoggedIn             = 530, ///< Not logged in
    sfFtpNeedAccountToStore      = 532, ///< Need account for storing files
    sfFtpFileUnavailable         = 550, ///< Requested action not taken, file unavailable
    sfFtpPageTypeUnknown         = 551, ///< Requested action aborted, page type unknown
    sfFtpNotEnoughMemory         = 552, ///< Requested file action aborted, exceeded storage allocation
    sfFtpFilenameNotAllowed      = 553, ///< Requested action not taken, file name not allowed

    // 10xx: SFML custom codes
    sfFtpInvalidResponse  = 1000, ///< Response is not a valid FTP one
    sfFtpConnectionFailed = 1001, ///< Connection with server failed
    sfFtpConnectionClosed = 1002, ///< Connection with server closed
    sfFtpInvalidFile      = 1003  // Invalid file to upload / download
}

#[doc(hidden)]
pub struct Ftp {
    priv ftp : *csfml::sfFtp
}


#[doc(hidden)]
pub struct Response{
    priv response : *csfml::sfFtpResponse
}

#[doc(hidden)]
pub struct ListingResponse{
    priv listingResponse : *csfml::sfFtpListingResponse
}


#[doc(hidden)]
pub struct DirectoryResponse{
    priv directoryResponse : *csfml::sfFtpDirectoryResponse
}


impl ListingResponse {
    pub fn is_ok(&self) -> bool {
        match unsafe {csfml::sfFtpListingResponse_isOk(self.listingResponse)} {
            0 => false,
            _ => true
        }
    }
    
    pub fn get_status(&self) -> FtpStatus {
        unsafe {
            csfml::sfFtpListingResponse_getStatus(self.listingResponse)
        }
    }

    pub fn get_message(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(csfml::sfFtpListingResponse_getMessage(self.listingResponse))
        }
    }

    pub fn get_count(&self) -> u64 {
        unsafe {
            csfml::sfFtpListingResponse_getCount(self.listingResponse) as u64
        }
    }

    pub fn get_name(&self, index : u64) -> ~str {
        unsafe {
            str::raw::from_c_str(csfml::sfFtpListingResponse_getName(self.listingResponse, index as size_t))            
        }
    }
}

impl Drop for ListingResponse {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfFtpListingResponse_destroy(self.listingResponse)
        }
    }
}

impl DirectoryResponse {
    pub fn is_ok(&self) -> bool {
        match unsafe {csfml::sfFtpDirectoryResponse_isOk(self.directoryResponse)} {
            0 => false,
            _ => true
        }
    }
    
    pub fn get_status(&self) -> FtpStatus {
        unsafe {
            csfml::sfFtpDirectoryResponse_getStatus(self.directoryResponse)
        }
    }
    
    pub fn get_message(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(csfml::sfFtpDirectoryResponse_getMessage(self.directoryResponse))
        }
    }
    
    pub fn get_directory(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(csfml::sfFtpDirectoryResponse_getDirectory(self.directoryResponse))
        }
    }
}

impl Drop for DirectoryResponse {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfFtpDirectoryResponse_destroy(self.directoryResponse)
        }
    }
}

impl Response {
    pub fn is_ok(&self) -> bool {
        match unsafe {csfml::sfFtpResponse_isOk(self.response)} {
            0 => false,
            _ => true
        }
    }

    pub fn get_status(&self) -> FtpStatus {
        unsafe {
            csfml::sfFtpResponse_getStatus(self.response)
        }
    }

    pub fn get_message(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(csfml::sfFtpResponse_getMessage(self.response))
        }
    }
}

impl Drop for Response {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfFtpResponse_destroy(self.response)
        }
    }
}

impl Drop for Ftp {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfFtp_destroy(self.ftp)
        }
    }
}

/*
        fn sfFtp_create() -> *sfFtp;
        fn sfFtp_connect(ftp : *sfFtp, server : sfIpAddress, port : u16, timeout : time::csfml::sfTime) -> *sfFtpResponse;
        fn sfFtp_loginAnonymous(ftp : *sfFtp) -> *sfFtpResponse;
        fn sfFtp_login(ftp : *sfFtp, userName : *c_char, password : *c_char) -> *sfFtpResponse;
        fn sfFtp_disconnect(ftp : *sfFtp) -> *sfFtpResponse;
        fn sfFtp_keepAlive(ftp : *sfFtp) -> *sfFtpResponse;
        fn sfFtp_getWorkingDirectory(ftp : *sfFtp) -> *sfFtpDirectoryResponse;
        fn sfFtp_getDirectoryListing(ftp : *sfFtp, directory : *c_char) -> sfFtpListingResponse;
        fn sfFtp_changeDirectory(ftp : *sfFtp, directory : *c_char) -> *sfFtpResponse;
        fn sfFtp_parentDirectory(ftp : *sfFtp) -> *sfFtpResponse;
        fn sfFtp_createDirectory(ftp : *sfFtp, name : *c_char) -> *sfFtpResponse;
        fn sfFtp_deleteDirectory(ftp : *sfFtp, name : *c_char) -> *sfFtpResponse;
        fn sfFtp_renameFile(ftp : *sfFtp, file : *c_char, newName : *c_char) -> *sfFtpResponse;
        fn sfFtp_deleteFile(ftp : *sfFtp, name : *c_char) -> *sfFtpResponse;
        fn sfFtp_download(ftp : *sfFtp, distantFile : *c_char, destPath : *c_char, mode : FtpTransferMode) -> *sfFtpResponse;
        fn sfFtp_upload(ftp : *sfFtp, localFile : *c_char, destPath : *c_char, mode : FtpTransferMode) -> *sfFtpResponse;
*/