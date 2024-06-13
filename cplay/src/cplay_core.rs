use std::{fs::File, io::BufReader};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use crate::{error::Error, CResult};

pub struct Sound {
    sink: Sink,
    stream: OutputStream,
    handle: OutputStreamHandle,
}

impl Sound {
    pub fn new() -> CResult<Box<Self>> {
        let (stream, handle) = OutputStream::try_default().map_err(|e| Error::StreamError(e))?;
        let sink = Sink::try_new(&handle).map_err(|e| Error::PlayError(e))?;

        Ok(Box::new(Self {
            sink: sink,
            stream: stream,
            handle: handle,
        }))
    }

    pub fn append(&mut self,file:Box<BufReader<File>>)->CResult<()>{
        let src = Decoder::new(file).map_err(|e| Error::DecoderError(e))?;
        self.sink.append(src);

        Ok(())
    }

    pub fn play(&self){
        self.sink.sleep_until_end();
    }
}

#[cfg(test)]
mod test{
    use std::time::Duration;

    use rodio::Source;

    use super::*;
    
    fn open_test_file(file:&str)->Box<BufReader<File>>{
        Box::new(BufReader::new(File::open(file).unwrap()))
    }

    #[test]
    fn sound_play_test(){
        let mp3_1= open_test_file("./tests/res/j.mp3");
        let mp3_2 = open_test_file("./tests/res/g.mp3");
        let wav_1 = open_test_file("./tests/res/m.wav");

        let mut s = Sound::new().unwrap();
        s.append(wav_1).unwrap();
        s.append(mp3_1).unwrap();
        s.append(mp3_2).unwrap();
        s.play();
    }

    fn sound_play_reverv_test(){
        let mp3_2 = open_test_file("./tests/res/g.mp3");
        let mut s = Sound::new().unwrap();
        let src = Decoder::new(mp3_2).map_err(|e| Error::DecoderError(e)).unwrap();
        
        
        //let with_reverb = src.reverb(Duration::from_millis(40), 0.7);
    }
}