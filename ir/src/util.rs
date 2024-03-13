use std::sync::{Arc, Mutex};

pub fn create_arc_mutex<T>(data:T)->(Arc<Mutex<T>>,Arc<Mutex<T>>){
    let arc_mutex_data = Arc::new(Mutex::new(data));
    let arc_mutex_data_clone = Arc::clone(&arc_mutex_data);

    (arc_mutex_data,arc_mutex_data_clone)
}