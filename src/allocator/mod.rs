/// All the allocators available to ableOS
pub mod fixed_size_block;

/// The starting location of the heap
pub const HEAP_START: usize = 0x_4444_4444_0000;
/// TODO: owo what is this?
pub const HEAP_SIZE: usize = 200 * 1024; // 100 KiB

use fixed_size_block::FixedSizeBlockAllocator;
#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

/*
use linked_list_allocator::LockedHeap;
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
*/

use x86_64::{
	structures::paging::{
		mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
	},
	VirtAddr,
};

/// TODO: owo?
pub fn init_heap(
	mapper: &mut impl Mapper<Size4KiB>,
	frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
	let page_range = {
		let heap_start = VirtAddr::new(HEAP_START as u64);
		let heap_end = heap_start + HEAP_SIZE - 1u64;
		let heap_start_page = Page::containing_address(heap_start);
		let heap_end_page = Page::containing_address(heap_end);
		Page::range_inclusive(heap_start_page, heap_end_page)
	};

	for page in page_range {
		let frame = frame_allocator
			.allocate_frame()
			.ok_or(MapToError::FrameAllocationFailed)?;
		let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
		unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
	}
	unsafe {
		ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
	}
	Ok(())
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
	panic!("allocation error: {:?}", layout)
}

/// (Able) No clue what this does
pub struct Locked<A> {
	inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
	/// Creates a new Locked spin mutex
	pub const fn new(inner: A) -> Self {
		Locked {
			inner: spin::Mutex::new(inner),
		}
	}
	/// Locks the spin mutex
	pub fn lock(&self) -> spin::MutexGuard<A> {
		self.inner.lock()
	}
}

#[test_case]
fn allocate_test() {
	use alloc::vec::Vec;
	let mut vec = Vec::new();
	for i in 0..500 {
		vec.push(i);
	}
}
