#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate csfml_system_sys;
use csfml_system_sys::*;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfIpAddress {
    pub address: [::std::os::raw::c_schar; 16usize],
}
#[test]
fn bindgen_test_layout_sfIpAddress() {
    assert_eq!(::std::mem::size_of::<sfIpAddress>() , 16usize , concat ! (
               "Size of: " , stringify ! ( sfIpAddress ) ));
    assert_eq! (::std::mem::align_of::<sfIpAddress>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( sfIpAddress ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfIpAddress ) ) . address as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfIpAddress ) , "::" ,
                stringify ! ( address ) ));
}
impl Clone for sfIpAddress {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "sfIpAddress_None"]
    pub static sfIpAddress_None: sfIpAddress;
}
extern "C" {
    #[link_name = "sfIpAddress_Any"]
    pub static sfIpAddress_Any: sfIpAddress;
}
extern "C" {
    #[link_name = "sfIpAddress_LocalHost"]
    pub static sfIpAddress_LocalHost: sfIpAddress;
}
extern "C" {
    #[link_name = "sfIpAddress_Broadcast"]
    pub static sfIpAddress_Broadcast: sfIpAddress;
}
extern "C" {
    pub fn sfIpAddress_fromString(address: *const ::std::os::raw::c_schar)
     -> sfIpAddress;
}
extern "C" {
    pub fn sfIpAddress_fromBytes(byte0: sfUint8, byte1: sfUint8,
                                 byte2: sfUint8, byte3: sfUint8)
     -> sfIpAddress;
}
extern "C" {
    pub fn sfIpAddress_fromInteger(address: sfUint32) -> sfIpAddress;
}
extern "C" {
    pub fn sfIpAddress_toString(address: sfIpAddress,
                                string: *mut ::std::os::raw::c_schar);
}
extern "C" {
    pub fn sfIpAddress_toInteger(address: sfIpAddress) -> sfUint32;
}
extern "C" {
    pub fn sfIpAddress_getLocalAddress() -> sfIpAddress;
}
extern "C" {
    pub fn sfIpAddress_getPublicAddress(timeout: sfTime) -> sfIpAddress;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfFtpDirectoryResponse([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfFtpListingResponse([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfFtpResponse([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfFtp([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfHttpRequest([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfHttpResponse([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfHttp([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfPacket([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfSocketSelector([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfTcpListener([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfTcpSocket([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfUdpSocket([u8; 0]);
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: f64,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(::std::mem::size_of::<max_align_t>() , 32usize , concat ! (
               "Size of: " , stringify ! ( max_align_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const max_align_t ) ) .
                __clang_max_align_nonce1 as * const _ as usize } , 0usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( max_align_t ) , "::" ,
                stringify ! ( __clang_max_align_nonce1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const max_align_t ) ) .
                __clang_max_align_nonce2 as * const _ as usize } , 16usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( max_align_t ) , "::" ,
                stringify ! ( __clang_max_align_nonce2 ) ));
}
impl Clone for max_align_t {
    fn clone(&self) -> Self { *self }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfFtpTransferMode {
    sfFtpBinary = 0,
    sfFtpAscii = 1,
    sfFtpEbcdic = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfFtpStatus {
    sfFtpRestartMarkerReply = 110,
    sfFtpServiceReadySoon = 120,
    sfFtpDataConnectionAlreadyOpened = 125,
    sfFtpOpeningDataConnection = 150,
    sfFtpOk = 200,
    sfFtpPointlessCommand = 202,
    sfFtpSystemStatus = 211,
    sfFtpDirectoryStatus = 212,
    sfFtpFileStatus = 213,
    sfFtpHelpMessage = 214,
    sfFtpSystemType = 215,
    sfFtpServiceReady = 220,
    sfFtpClosingConnection = 221,
    sfFtpDataConnectionOpened = 225,
    sfFtpClosingDataConnection = 226,
    sfFtpEnteringPassiveMode = 227,
    sfFtpLoggedIn = 230,
    sfFtpFileActionOk = 250,
    sfFtpDirectoryOk = 257,
    sfFtpNeedPassword = 331,
    sfFtpNeedAccountToLogIn = 332,
    sfFtpNeedInformation = 350,
    sfFtpServiceUnavailable = 421,
    sfFtpDataConnectionUnavailable = 425,
    sfFtpTransferAborted = 426,
    sfFtpFileActionAborted = 450,
    sfFtpLocalError = 451,
    sfFtpInsufficientStorageSpace = 452,
    sfFtpCommandUnknown = 500,
    sfFtpParametersUnknown = 501,
    sfFtpCommandNotImplemented = 502,
    sfFtpBadCommandSequence = 503,
    sfFtpParameterNotImplemented = 504,
    sfFtpNotLoggedIn = 530,
    sfFtpNeedAccountToStore = 532,
    sfFtpFileUnavailable = 550,
    sfFtpPageTypeUnknown = 551,
    sfFtpNotEnoughMemory = 552,
    sfFtpFilenameNotAllowed = 553,
    sfFtpInvalidResponse = 1000,
    sfFtpConnectionFailed = 1001,
    sfFtpConnectionClosed = 1002,
    sfFtpInvalidFile = 1003,
}
extern "C" {
    pub fn sfFtpListingResponse_destroy(ftpListingResponse:
                                            *mut sfFtpListingResponse);
}
extern "C" {
    pub fn sfFtpListingResponse_isOk(ftpListingResponse:
                                         *const sfFtpListingResponse)
     -> sfBool;
}
extern "C" {
    pub fn sfFtpListingResponse_getStatus(ftpListingResponse:
                                              *const sfFtpListingResponse)
     -> sfFtpStatus;
}
extern "C" {
    pub fn sfFtpListingResponse_getMessage(ftpListingResponse:
                                               *const sfFtpListingResponse)
     -> *const ::std::os::raw::c_schar;
}
extern "C" {
    pub fn sfFtpListingResponse_getCount(ftpListingResponse:
                                             *const sfFtpListingResponse)
     -> usize;
}
extern "C" {
    pub fn sfFtpListingResponse_getName(ftpListingResponse:
                                            *const sfFtpListingResponse,
                                        index: usize)
     -> *const ::std::os::raw::c_schar;
}
extern "C" {
    pub fn sfFtpDirectoryResponse_destroy(ftpDirectoryResponse:
                                              *mut sfFtpDirectoryResponse);
}
extern "C" {
    pub fn sfFtpDirectoryResponse_isOk(ftpDirectoryResponse:
                                           *const sfFtpDirectoryResponse)
     -> sfBool;
}
extern "C" {
    pub fn sfFtpDirectoryResponse_getStatus(ftpDirectoryResponse:
                                                *const sfFtpDirectoryResponse)
     -> sfFtpStatus;
}
extern "C" {
    pub fn sfFtpDirectoryResponse_getMessage(ftpDirectoryResponse:
                                                 *const sfFtpDirectoryResponse)
     -> *const ::std::os::raw::c_schar;
}
extern "C" {
    pub fn sfFtpDirectoryResponse_getDirectory(ftpDirectoryResponse:
                                                   *const sfFtpDirectoryResponse)
     -> *const ::std::os::raw::c_schar;
}
extern "C" {
    pub fn sfFtpResponse_destroy(ftpResponse: *mut sfFtpResponse);
}
extern "C" {
    pub fn sfFtpResponse_isOk(ftpResponse: *const sfFtpResponse) -> sfBool;
}
extern "C" {
    pub fn sfFtpResponse_getStatus(ftpResponse: *const sfFtpResponse)
     -> sfFtpStatus;
}
extern "C" {
    pub fn sfFtpResponse_getMessage(ftpResponse: *const sfFtpResponse)
     -> *const ::std::os::raw::c_schar;
}
extern "C" {
    pub fn sfFtp_create() -> *mut sfFtp;
}
extern "C" {
    pub fn sfFtp_destroy(ftp: *mut sfFtp);
}
extern "C" {
    pub fn sfFtp_connect(ftp: *mut sfFtp, server: sfIpAddress,
                         port: ::std::os::raw::c_ushort, timeout: sfTime)
     -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_loginAnonymous(ftp: *mut sfFtp) -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_login(ftp: *mut sfFtp, name: *const ::std::os::raw::c_schar,
                       password: *const ::std::os::raw::c_schar)
     -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_disconnect(ftp: *mut sfFtp) -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_keepAlive(ftp: *mut sfFtp) -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_getWorkingDirectory(ftp: *mut sfFtp)
     -> *mut sfFtpDirectoryResponse;
}
extern "C" {
    pub fn sfFtp_getDirectoryListing(ftp: *mut sfFtp,
                                     directory:
                                         *const ::std::os::raw::c_schar)
     -> *mut sfFtpListingResponse;
}
extern "C" {
    pub fn sfFtp_changeDirectory(ftp: *mut sfFtp,
                                 directory: *const ::std::os::raw::c_schar)
     -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_parentDirectory(ftp: *mut sfFtp) -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_createDirectory(ftp: *mut sfFtp,
                                 name: *const ::std::os::raw::c_schar)
     -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_deleteDirectory(ftp: *mut sfFtp,
                                 name: *const ::std::os::raw::c_schar)
     -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_renameFile(ftp: *mut sfFtp,
                            file: *const ::std::os::raw::c_schar,
                            newName: *const ::std::os::raw::c_schar)
     -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_deleteFile(ftp: *mut sfFtp,
                            name: *const ::std::os::raw::c_schar)
     -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_download(ftp: *mut sfFtp,
                          remoteFile: *const ::std::os::raw::c_schar,
                          localPath: *const ::std::os::raw::c_schar,
                          mode: sfFtpTransferMode) -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_upload(ftp: *mut sfFtp,
                        localFile: *const ::std::os::raw::c_schar,
                        remotePath: *const ::std::os::raw::c_schar,
                        mode: sfFtpTransferMode) -> *mut sfFtpResponse;
}
extern "C" {
    pub fn sfFtp_sendCommand(ftp: *mut sfFtp,
                             command: *const ::std::os::raw::c_schar,
                             parameter: *const ::std::os::raw::c_schar)
     -> *mut sfFtpResponse;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfHttpMethod {
    sfHttpGet = 0,
    sfHttpPost = 1,
    sfHttpHead = 2,
    sfHttpPut = 3,
    sfHttpDelete = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfHttpStatus {
    sfHttpOk = 200,
    sfHttpCreated = 201,
    sfHttpAccepted = 202,
    sfHttpNoContent = 204,
    sfHttpResetContent = 205,
    sfHttpPartialContent = 206,
    sfHttpMultipleChoices = 300,
    sfHttpMovedPermanently = 301,
    sfHttpMovedTemporarily = 302,
    sfHttpNotModified = 304,
    sfHttpBadRequest = 400,
    sfHttpUnauthorized = 401,
    sfHttpForbidden = 403,
    sfHttpNotFound = 404,
    sfHttpRangeNotSatisfiable = 407,
    sfHttpInternalServerError = 500,
    sfHttpNotImplemented = 501,
    sfHttpBadGateway = 502,
    sfHttpServiceNotAvailable = 503,
    sfHttpGatewayTimeout = 504,
    sfHttpVersionNotSupported = 505,
    sfHttpInvalidResponse = 1000,
    sfHttpConnectionFailed = 1001,
}
extern "C" {
    pub fn sfHttpRequest_create() -> *mut sfHttpRequest;
}
extern "C" {
    pub fn sfHttpRequest_destroy(httpRequest: *mut sfHttpRequest);
}
extern "C" {
    pub fn sfHttpRequest_setField(httpRequest: *mut sfHttpRequest,
                                  field: *const ::std::os::raw::c_schar,
                                  value: *const ::std::os::raw::c_schar);
}
extern "C" {
    pub fn sfHttpRequest_setMethod(httpRequest: *mut sfHttpRequest,
                                   method: sfHttpMethod);
}
extern "C" {
    pub fn sfHttpRequest_setUri(httpRequest: *mut sfHttpRequest,
                                uri: *const ::std::os::raw::c_schar);
}
extern "C" {
    pub fn sfHttpRequest_setHttpVersion(httpRequest: *mut sfHttpRequest,
                                        major: ::std::os::raw::c_uint,
                                        minor: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn sfHttpRequest_setBody(httpRequest: *mut sfHttpRequest,
                                 body: *const ::std::os::raw::c_schar);
}
extern "C" {
    pub fn sfHttpResponse_destroy(httpResponse: *mut sfHttpResponse);
}
extern "C" {
    pub fn sfHttpResponse_getField(httpResponse: *const sfHttpResponse,
                                   field: *const ::std::os::raw::c_schar)
     -> *const ::std::os::raw::c_schar;
}
extern "C" {
    pub fn sfHttpResponse_getStatus(httpResponse: *const sfHttpResponse)
     -> sfHttpStatus;
}
extern "C" {
    pub fn sfHttpResponse_getMajorVersion(httpResponse: *const sfHttpResponse)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfHttpResponse_getMinorVersion(httpResponse: *const sfHttpResponse)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfHttpResponse_getBody(httpResponse: *const sfHttpResponse)
     -> *const ::std::os::raw::c_schar;
}
extern "C" {
    pub fn sfHttp_create() -> *mut sfHttp;
}
extern "C" {
    pub fn sfHttp_destroy(http: *mut sfHttp);
}
extern "C" {
    pub fn sfHttp_setHost(http: *mut sfHttp,
                          host: *const ::std::os::raw::c_schar,
                          port: ::std::os::raw::c_ushort);
}
extern "C" {
    pub fn sfHttp_sendRequest(http: *mut sfHttp,
                              request: *const sfHttpRequest, timeout: sfTime)
     -> *mut sfHttpResponse;
}
extern "C" {
    pub fn sfPacket_create() -> *mut sfPacket;
}
extern "C" {
    pub fn sfPacket_copy(packet: *const sfPacket) -> *mut sfPacket;
}
extern "C" {
    pub fn sfPacket_destroy(packet: *mut sfPacket);
}
extern "C" {
    pub fn sfPacket_append(packet: *mut sfPacket,
                           data: *const ::std::os::raw::c_void,
                           sizeInBytes: usize);
}
extern "C" {
    pub fn sfPacket_clear(packet: *mut sfPacket);
}
extern "C" {
    pub fn sfPacket_getData(packet: *const sfPacket)
     -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sfPacket_getDataSize(packet: *const sfPacket) -> usize;
}
extern "C" {
    pub fn sfPacket_endOfPacket(packet: *const sfPacket) -> sfBool;
}
extern "C" {
    pub fn sfPacket_canRead(packet: *const sfPacket) -> sfBool;
}
extern "C" {
    pub fn sfPacket_readBool(packet: *mut sfPacket) -> sfBool;
}
extern "C" {
    pub fn sfPacket_readInt8(packet: *mut sfPacket) -> sfInt8;
}
extern "C" {
    pub fn sfPacket_readUint8(packet: *mut sfPacket) -> sfUint8;
}
extern "C" {
    pub fn sfPacket_readInt16(packet: *mut sfPacket) -> sfInt16;
}
extern "C" {
    pub fn sfPacket_readUint16(packet: *mut sfPacket) -> sfUint16;
}
extern "C" {
    pub fn sfPacket_readInt32(packet: *mut sfPacket) -> sfInt32;
}
extern "C" {
    pub fn sfPacket_readUint32(packet: *mut sfPacket) -> sfUint32;
}
extern "C" {
    pub fn sfPacket_readFloat(packet: *mut sfPacket) -> f32;
}
extern "C" {
    pub fn sfPacket_readDouble(packet: *mut sfPacket) -> f64;
}
extern "C" {
    pub fn sfPacket_readString(packet: *mut sfPacket,
                               string: *mut ::std::os::raw::c_schar);
}
extern "C" {
    pub fn sfPacket_readWideString(packet: *mut sfPacket,
                                   string: *mut wchar_t);
}
extern "C" {
    pub fn sfPacket_writeBool(packet: *mut sfPacket, arg1: sfBool);
}
extern "C" {
    pub fn sfPacket_writeInt8(packet: *mut sfPacket, arg1: sfInt8);
}
extern "C" {
    pub fn sfPacket_writeUint8(packet: *mut sfPacket, arg1: sfUint8);
}
extern "C" {
    pub fn sfPacket_writeInt16(packet: *mut sfPacket, arg1: sfInt16);
}
extern "C" {
    pub fn sfPacket_writeUint16(packet: *mut sfPacket, arg1: sfUint16);
}
extern "C" {
    pub fn sfPacket_writeInt32(packet: *mut sfPacket, arg1: sfInt32);
}
extern "C" {
    pub fn sfPacket_writeUint32(packet: *mut sfPacket, arg1: sfUint32);
}
extern "C" {
    pub fn sfPacket_writeFloat(packet: *mut sfPacket, arg1: f32);
}
extern "C" {
    pub fn sfPacket_writeDouble(packet: *mut sfPacket, arg1: f64);
}
extern "C" {
    pub fn sfPacket_writeString(packet: *mut sfPacket,
                                string: *const ::std::os::raw::c_schar);
}
extern "C" {
    pub fn sfPacket_writeWideString(packet: *mut sfPacket,
                                    string: *const wchar_t);
}
extern "C" {
    pub fn sfSocketSelector_create() -> *mut sfSocketSelector;
}
extern "C" {
    pub fn sfSocketSelector_copy(selector: *const sfSocketSelector)
     -> *mut sfSocketSelector;
}
extern "C" {
    pub fn sfSocketSelector_destroy(selector: *mut sfSocketSelector);
}
extern "C" {
    pub fn sfSocketSelector_addTcpListener(selector: *mut sfSocketSelector,
                                           socket: *mut sfTcpListener);
}
extern "C" {
    pub fn sfSocketSelector_addTcpSocket(selector: *mut sfSocketSelector,
                                         socket: *mut sfTcpSocket);
}
extern "C" {
    pub fn sfSocketSelector_addUdpSocket(selector: *mut sfSocketSelector,
                                         socket: *mut sfUdpSocket);
}
extern "C" {
    pub fn sfSocketSelector_removeTcpListener(selector: *mut sfSocketSelector,
                                              socket: *mut sfTcpListener);
}
extern "C" {
    pub fn sfSocketSelector_removeTcpSocket(selector: *mut sfSocketSelector,
                                            socket: *mut sfTcpSocket);
}
extern "C" {
    pub fn sfSocketSelector_removeUdpSocket(selector: *mut sfSocketSelector,
                                            socket: *mut sfUdpSocket);
}
extern "C" {
    pub fn sfSocketSelector_clear(selector: *mut sfSocketSelector);
}
extern "C" {
    pub fn sfSocketSelector_wait(selector: *mut sfSocketSelector,
                                 timeout: sfTime) -> sfBool;
}
extern "C" {
    pub fn sfSocketSelector_isTcpListenerReady(selector:
                                                   *const sfSocketSelector,
                                               socket: *mut sfTcpListener)
     -> sfBool;
}
extern "C" {
    pub fn sfSocketSelector_isTcpSocketReady(selector:
                                                 *const sfSocketSelector,
                                             socket: *mut sfTcpSocket)
     -> sfBool;
}
extern "C" {
    pub fn sfSocketSelector_isUdpSocketReady(selector:
                                                 *const sfSocketSelector,
                                             socket: *mut sfUdpSocket)
     -> sfBool;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfSocketStatus {
    sfSocketDone = 0,
    sfSocketNotReady = 1,
    sfSocketPartial = 2,
    sfSocketDisconnected = 3,
    sfSocketError = 4,
}
extern "C" {
    pub fn sfTcpListener_create() -> *mut sfTcpListener;
}
extern "C" {
    pub fn sfTcpListener_destroy(listener: *mut sfTcpListener);
}
extern "C" {
    pub fn sfTcpListener_setBlocking(listener: *mut sfTcpListener,
                                     blocking: sfBool);
}
extern "C" {
    pub fn sfTcpListener_isBlocking(listener: *const sfTcpListener) -> sfBool;
}
extern "C" {
    pub fn sfTcpListener_getLocalPort(listener: *const sfTcpListener)
     -> ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn sfTcpListener_listen(listener: *mut sfTcpListener,
                                port: ::std::os::raw::c_ushort,
                                address: sfIpAddress) -> sfSocketStatus;
}
extern "C" {
    pub fn sfTcpListener_accept(listener: *mut sfTcpListener,
                                connected: *mut *mut sfTcpSocket)
     -> sfSocketStatus;
}
extern "C" {
    pub fn sfTcpSocket_create() -> *mut sfTcpSocket;
}
extern "C" {
    pub fn sfTcpSocket_destroy(socket: *mut sfTcpSocket);
}
extern "C" {
    pub fn sfTcpSocket_setBlocking(socket: *mut sfTcpSocket,
                                   blocking: sfBool);
}
extern "C" {
    pub fn sfTcpSocket_isBlocking(socket: *const sfTcpSocket) -> sfBool;
}
extern "C" {
    pub fn sfTcpSocket_getLocalPort(socket: *const sfTcpSocket)
     -> ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn sfTcpSocket_getRemoteAddress(socket: *const sfTcpSocket)
     -> sfIpAddress;
}
extern "C" {
    pub fn sfTcpSocket_getRemotePort(socket: *const sfTcpSocket)
     -> ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn sfTcpSocket_connect(socket: *mut sfTcpSocket,
                               remoteAddress: sfIpAddress,
                               remotePort: ::std::os::raw::c_ushort,
                               timeout: sfTime) -> sfSocketStatus;
}
extern "C" {
    pub fn sfTcpSocket_disconnect(socket: *mut sfTcpSocket);
}
extern "C" {
    pub fn sfTcpSocket_send(socket: *mut sfTcpSocket,
                            data: *const ::std::os::raw::c_void, size: usize)
     -> sfSocketStatus;
}
extern "C" {
    pub fn sfTcpSocket_sendPartial(socket: *mut sfTcpSocket,
                                   data: *const ::std::os::raw::c_void,
                                   size: usize, sent: *mut usize)
     -> sfSocketStatus;
}
extern "C" {
    pub fn sfTcpSocket_receive(socket: *mut sfTcpSocket,
                               data: *mut ::std::os::raw::c_void, size: usize,
                               received: *mut usize) -> sfSocketStatus;
}
extern "C" {
    pub fn sfTcpSocket_sendPacket(socket: *mut sfTcpSocket,
                                  packet: *mut sfPacket) -> sfSocketStatus;
}
extern "C" {
    pub fn sfTcpSocket_receivePacket(socket: *mut sfTcpSocket,
                                     packet: *mut sfPacket) -> sfSocketStatus;
}
extern "C" {
    pub fn sfUdpSocket_create() -> *mut sfUdpSocket;
}
extern "C" {
    pub fn sfUdpSocket_destroy(socket: *mut sfUdpSocket);
}
extern "C" {
    pub fn sfUdpSocket_setBlocking(socket: *mut sfUdpSocket,
                                   blocking: sfBool);
}
extern "C" {
    pub fn sfUdpSocket_isBlocking(socket: *const sfUdpSocket) -> sfBool;
}
extern "C" {
    pub fn sfUdpSocket_getLocalPort(socket: *const sfUdpSocket)
     -> ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn sfUdpSocket_bind(socket: *mut sfUdpSocket,
                            port: ::std::os::raw::c_ushort,
                            address: sfIpAddress) -> sfSocketStatus;
}
extern "C" {
    pub fn sfUdpSocket_unbind(socket: *mut sfUdpSocket);
}
extern "C" {
    pub fn sfUdpSocket_send(socket: *mut sfUdpSocket,
                            data: *const ::std::os::raw::c_void, size: usize,
                            remoteAddress: sfIpAddress,
                            remotePort: ::std::os::raw::c_ushort)
     -> sfSocketStatus;
}
extern "C" {
    pub fn sfUdpSocket_receive(socket: *mut sfUdpSocket,
                               data: *mut ::std::os::raw::c_void, size: usize,
                               received: *mut usize,
                               remoteAddress: *mut sfIpAddress,
                               remotePort: *mut ::std::os::raw::c_ushort)
     -> sfSocketStatus;
}
extern "C" {
    pub fn sfUdpSocket_sendPacket(socket: *mut sfUdpSocket,
                                  packet: *mut sfPacket,
                                  remoteAddress: sfIpAddress,
                                  remotePort: ::std::os::raw::c_ushort)
     -> sfSocketStatus;
}
extern "C" {
    pub fn sfUdpSocket_receivePacket(socket: *mut sfUdpSocket,
                                     packet: *mut sfPacket,
                                     remoteAddress: *mut sfIpAddress,
                                     remotePort:
                                         *mut ::std::os::raw::c_ushort)
     -> sfSocketStatus;
}
extern "C" {
    pub fn sfUdpSocket_maxDatagramSize() -> ::std::os::raw::c_uint;
}
