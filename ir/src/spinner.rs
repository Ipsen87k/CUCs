use std::{io::{self, Write}, sync::{Arc, Mutex}, thread, time::Duration};

pub struct Spinner{
    image_execute_is_end:Mutex<bool>,
}

impl Spinner{
    pub fn new()->Self{
        Self{
            image_execute_is_end:Mutex::new(false),
        }
    }

    pub fn spinner_show(&self){
        let spinner = vec!['/','-','\\','|'];

        let mut count =0;
        loop {
            if *self.image_execute_is_end.lock().unwrap(){
                break;
            }
            print!("\r{}",spinner[count]);
            io::stdout().flush().unwrap();

            count = (count+1)  %spinner.len();
            thread::sleep(Duration::from_secs(1));
        }
    }
}

pub fn spinner_show(img_execute_is_end:Arc<Mutex<bool>>){
    let spinner = vec!['/','-','\\','|'];

    let mut count =0;
    loop {
        if let Ok(img_execute_is_end) = img_execute_is_end.try_lock(){
            if *img_execute_is_end{
                break;
            }
        }
        print!("\r{}",spinner[count]);
        io::stdout().flush().unwrap();

        count = (count+1)  %spinner.len();
        thread::sleep(Duration::from_secs(1));
    }

    println!("\n完了");
}