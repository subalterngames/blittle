pub use rayon::max_num_threads;

/// Parameters for multi-threaded blitting.
#[derive(Copy, Clone)]
pub enum ThreadedBlitParams {
    /// The number of pixel rows per threaded chunk.
    ChunkSize(usize),
    /// Use up to this many threads to parallelize chunks.
    /// The chunk size is: `image.height / (NumThreads.0.min(max_num_threads))`
    NumThreads(usize),
    /// The number of chunks.
    /// The chunk size is: `image.height / NumChunks.0`
    NumChunks(usize),
}


impl ThreadedBlitParams {
    pub fn get_chunk_size(self, height: usize) -> usize {
        match self {
            Self::ChunkSize(size) => size,
            Self::NumThreads(num) => height / num.min(max_num_threads()),
            Self::NumChunks(num) => height / num
        }
    }
}

impl Default for ThreadedBlitParams {
    fn default() -> Self {
        Self::ChunkSize(128)
    }
}