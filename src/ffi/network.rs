/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
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

pub mod socket_status {

    use libc::c_int;

    pub type SocketStatus = c_int;
    pub const SOCKETNONE:          SocketStatus = 0;
    pub const SOCKETNOTREADY:      SocketStatus = 1;
    pub const SOCKETDISCONNECTED:  SocketStatus = 2;
    pub const SOCKETERROR:         SocketStatus = 3;
}

pub mod packet {
    use libc::{c_void, size_t, c_float, c_double, c_char};

    use ffi::sfml_types::SfBool;

    #[repr(C)]
    pub struct sfPacket {
        this: *mut c_void
    }

    extern "C" {
        pub fn sfPacket_create() -> *mut sfPacket;
        pub fn sfPacket_copy(pack: *mut sfPacket) -> *mut sfPacket;
        pub fn sfPacket_destroy(pack: *mut sfPacket) -> ();
        //fn sfPacket_append(pack: *mut sfPacket, data: *mut c_void, sizeInBytes: size_t) -> ();
        pub fn sfPacket_clear(pack: *mut sfPacket) -> ();
        //fn sfPacket_getData(pack: *mut sfPacket) -> *mut c_void;
        pub fn sfPacket_getDataSize(pack: *mut sfPacket) -> size_t;
        pub fn sfPacket_endOfPacket(pack: *mut sfPacket) -> SfBool;
        pub fn sfPacket_canRead(pack: *mut sfPacket) -> SfBool;
        pub fn sfPacket_readBool(pack: *mut sfPacket) -> SfBool;
        pub fn sfPacket_readInt8(pack: *mut sfPacket) -> i8;
        pub fn sfPacket_readUint8(pack: *mut sfPacket) -> u8;
        pub fn sfPacket_readInt16(pack: *mut sfPacket) -> i16;
        pub fn sfPacket_readUint16(pack: *mut sfPacket) -> u16;
        pub fn sfPacket_readInt32(pack: *mut sfPacket) -> i32;
        pub fn sfPacket_readUint32(pack: *mut sfPacket) -> u32;
        pub fn sfPacket_readFloat(pack: *mut sfPacket) -> c_float;
        pub fn sfPacket_readDouble(pack: *mut sfPacket) -> c_double;
        pub fn sfPacket_readString(pack: *mut sfPacket, string: *mut u8) -> ();
        //fn sfPacket_readWideString(pack: *mut sfPacket, string: *mut wchar_t) -> ();
        pub fn sfPacket_writeBool(pack: *mut sfPacket, data: SfBool) -> ();
        pub fn sfPacket_writeInt8(pack: *mut sfPacket, data: i8) -> ();
        pub fn sfPacket_writeUint8(pack: *mut sfPacket, data: u8) -> ();
        pub fn sfPacket_writeInt16(pack: *mut sfPacket, data: i16) -> ();
        pub fn sfPacket_writeUint16(pack: *mut sfPacket, data: u16) -> ();
        pub fn sfPacket_writeInt32(pack: *mut sfPacket, data: i32) -> ();
        pub fn sfPacket_writeUint32(pack: *mut sfPacket, data: u32) -> ();
        pub fn sfPacket_writeFloat(pack: *mut sfPacket, data: c_float) -> ();
        pub fn sfPacket_writeDouble(pack: *mut sfPacket, data: c_double) -> ();
        pub fn sfPacket_writeString(pack: *mut sfPacket, string: *const c_char) -> ();
        //fn sfPacket_writeWideString(pack: *mut sfPacket, string: *mut wchar_t) -> ();
    }
}

pub mod ip_address {
    use libc::c_char;

    use ffi::system::time::sfTime;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct sfIpAddress {
        c1: u8,
        c2: u8,
        c3: u8,
        c4: u8,
        c5: u8,
        c6: u8,
        c7: u8,
        c8: u8,
        c9: u8,
        c10: u8,
        c11: u8,
        c12: u8,
        c13: u8,
        c14: u8,
        c15: u8,
        c16: u8
    }

    extern "C" {
        pub fn sfIpAddress_fromString(address: *const c_char) -> sfIpAddress;
        pub fn sfIpAddress_fromBytes(byte0: u8, byte1: u8, byte2: u8, byte3: u8) -> sfIpAddress;
        pub fn sfIpAddress_fromInteger(address: u32) -> sfIpAddress;
        pub fn sfIpAddress_toString(address: sfIpAddress, string: *mut u8) -> ();
        pub fn sfIpAddress_toInteger(address: sfIpAddress) -> u32;
        pub fn sfIpAddress_getLocalAddress() -> sfIpAddress;
        pub fn sfIpAddress_getPublicAddress(timeout: sfTime) -> sfIpAddress;
    }
}

pub mod tcp_listener {
    use libc::{c_void};

    use ffi::network::tcp_socket::sfTcpSocket;
    use ffi::network::socket_status::SocketStatus;
    use ffi::sfml_types::SfBool;

    #[repr(C)]
    pub struct sfTcpListener {
        this: *mut c_void
    }

    extern "C" {
        pub fn sfTcpListener_create() -> *mut sfTcpListener;
        pub fn sfTcpListener_destroy(listener: *mut sfTcpListener) -> ();
        pub fn sfTcpListener_setBlocking(listener: *mut sfTcpListener, blocking: SfBool) -> ();
        pub fn sfTcpListener_isBlocking(listener: *mut sfTcpListener) -> SfBool;
        pub fn sfTcpListener_getLocalPort(listener: *mut sfTcpListener) -> u16;
        pub fn sfTcpListener_listen(listener: *mut sfTcpListener, port: u16) -> SocketStatus;
        pub fn sfTcpListener_accept(listener: *mut sfTcpListener, connected: *mut *mut sfTcpSocket) -> SocketStatus;
    }
}

pub mod tcp_socket {
    use libc::{c_void, size_t};

    use ffi::system::time::sfTime;
    use ffi::network::ip_address::sfIpAddress;
    use ffi::network::socket_status::SocketStatus;
    use ffi::network::packet::sfPacket;
    use ffi::sfml_types::SfBool;

    #[repr(C)]
    pub struct sfTcpSocket {
        this: *mut c_void
    }

    extern "C" {
        pub fn sfTcpSocket_create() -> *mut sfTcpSocket;
        pub fn sfTcpSocket_destroy(socket: *mut sfTcpSocket) -> ();
        pub fn sfTcpSocket_setBlocking(socket: *mut sfTcpSocket, blocking: SfBool) -> ();
        pub fn sfTcpSocket_isBlocking(socket: *mut sfTcpSocket) -> SfBool;
        pub fn sfTcpSocket_getLocalPort(socket: *mut sfTcpSocket) -> u16;
        pub fn sfTcpSocket_getRemoteAddress(socket: *mut sfTcpSocket) -> sfIpAddress;
        pub fn sfTcpSocket_getRemotePort(socket: *mut sfTcpSocket) -> u16;
        pub fn sfTcpSocket_connect(socket: *mut sfTcpSocket, host: sfIpAddress, port: u16,  timeout: sfTime) -> SocketStatus;
        pub fn sfTcpSocket_disconnect(socket: *mut sfTcpSocket) -> ();
        pub fn sfTcpSocket_send(socket: *mut sfTcpSocket, data: *const i8, size: size_t) -> SocketStatus;
        pub fn sfTcpSocket_receive(socket: *mut sfTcpSocket, data: *mut i8, maxSize: size_t, sizeReceived: *mut size_t) -> SocketStatus;
        pub fn sfTcpSocket_sendPacket(socket: *mut sfTcpSocket, packet: *mut sfPacket) -> SocketStatus;
        pub fn sfTcpSocket_receivePacket(socket: *mut sfTcpSocket, packet: *mut sfPacket) -> SocketStatus;
    }

}

pub mod udp_socket {
    use libc::{size_t, c_void};

    use ffi::network::socket_status::SocketStatus;
    use ffi::network::ip_address::sfIpAddress;
    use ffi::network::packet::sfPacket;
    use ffi::sfml_types::SfBool;

    #[repr(C)]
    pub struct sfUdpSocket {
        this: *mut c_void
    }

    extern "C" {
        pub fn sfUdpSocket_create() -> *mut sfUdpSocket;
        pub fn sfUdpSocket_destroy(socket: *mut sfUdpSocket) -> ();
        pub fn sfUdpSocket_setBlocking(socket: *mut sfUdpSocket, blocking: SfBool) -> ();
        pub fn sfUdpSocket_isBlocking(socket: *mut sfUdpSocket) -> SfBool;
        pub fn sfUdpSocket_getLocalPort(socket: *mut sfUdpSocket) -> u16;
        pub fn sfUdpSocket_bind(socket: *mut sfUdpSocket, port: u16) -> SocketStatus;
        pub fn sfUdpSocket_unbind(socket: *mut sfUdpSocket) -> ();
        pub fn sfUdpSocket_send(socket: *mut sfUdpSocket, data: *mut i8, size: size_t, address: sfIpAddress, port: u16) -> SocketStatus;
        pub fn sfUdpSocket_receive(socket: *mut sfUdpSocket, data: *mut i8, maxSize: size_t, sizeReceived: *mut size_t, address: *mut sfIpAddress, port: *mut u16) -> SocketStatus;
        pub fn sfUdpSocket_sendPacket(socket: *mut sfUdpSocket, packet: *mut sfPacket, address: sfIpAddress, port: u16) -> SocketStatus;
        pub fn sfUdpSocket_receivePacket(socket: *mut sfUdpSocket, packet: *mut sfPacket, address: *mut sfIpAddress, port: *mut u16) -> SocketStatus;
        pub fn sfUdpSocket_maxDatagramSize() -> u32;
    }
}

pub mod ftp {
    use libc::{c_void, c_char, size_t, c_int};

    use ffi::network::ip_address::sfIpAddress;
    use ffi::system::time::sfTime;
    use ffi::sfml_types::SfBool;

    pub type TransferMode = c_int;
    pub const FTPBINARY:   TransferMode = 0;
    pub const FTPASCII:    TransferMode = 1;
    pub const FTPEBCDIC:   TransferMode = 2;

    pub type Status = c_int;
    pub const RESTARTMARKERREPLY:          Status = 110;
    pub const SERVICEREADYSOON:            Status = 120;
    pub const DATACONNECTIONALREADYOPENED: Status = 125;
    pub const OPENINGDATACONNECTION:       Status = 150;

    pub const OK:                          Status = 200;
    pub const POINTLESSCOMMAND:            Status = 202;
    pub const SYSTEMSTATUS:                Status = 211;
    pub const DIRECTORYSTATUS:             Status = 212;
    pub const FILESTATUS:                  Status = 213;
    pub const HELPMESSAGE:                 Status = 214;
    pub const SYSTEMTYPE:                  Status = 215;
    pub const SERVICEREADY:                Status = 220;
    pub const CLOSINGCONNECTION:           Status = 221;
    pub const DATACONNECTIONOPENED:        Status = 225;
    pub const CLOSINGDATACONNECTION:       Status = 226;
    pub const ENTERINGPASSIVEMODE:         Status = 227;
    pub const LOGGEDIN:                    Status = 230;
    pub const FILEACTIONOK:                Status = 250;
    pub const DIRECTORYOK:                 Status = 257;

    pub const NEEDPASSWORD:                Status = 331;
    pub const NEEDACCOUNTTOLOGIN:          Status = 332;
    pub const NEEDINFORMATION:             Status = 350;

    pub const SERVICEUNAVAILABLE:          Status = 421;
    pub const DATACONNECTIONUNAVAILABLE:   Status = 425;
    pub const TRANSFERABORTED:             Status = 426;
    pub const FILEACTIONABORTED:           Status = 450;
    pub const LOCALERROR:                  Status = 451;
    pub const INSUFFICIENTSTORAGESPACE:    Status = 452;

    pub const COMMANDUNKNOWN:              Status = 500;
    pub const PARAMETERSUNKNOWN:           Status = 501;
    pub const COMMANDNOTIMPLEMENTED:       Status = 502;
    pub const BADCOMMANDSEQUENCE:          Status = 503;
    pub const PARAMETERNOTIMPLEMENTED:     Status = 504;
    pub const NOTLOGGEDIN:                 Status = 530;
    pub const NEEDACCOUNTTOSTORE:          Status = 532;
    pub const FILEUNAVAILABLE:             Status = 550;
    pub const PAGETYPEUNKNOWN:             Status = 551;
    pub const NOTENOUGHMEMORY:             Status = 552;
    pub const FILENAMENOTALLOWED:          Status = 553;

    pub const INVALIDRESPONSE:             Status = 1000;
    pub const CONNECTIONFAILED:            Status = 1001;
    pub const CONNECTIONCLOSED:            Status = 1002;
    pub const INVALIDFILE:                 Status = 1003;

    #[repr(C)]
    pub struct sfFtp {
        this: *mut c_void
    }

    #[repr(C)]
    pub struct sfFtpDirectoryResponse {
        this: *mut c_void
    }

    #[repr(C)]
    pub struct sfFtpListingResponse {
        this: *mut c_void
    }

    #[repr(C)]
    pub struct sfFtpResponse {
        this: *mut c_void
    }

    extern "C" {
        pub fn sfFtpListingResponse_destroy(ftpListingResponse: *mut sfFtpListingResponse) -> ();
        pub fn sfFtpListingResponse_isOk(ftpListingResponse: *mut sfFtpListingResponse) -> SfBool;
        pub fn sfFtpListingResponse_getStatus(ftpListingResponse: *mut sfFtpListingResponse) -> Status;
        pub fn sfFtpListingResponse_getMessage(ftpListingResponse: *mut sfFtpListingResponse) -> *const c_char;
        pub fn sfFtpListingResponse_getCount(ftpListingResponse: *mut sfFtpListingResponse) -> size_t;
        pub fn sfFtpListingResponse_getName(ftpListingResponse: *mut sfFtpListingResponse, index: size_t) -> *const c_char;
        pub fn sfFtpDirectoryResponse_destroy(ftpDirectoryResponse: *mut sfFtpDirectoryResponse) -> ();
        pub fn sfFtpDirectoryResponse_isOk(ftpDirectoryResponse: *mut sfFtpDirectoryResponse) -> SfBool;
        pub fn sfFtpDirectoryResponse_getStatus(ftpDirectoryResponse: *mut sfFtpDirectoryResponse) -> Status;
        pub fn sfFtpDirectoryResponse_getMessage(ftpDirectoryResponse: *mut sfFtpDirectoryResponse) -> *const c_char;
        pub fn sfFtpDirectoryResponse_getDirectory(ftpDirectoryResponse: *mut sfFtpDirectoryResponse) -> *const c_char;
        pub fn sfFtpResponse_destroy(ftpResponse: *mut sfFtpResponse) -> ();
        pub fn sfFtpResponse_isOk(ftpResponse: *mut sfFtpResponse) -> SfBool;
        pub fn sfFtpResponse_getStatus(ftpResponse: *mut sfFtpResponse) -> Status;
        pub fn sfFtpResponse_getMessage(ftpResponse: *mut sfFtpResponse) -> *const c_char;
        pub fn sfFtp_create() -> *mut sfFtp;
        pub fn sfFtp_destroy(ftp: *mut sfFtp) -> ();
        pub fn sfFtp_connect(ftp: *mut sfFtp, server: sfIpAddress, port: u16, timeout: sfTime) -> *mut sfFtpResponse;
        pub fn sfFtp_loginAnonymous(ftp: *mut sfFtp) -> *mut sfFtpResponse;
        pub fn sfFtp_login(ftp: *mut sfFtp, userName: *const c_char, password: *const c_char) -> *mut sfFtpResponse;
        pub fn sfFtp_disconnect(ftp: *mut sfFtp) -> *mut sfFtpResponse;
        pub fn sfFtp_keepAlive(ftp: *mut sfFtp) -> *mut sfFtpResponse;
        pub fn sfFtp_getWorkingDirectory(ftp: *mut sfFtp) -> *mut sfFtpDirectoryResponse;
        pub fn sfFtp_getDirectoryListing(ftp: *mut sfFtp, directory: *const c_char) -> *mut sfFtpListingResponse;
        pub fn sfFtp_changeDirectory(ftp: *mut sfFtp, directory: *const c_char) -> *mut sfFtpResponse;
        pub fn sfFtp_parentDirectory(ftp: *mut sfFtp) -> *mut sfFtpResponse;
        pub fn sfFtp_createDirectory(ftp: *mut sfFtp, name: *const c_char) -> *mut sfFtpResponse;
        pub fn sfFtp_deleteDirectory(ftp: *mut sfFtp, name: *const c_char) -> *mut sfFtpResponse;
        pub fn sfFtp_renameFile(ftp: *mut sfFtp, file: *const c_char, newName: *const c_char) -> *mut sfFtpResponse;
        pub fn sfFtp_deleteFile(ftp: *mut sfFtp, name: *const c_char) -> *mut sfFtpResponse;
        pub fn sfFtp_download(ftp: *mut sfFtp, distantFile: *const c_char, destPath: *const c_char, mode: TransferMode) -> *mut sfFtpResponse;
        pub fn sfFtp_upload(ftp: *mut sfFtp, localFile: *const c_char, destPath: *const c_char, mode: TransferMode) -> *mut sfFtpResponse;
    }
}

pub mod http {
    use libc::{c_char, c_void,};

    use ffi::system::time::sfTime;

    pub type Method = i32;
    pub const GET: Method = 0;
    pub const POST: Method = 1;
    pub const HEAD: Method = 2;

    pub type Status = i32;
    pub const OK:                  Status = 200;
    pub const CREATED:             Status = 201;
    pub const ACCEPTED:            Status = 202;
    pub const NOCONTENT:           Status = 204;
    pub const RESETCONTENT:        Status = 205;
    pub const PARTIALCONTENT:      Status = 206;

    pub const MULTIPLECHOICES:     Status = 300;
    pub const MOVEDPERMANENTLY:    Status = 301;
    pub const MOVEDTEMPORARILY:    Status = 302;
    pub const NOTMODIFIED:         Status = 304;


    pub const BADREQUEST:          Status = 400;
    pub const UNAUTHORIZED:        Status = 401;
    pub const FORBIDDEN:           Status = 403;
    pub const NOTFOUND:            Status = 404;
    pub const RANGENOTSATISFIABLE: Status = 407;

    pub const INTERNALSERVERERROR: Status = 500;
    pub const NOTIMPLEMENTED:      Status = 501;
    pub const BADGATEWAY:          Status = 502;
    pub const SERVICENOTAVAILABLE: Status = 503;
    pub const GATEWAYTIMEOUT:      Status = 504;
    pub const VERSIONNOTSUPPORTED: Status = 505;

    pub const INVALIDRESPONSE:     Status = 1000;
    pub const CONNECTIONFAILED:    Status = 1001;

    #[repr(C)]
    pub struct sfHttpRequest {
        this: *mut c_void
    }

    #[repr(C)]
    pub struct sfHttpResponse {
        this: *mut c_void
    }

    #[repr(C)]
    pub struct sfHttp {
        this: *mut c_void
    }

    extern "C" {
        pub fn sfHttpRequest_create() -> *mut sfHttpRequest;
        pub fn sfHttpRequest_destroy(httpRequest: *mut sfHttpRequest) -> ();
        pub fn sfHttpRequest_setField(httpRequest: *mut sfHttpRequest, field: *const c_char, value: *const c_char) -> ();
        pub fn sfHttpRequest_setMethod(httpRequest: *mut sfHttpRequest, method: Method) -> ();
        pub fn sfHttpRequest_setUri(httpRequest: *mut sfHttpRequest, uri: *const c_char) -> ();
        pub fn sfHttpRequest_setHttpVersion(httpRequest: *mut sfHttpRequest, major: u32, minor: u32) -> ();
        pub fn sfHttpRequest_setBody(httpRequest: *mut sfHttpRequest, body: *const c_char) -> ();
        pub fn sfHttpResponse_destroy(httpResponse: *mut sfHttpResponse) -> ();
        pub fn sfHttpResponse_getField(httpResponse: *mut sfHttpResponse, field: *const c_char) -> *const c_char;
        pub fn sfHttpResponse_getStatus(httpResponse: *mut sfHttpResponse) -> Status;
        pub fn sfHttpResponse_getMajorVersion(httpResponse: *mut sfHttpResponse) -> u32;
        pub fn sfHttpResponse_getMinorVersion(httpResponse: *mut sfHttpResponse) -> u32;
        pub fn sfHttpResponse_getBody(httpResponse: *mut sfHttpResponse) -> *const c_char;
        pub fn sfHttp_create() -> *mut sfHttp;
        pub fn sfHttp_destroy(http: *mut sfHttp) -> ();
        pub fn sfHttp_setHost(http: *mut sfHttp, host: *const c_char, port: u16) -> ();
        pub fn sfHttp_sendRequest(http: *mut sfHttp, httpRequest: *mut sfHttpRequest, timeout: sfTime) -> *mut sfHttpResponse;
    }
}
