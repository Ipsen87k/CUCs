pub trait Memory {
    fn get_total_memory(&self)->u64;
    fn get_used_memory(&self)->u64;
}