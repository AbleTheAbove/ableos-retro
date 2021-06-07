//! The Able File System will be the system wide file system

#[repr(u64)]
pub enum ModeFlag {
	Normal = 0,   // '0' 	Normal file
	HardLink = 1, // '1' 	Hard link
	SymLink = 2,  // '2' 	Symbolic link
	CharDev = 3,  // '3' 	Character device
	BlockDev = 4, // '4' 	Block device
	Dir = 5,      // '5' 	Directory
	Pipe = 6,     // '6' 	Named pipe (FIFO)
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
	/// The File's mode
	mode: ModeFlag,
	/// The id of the user who owns this particular file.
	uid: u64,
	/// The id of the group that owns this file.
	gid: u64,
	/// The size in bytes of this file, not including
	/// this header.
	size: u128,
	// IDEA: seeing as we will not run out of time in a u64 move to that to save bytes
	/// Last modified time
	mtime: u128,
	chksum: u64,
	typeflag: Type,
	linkname_len: u8,
	linkname: [u8; 256],
	magic: [u8; 6],
	version: [u8; 2],
	uname: [u8; 32],
	gname: [u8; 32],
	devmajor: u64,
	devminor: u64,
	unused: [u8; 354],
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
