pub enum InodeType {
	CLEAR           = 0x0FFF, // For clearing the top hex digit
	FIFO            = 0x1000,
	CharacterDevice = 0x2000,
	Directory       = 0x4000,
	Blockdevice     = 0x6000,
	RegularFile     = 0x8000,
	SymbolicLink    = 0xA000,
	UnixSocket      = 0xC000,
}

pub enum PermissionMasks {
	OtherExecute = 0x001, // Other—execute permission
	OtherWrite   = 0x002, // Other—write permission
	OtherRead    = 0x004, // Other—read permission
	GroupExecute = 0x008, // Group—execute permission
	GroupWrite   = 0x010, // Group—write permission
	GroupRead    = 0x020, // Group—read permission
	UserExecute  = 0x040, // User—execute permission
	UserWrite    = 0x080, // User—write permission
	UserRead     = 0x100, // User—read permission
	StickyBit    = 0x200, // Sticky Bit
	SetGroupId   = 0x400, // Set group ID
	SetUserId    = 0x800, // Set user ID
}

impl core::ops::BitOr for PermissionMasks {
	type Output = u16;
	fn bitor(self, rhs: Self) -> u16 {
		self as u16 | rhs as u16
	}
}

impl core::ops::BitOr<u16> for PermissionMasks {
	type Output = u16;
	fn bitor(self, rhs: u16) -> u16 {
		self as u16 | rhs
	}
}

pub enum InodeFlags {
	SecureDelete       = 0x00000001, // 	Secure deletion (not used)
	PersistAfterDelete = 0x00000002, // 	Keep a copy of data when deleted (not used)
	Compression        = 0x00000004, // 	File compression (not used)
	Sync               = 0x00000008, // 	Synchronous updates—new data is written immediately to disk
	Immutable          = 0x00000010, // 	Immutable file (content cannot be changed)
	AppendOnly         = 0x00000020, // 	Append only
	InvisibleToDump    = 0x00000040, // 	File is not included in 'dump' command
	NoUpdateLatAccess  = 0x00000080, // 	Last accessed time should not updated
	// ... 	(Reserved)
	HashIndexDir       = 0x00010000, //	Hash indexed directory
	AFSDir             = 0x00020000, //	AFS directory
	JournalFileData    = 0x00040000, //	Journal file data
}

impl core::ops::BitOr for InodeFlags {
	type Output = u32;
	fn bitor(self, rhs: Self) -> u32 {
		self as u32 | rhs as u32
	}
}

impl core::ops::BitOr<u32> for InodeFlags {
	type Output = u32;
	fn bitor(self, rhs: u32) -> u32 {
		self as u32 | rhs
	}
}

pub struct Inode {
	/// 	Type and Permissions (see below)
	permissions: u16,
	/// 	User ID
	lower_userid: u16,
	/// 	Lower 32 bits of size in bytes
	lower_size: u32,
	/// 	Last Access Time (in POSIX time)
	last_access: u32,
	/// 	Creation Time (in POSIX time)
	created: u32,
	/// 	Last Modification time (in POSIX time)
	last_modified: u32,
	/// 	Deletion time (in POSIX time)
	deleted: u32,
	/// 	Group ID
	lower_groupid: u16,
	/// 	Count of hard links (directory entries) to this inode. When this reaches 0, the data blocks are marked as unallocated.
	num_hardlinks: u16,
	/// 	Count of disk sectors (not Ext2 blocks) in use by this inode, not counting the actual inode structure nor directory entries linking to the inode.
	num_sectors: u32,
	/// 	Flags (see below)
	inode_flags: u32,
	/// 	Operating System Specific value #1
	os_specific_1: u32,
	/// 	Direct Block Pointer 0
	direct_block_ptr_0: u32,
	/// 	Direct Block Pointer 1
	direct_block_ptr_1: u32,
	/// 	Direct Block Pointer 2
	direct_block_ptr_2: u32,
	/// 	Direct Block Pointer 3
	direct_block_ptr_3: u32,
	/// 	Direct Block Pointer 4
	direct_block_ptr_4: u32,
	/// 	Direct Block Pointer 5
	direct_block_ptr_5: u32,
	/// 	Direct Block Pointer 6
	direct_block_ptr_6: u32,
	/// 	Direct Block Pointer 7
	direct_block_ptr_7: u32,
	/// 	Direct Block Pointer 8
	direct_block_ptr_8: u32,
	/// 	Direct Block Pointer 9
	direct_block_ptr_9: u32,
	/// 	Direct Block Pointer 10
	direct_block_ptr_a: u32,
	/// 	Direct Block Pointer 11
	direct_block_ptr_b: u32,
	/// 	Singly Indirect Block Pointer (Points to a block that is a list of block pointers to data)
	singly_indeirect_ptr: u32,
	/// 	Doubly Indirect Block Pointer (Points to a block that is a list of block pointers to Singly Indirect Blocks)
	doubly_indirect_ptr: u32,
	/// 	Triply Indirect Block Pointer (Points to a block that is a list of block pointers to Doubly Indirect Blocks)
	tripl_indirect_ptr: u32,
	/// 	Generation number (Primarily used for NFS)
	generation_number: u32,
	/// 	In Ext2 version 0, this field is reserved. In version >= 1, Extended attribute block (File ACL).
	file_acl: u32,
	/// 	In Ext2 version 0, this field is reserved. In version >= 1, Upper 32 bits of file size (if feature bit set) if it's a file, Directory ACL if it's a directory
	upper_size_or_dir_acl: u32,
	/// 	Block address of fragment
	fragment_block_address: u32,
	/// Fragment number
	fragment_number: u8,
	/// Fragment size
	fragment_size: u8,
	/// (reserved)
	reserved_1: u16,
	/// High 16 bits of 32-bit User ID
	higher_userid: u16,
	/// High 16 bits of 32-bit Group ID
	higher_groupid: u16,
	/// (reserved)
	reserved_2: u32,
}
