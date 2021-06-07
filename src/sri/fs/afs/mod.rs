//! The Able File System will be the system wide file system

#[repr(transparent)]
pub struct ProcessId(u64);

pub struct FileMount {
	offset: u128,
	process_lock_id: Option<ProcessId>,
}

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

#[repr(u8)]
pub enum Type {
	x, // TODO: Actually implement this
}

pub struct File {
	/// The length for the file name.
	name_len: u8,
	/// The name of the file.
	name: [u8; 256],
	/// AAAAAAAAAAAAA
	mode: ModeFlag,
	/// The id of the user who owns this particular file.
	uid: u64,
	/// The id of the group that owns this file.
	gid: u64,
	/// The size in bytes of this file, not including
	/// this header.
	size: u128,
	/// Time file was last modified.
	mtime: u128,
	/// Checksum for data, not including header.
	checksum: u64,
	///
	typeflag: Type,
	linkname_len: u8,
	linkname: [u8; 256],
	magic: [u8; 6],
	version: [u8; 2],
	uname: [u8; 32],
	gname: [u8; 32],
	devmajor: u64,
	devminor: u64,
	unused: [u8; 363],
}

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
