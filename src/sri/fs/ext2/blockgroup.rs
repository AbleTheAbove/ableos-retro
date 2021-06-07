pub struct BlockGroup {
	/// Block address of block usage bitmap
	pub block_usage_bitmap: u32,
	/// Block address of inode usage bitmap
	pub inode_usage_bitmap: u32,
	/// Starting block address of inode table
	pub inode_table_addr: u32,
	/// Number of unallocated blocks in group
	pub num_unallocated_blocks: u16,
	/// Number of unallocated inodes in group
	pub num_unallocated_inodes: u16,
	/// Number of directories in group
	pub num_dirs_in_group: u16,
	/// (Unused)
	pub unused: [u8; 14],
}
