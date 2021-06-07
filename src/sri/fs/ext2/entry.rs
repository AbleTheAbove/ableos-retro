use crate::alloc::vec::Vec;

pub struct Entry {
	/// Inode
	inode: u32,
	/// Total size of this entry (Including all subfields)
	total_size: u16,
	/// Name Length least-significant 8 bits
	lower_size: u8,
	/// Type indicator (only if the feature bit for "directory entries have file type byte" is set, else this is the most-significant 8 bits of the Name Length)
	upper_size: u8,
	/// Name characters
	name: Vec<u8>,
}
