pub trait Cpu {
    fn refresh_cpu(&mut self);
    fn get_cpu_usage(&self)->Vec<f32>;
}