//! The Able File System will be the system wide file system

#[repr(transparent)]
pub struct ProcessId(u32);
/// https://man7.org/linux/man-pages/man2/open.2.html
#[repr(u32)]
pub enum OpenOptionsMask {
	/// Append to the file's end, don't overwrite
	Append         = 0x00000001,
	Async          = 0x00000002,
	CloseOnExecute = 0x00000004,
	/// Create the file, or open if exists UNLESS
	/// exclude
	Create         = 0x00000008,
	/// Try to minimize cache effects of the I/O to and from this
	/// file.  In general this will degrade performance, but it is
	/// useful in special situations, such as when applications do
	/// their own caching.  File I/O is done directly to/from
	/// user-space buffers.  The O_DIRECT flag on its own makes an
	/// effort to transfer data synchronously, but does not give
	/// the guarantees of the O_SYNC flag that data and necessary
	/// metadata are transferred.  To guarantee synchronous I/O,
	/// O_SYNC must be used in addition to O_DIRECT.  See NOTES
	/// below for further discussion.
	Direct         = 0x00000010,
	/// If pathname is not a directory, cause the open to fail.
	/// This flag was added in kernel version 2.1.126, to avoid
	/// denial-of-service problems if opendir(3) is called on a
	/// FIFO or tape device.
	Directory      = 0x00000020,
	/// Ensure that file is created, else return
	/// with an error
	Exlusive       = 0x00000040,
	/// Do not modify last access time
	NoAccessTime   = 0x00000080,
	/// Do not follow sym links
	NoFollow       = 0x00000100,
	/// No operations on this file will block
	NonBlocking    = 0x00000200,
	/// If this file refers to terminal,
	/// do not set that tty as current process's
	/// controlling terminal
	NoTtyControl   = 0x00000400,
	/// Allow only meta operations, ie operations
	/// on file descriptor. Also used to indicate
	/// a location???
	Path           = 0x00000800,
	/// Acquire a read lock
	Read           = 0x00001000,
	/// Write directly to disk
	Sync           = 0x00002000,
	/// Create an unnamed temporary regular file.  The pathname
	/// argument specifies a directory; an unnamed inode will be
	/// created in that directory's filesystem.  Anything written
	/// to the resulting file will be lost when the last file
	/// descriptor is closed, unless the file is given a name.
	Temporary      = 0x00004000,
	/// If the file already exists and is a regular file and the
	/// access mode allows writing (i.e., is O_RDWR or O_WRONLY)
	/// it will be truncated to length 0.  If the file is a FIFO
	/// or terminal device file, the O_TRUNC flag is ignored.
	/// Otherwise, the effect of O_TRUNC is unspecified.
	Truncate       = 0x00008000,
	/// Acquire a write lock (may fail)
	Write          = 0x00010000,
}

impl core::ops::BitOr for OpenOptionsMask {
	type Output = u32;
	fn bitor(self, rhs: Self) -> u32 {
		self as u32 | rhs as u32
	}
}

impl core::ops::BitOr<u32> for OpenOptionsMask {
	type Output = u32;
	fn bitor(self, rhs: u32) -> u32 {
		self as u32 | rhs
	}
}

pub enum LockType {
	Unlocked = 0,
	Read     = 1,
	Write    = 2,
}

pub enum Whence {
	Start  = 0,
	Cursor = 1,
	End    = 2,
}

pub struct FileLock {
	/// Type of lock
	kind: LockType,
	/// Where start is relative to
	whence: Whence,
	/// The start of the lock, whence-relative to file
	start: u64,
	/// The length of the lock
	len: u64,
	/// The process that holds the lock
	pid: ProcessId,
}

pub struct FileMount {
	options: u32,
	offset: u128,
	lock: FileLock,
	buffer_filled: u16,
	buffer: [u8; 4096],
}

/// IDK
#[repr(u8)]
pub enum ModeFlag {
	Normal   = 0, // Normal file
	HardLink = 1, // Hard link, basically a COW
	SymLink  = 2, // Symbolic link
	CharDev  = 3, // Character device
	BlockDev = 4, // Block device
	Dir      = 5, // Directory
	Pipe     = 6, // Named pipe (FIFO)
}

pub struct File {
	/// The length for the file name.
	name_len: u16,
	/// The name of the file.
	name: [u8; 256],
	/// The File's mode
	mode: ModeFlag,
	/// The id of the user who owns this particular file.
	uid: u64,
	/// The id of the group that owns this file.
	gid: u64,
	/// The size in bytes of this file, not including
	/// this header.
	size: u128,
	/// Time file was last modified.
	// IDEA: seeing as we will not run out of time in a u64 move to that to save bytes
	/// Last modified time
	mtime: u64,
	/// Time file was last accessed
	atime: u64,
	/// Checksum for data, not including header.
	checksum: u64,
	linkname_len: u16,
	linkname: [u8; 256],
	magic: [u8; 6],
	version: [u8; 2],
	uname: [u8; 32],
	gname: [u8; 32],
	devmajor: u64,
	devminor: u64,
	unused: [u8; 362],
}

impl File {
	//    Create a file
	//    Delete a file
	//    Open a file
	//    Close a file
	//    Read data from a file
	//    Write data to a file
	//    Reposition the current file pointer in a file
	//    Append data to the end of a file
	//    Truncate a file (delete its contents)
	//    Rename a file
	//    Copy a file
}
