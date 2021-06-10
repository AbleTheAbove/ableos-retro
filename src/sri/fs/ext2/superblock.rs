#[repr(u32)]
pub enum Ext2FeatureMasks {
	/// Preallocate some number of (contiguous?) blocks (see byte 205 in the superblock) to a directory when creating a new one (to reduce fragmentation?)
	Preallocate        = 0x0001,
	/// AFS server inodes exist
	AFSServer          = 0x0002,
	/// File system has a journal (Ext3)
	Ext3               = 0x0004,
	/// Inodes have extended attributes
	InodeExtensions    = 0x0008,
	/// File system can resize itself for larger partitions
	ResizableFilSystem = 0x0010,
	/// Directories use hash index
	HashIndexing       = 0x0020,
}

impl core::ops::BitOr for Ext2FeatureMasks {
	type Output = u32;
	fn bitor(self, rhs: Self) -> u32 {
		self as u32 | rhs as u32
	}
}

impl core::ops::BitOr<u32> for Ext2FeatureMasks {
	type Output = u32;
	fn bitor(self, rhs: u32) -> u32 {
		self as u32 | rhs
	}
}

#[repr(u32)]
pub enum FeatureRequiredMasks {
	/// Compression is used
	Compression     = 0x0001,
	/// Directory entries contain a type field
	DictionaryTypes = 0x0002,
	/// File system needs to replay its journal
	JournalReplay   = 0x0004,
	/// File system uses a journal device
	Journal         = 0x0008,
}

impl core::ops::BitOr for FeatureRequiredMasks {
	type Output = u32;
	fn bitor(self, rhs: Self) -> u32 {
		self as u32 | rhs as u32
	}
}

impl core::ops::BitOr<u32> for FeatureRequiredMasks {
	type Output = u32;
	fn bitor(self, rhs: u32) -> u32 {
		self as u32 | rhs
	}
}

pub enum WriteFeatures {
	/// Sparse superblocks and group descriptor tables
	Sparse                = 0x0001,
	/// File system uses a 64-bit file size
	FileSizeU64           = 0x0002,
	/// Directory contents are stored in the form of a Binary Tree
	DirectoryIsBinaryTree = 0x0004,
}

#[repr(u16)]
pub enum FileSystemState {
	Clean  = 1,
	Errors = 2,
}

#[repr(u16)]
pub enum ErrorContingency {
	Ignore          = 1,
	RemountReadOnly = 2,
	KernelPanic     = 3,
}

pub struct SuperBlock {
	/// Total number of inodes in file system
	pub total_inodes: u32,
	/// Total number of blocks in file system
	pub total_blocks: u32,
	/// Number of blocks reserved for superuser (see offset 80)
	pub super_user_blocks: u32,
	/// Total number of unallocated blocks
	pub total_unallocated_blocks: u32,
	/// Total number of unallocated inodes
	pub total_unallocated_inodes: u32,
	/// Block number of the block containing the superblock
	pub super_block_number: u32,
	/// log2 (block size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the block size)
	pub log2_block_size: u32,
	/// log2 (fragment size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the fragment size)
	pub log2_fragment_size: u32,
	/// Number of blocks in each block group
	pub blocks_per_group: u32,
	/// Number of fragments in each block group
	pub fragments_per_group: u32,
	/// Number of inodes in each block group
	pub inodes_per_group: u32,
	/// Last mount time (in POSIX time)
	pub last_mount_time: u32,
	/// Last written time (in POSIX time)
	pub last_written_time: u32,
	/// Number of times the volume has been mounted since its last consistency check (fsck)
	pub mounts_since_check: u16,
	/// Number of mounts allowed before a consistency check (fsck) must be done
	pub max_mounts_before_check: u16,
	/// Ext2 signature (0xef53), used to help confirm the presence of Ext2 on a volume
	pub signature: u16,
	/// File system state (see below)
	pub file_system_state: FileSystemState,
	/// What to do when an error is detected (see below)
	pub error_contingency: ErrorContingency,
	/// Minor portion of version (combine with Major portion below to construct full version field)
	pub minor: u16,
	/// POSIX time of last consistency check (fsck)
	pub last_consistency_check: u32,
	/// Interval (in POSIX time) between forced consistency checks (fsck)
	pub time_between_foced_checks: u32,
	/// Operating system ID from which the filesystem on this volume was created (see below)
	pub os_id: u32,
	/// Major portion of version (combine with Minor portion above to construct full version field)
	pub major: u32,
	/// User ID that can use reserved blocks
	pub super_user_id: u16,
	/// Group ID that can use reserved blocks
	pub super_group_id: u16,
	/// First non-reserved inode in file system. (In versions < 1.0, this is fixed as 11)
	pub first_non_reserved_inode: u32,
	/// Size of each inode structure in bytes. (In versions < 1.0, this is fixed as 128)
	pub inode_struct_size: u16,
	/// Block group that this superblock is part of (if backup copy)
	pub home_block_group: u16,
	/// Optional features present (features that are not required to read or write, but usually result in a performance increase. see below)
	pub optional_features: u32,
	/// Required features present (features that are required to be supported to read or write. see below)
	pub required_features: u32,
	/// Features that if not supported, the volume must be mounted read-only see below)
	pub write_features: u32,
	/// File system ID (what is output by blkid)
	pub file_system_id: [u8; 16],
	/// Volume name (C-style string: characters terminated by a 0 byte)
	pub volume_name: [u8; 16],
	/// Path volume was last mounted to (C-style string: characters terminated by a 0 byte)
	pub last_mount_location: [u8; 64],
	/// Compression algorithms used (see Required features above)
	pub compression_used: u32,
	/// Number of blocks to preallocate for files
	pub blocks_preallocate_files: u8,
	/// Number of blocks to preallocate for directories
	pub blocks_preallocate_directories: u8,
	/// (Unused)
	_unused: u16,
	/// Journal ID (same style as the File system ID above)
	pub jounal_id: [u8; 16],
	/// Journal inode
	pub journal_inode: u32,
	/// Journal device
	pub journal_device: u32,
	/// Head of orphan inode list
	pub head_orphan_inode_list: u32,
	/// (Unused)
	__unused: [u8; 788],
}

impl SuperBlock {
	/// From the Superblock, extract the size of each block, the total number of inodes, the total
	/// number of blocks, the number of blocks per block group, and the number of inodes in each block
	/// group. From this information we can infer the number of block groups there are by:
	///    Rounding up the total number of blocks divided by the number of blocks per block group
	///    Rounding up the total number of inodes divided by the number of inodes per block group
	///    Both (and check them against each other)
	pub fn get_num_block_groups(&self) -> u32 {
		((self.total_blocks / self.blocks_per_group) + (self.total_inodes / self.inodes_per_group))
			/ 2
	}
	/// From an inode address (remember that they start
	/// at 1), we can determine which group the inode is
	/// in, by using the formula:
	pub fn get_block_group(&self, addr: u32) -> u32 {
		(addr - 1) / self.inodes_per_group
	}
	/// Once we know which group an inode resides in, we
	/// can look up the actual inode by first retrieving
	/// that block group's inode table's starting address
	/// (see Block Group Descriptor above). The index of
	/// our inode in this block group's inode table can be
	/// determined by using the formula:
	pub fn get_index(&self, addr: u32) -> u32 {
		(addr - 1) % self.inodes_per_group
	}

	/// Next, we have to determine which block contains
	/// our inode. This is achieved from:
	pub fn get_inode_block_addr(&self, index: u32) -> u32 {
		(index * self.inode_struct_size as u32) / self.log2_block_size
	}
}
