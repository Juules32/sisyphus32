use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::Arc;

const PARALLELIZE_THRESHOLD: usize = 3;

static mut GLOBAL_THREAD_POOL: Option<Arc<ThreadPool>> = None;

pub(crate) struct GlobalThreadPool;

impl GlobalThreadPool {
    pub(crate) unsafe fn init() {
        Self::set_threadpool(1);
    }

    pub(crate) fn get() -> Arc<ThreadPool> {
        #[allow(static_mut_refs)]
        unsafe { GLOBAL_THREAD_POOL.clone().unwrap() }
    }

    pub(crate) fn should_parallelize() -> bool {
        Self::get().current_num_threads() >= PARALLELIZE_THRESHOLD
    }

    pub(crate) fn set_threadpool(num_threads: usize) {
        unsafe { GLOBAL_THREAD_POOL = Some(Arc::new(ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap())) };
    }
}
