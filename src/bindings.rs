// Pre-generated bindings for CuFile
// These are fallback bindings when bindgen is not available

use std::os::raw::{c_char, c_int, c_void};

// CuFile version
pub const CUFILE_VERSION: u32 = 1000;

/// The cufile is successful.
pub const CU_FILE_SUCCESS: i32 = 0;

/// The nvidia-fs driver is not loaded.
pub const CU_FILE_DRIVER_NOT_INITIALIZED: i32 = 5001;

/// An invalid property.
pub const CU_FILE_DRIVER_INVALID_PROPS: i32 = 5002;

/// A property range error.
pub const CU_FILE_DRIVER_UNSUPPORTED_LIMIT: i32 = 5003;

/// An nvidia-fs driver version mismatch.
pub const CU_FILE_DRIVER_VERSION_MISMATCH: i32 = 5004;

/// An nvidia-fs driver version read error.
pub const CU_FILE_DRIVER_VERSION_READ_ERROR: i32 = 5005;

/// Driver shutdown in progress.
pub const CU_FILE_DRIVER_CLOSING: i32 = 5006;

/// GDS is not supported on the current platform.
pub const CU_FILE_PLATFORM_NOT_SUPPORTED: i32 = 500;

/// GDS is not supported on the current file.
pub const CU_FILE_IO_NOT_SUPPORTED: i32 = 5008;

/// GDS is not supported on the current GPU.
pub const CU_FILE_DEVICE_NOT_SUPPORTED: i32 = 5009;

/// An nvidia-fs driver ioctl error.
pub const CU_FILE_NVFS_DRIVER_ERROR: i32 = 5010;

/// A CUDA Driver API error.
///
/// This error indicates a CUDA driver-api error.
/// If this is set, a CUDA-specific error code is set in the `cu_err` field for `cuFileError`.
pub const CU_FILE_CUDA_DRIVER_ERROR: i32 = 5011;

/// An invalid device pointer.
pub const CU_FILE_CUDA_POINTER_INVALID: i32 = 5012;

/// An invalid pointer memory type.
pub const CU_FILE_CUDA_MEMORY_TYPE_INVALID: i32 = 5013;

/// The pointer range exceeds the allocated address range.
pub const CU_FILE_CUDA_POINTER_RANGE_ERROR: i32 = 5014;

/// A CUDA context mismatch.
pub const CU_FILE_CUDA_CONTEXT_MISMATCH: i32 = 5015;

/// Access beyond the maximum pinned memory size.
pub const CU_FILE_INVALID_MAPPING_SIZE: i32 = 5016;

/// Access beyond the mapped size.
pub const CU_FILE_INVALID_MAPPING_RANGE: i32 = 5017;

/// An unsupported file type.
pub const CU_FILE_INVALID_FILE_TYPE: i32 = 5018;

/// Unsupported file open flags.
pub const CU_FILE_INVALID_FILE_OPEN_FLAG: i32 = 5019;

/// The fd direct IO is not set.
pub const CU_FILE_DIO_NOT_SET: i32 = 5020;

/// Invalid API arguments.
pub const CU_FILE_INVALID_VALUE: i32 = 5022;

/// Device pointer is already registered.
pub const CU_FILE_MEMORY_ALREADY_REGISTERED: i32 = 5023;

/// A device pointer lookup failure has occurred.
pub const CU_FILE_MEMORY_NOT_REGISTERED: i32 = 5024;

/// A driver or file access error.
pub const CU_FILE_PERMISSION_DENIED: i32 = 5025;

/// The driver is already open.
pub const CU_FILE_DRIVER_ALREADY_OPEN: i32 = 5026;

/// The file descriptor is not registered.
pub const CU_FILE_HANDLE_NOT_REGISTERED: i32 = 5027;

/// The file descriptor is already registered.
pub const CU_FILE_HANDLE_ALREADY_REGISTERED: i32 = 5028;

/// The GPU device cannot be not found.
pub const CU_FILE_DEVICE_NOT_FOUND: i32 = 5029;

/// An internal error has occurred. Refer to cufile.log for more details.
pub const CU_FILE_INTERNAL_ERROR: i32 = 5030;

/// Failed to obtain a new file descriptor.
pub const CU_FILE_GETNEWFD_FAILED: i32 = 5031;

/// An NVFS driver initialization error has occurred.
pub const CU_FILE_NVFS_SETUP_ERROR: i32 = 5033;

/// GDS is disabled by config on the current file.
pub const CU_FILE_IO_DISABLED: i32 = 5034;

/// Failed to submit a batch operation.
pub const CU_FILE_BATCH_SUBMIT_FAILED: i32 = 5035;

/// Failed to allocate pinned GPU memory.
pub const CU_FILE_GPU_MEMORY_PINNING_FAILED: i32 = 5036;

/// Queue full for batch operation.
pub const CU_FILE_BATCH_FULL: i32 = 5037;

/// cuFile stream operation is not supported.
pub const CU_FILE_ASYNC_NOT_SUPPORTED: i32 = 5038;

// Handle types
pub type CUfileHandle_t = *mut c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CUfileFileHandleType {
    /// linux based fd
    CU_FILE_HANDLE_TYPE_OPAQUE_FD = 1,
    /// windows based handle
    CU_FILE_HANDLE_TYPE_OPAQUE_WIN32 = 2,
    /// userspace based FS
    CU_FILE_HANDLE_TYPE_USERSPACE_FS = 3,
}

#[repr(C)]
pub union CUfileDescrHandle {
    pub fd: c_int, // Linux
    pub handle: *mut c_void, // Windows
}

#[repr(C)]
pub struct CUfileDescr_t {
    pub type_: CUfileFileHandleType,
    pub handle: CUfileDescrHandle,
    pub fs_ops: *const c_void,
}


// Error type
pub type CUfileError_t = c_int;

extern "C" {
    // Driver management
    pub fn cuFileDriverOpen() -> CUfileError_t;
    pub fn cuFileDriverClose() -> CUfileError_t;

    // Handle management
    pub fn cuFileHandleRegister(
        fh: *mut CUfileHandle_t,
        descr: *mut CUfileDescr_t,
    ) -> CUfileError_t;
    pub fn cuFileHandleDeregister(fh: CUfileHandle_t) -> CUfileError_t;

    // Buffer management
    pub fn cuFileBufRegister(devPtr_base: *const c_void, size: usize, flags: c_int) -> CUfileError_t;
    pub fn cuFileBufDeregister(devPtr_base: *const c_void) -> CUfileError_t;

    // Synchronous I/O
    pub fn cuFileRead(
        fh: CUfileHandle_t,
        bufPtr_base: *mut c_void,
        size: usize,
        file_offset: i64,
        bufPtr_offset: i64,
    ) -> isize;

    pub fn cuFileWrite(
        fh: CUfileHandle_t,
        bufPtr_base: *const c_void,
        size: usize,
        file_offset: i64,
        bufPtr_offset: i64,
    ) -> isize;

    // Error handling
    pub fn cuFileGetErrorString(error: CUfileError_t) -> *const c_char;
}