use core::{
	pin::Pin,
	task::{Context, Poll},
};

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{task::AtomicWaker, Stream};

use crate::println;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub struct ScancodeStream {
	_private: (),
}

impl ScancodeStream {
	pub fn new() -> Self {
		SCANCODE_QUEUE
			.try_init_once(|| ArrayQueue::new(100))
			.expect("ScancodeStream::new should only be called once");
		ScancodeStream { _private: () }
	}
}

impl Stream for ScancodeStream {
	type Item = u8;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		let queue = SCANCODE_QUEUE.try_get().expect("Not initialized");

		if let Some(scancode) = queue.pop() {
			return Poll::Ready(Some(scancode));
		}

		WAKER.register(&cx.waker());
		match queue.pop() {
			Some(scancode) => {
				WAKER.take();
				Poll::Ready(Some(scancode))
			},
			None => Poll::Pending,
		}
	}
}

pub(crate) fn add_scancode(scancode: u8) {
	if let Ok(queue) = SCANCODE_QUEUE.try_get() {
		if let Err(_) = queue.push(scancode) {
			println!("WARNING: scancode queue full; dropping keyboard input");
		}
	} else {
		println!("WARNING: scancode queue uninitialized");
	}
}
