use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

pub struct AudioSystem {
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
    music_sink: Option<Sink>,
}

impl AudioSystem {
    pub fn new() -> Self {
        match OutputStream::try_default() {
            Ok((stream, handle)) => AudioSystem {
                _stream: Some(stream),
                handle: Some(handle),
                music_sink: None,
            },
            Err(e) => {
                eprintln!("audio: no se pudo abrir el dispositivo de sonido: {e}");
                AudioSystem {
                    _stream: None,
                    handle: None,
                    music_sink: None,
                }
            }
        }
    }

    pub fn play_music_loop(&mut self, path: &str) {
        let Some(handle) = &self.handle else { return };

        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("audio: no se encontró '{path}' ({e}), sin música de fondo");
                return;
            }
        };

        let source = match Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("audio: no se pudo decodificar '{path}': {e}");
                return;
            }
        };

        match Sink::try_new(handle) {
            Ok(sink) => {
                sink.append(source.buffered().repeat_infinite());
                sink.play();
                self.music_sink = Some(sink);
            }
            Err(e) => eprintln!("audio: no se pudo crear el sink de música: {e}"),
        }
    }

    pub fn play_sfx(&self, path: &str) {
        let Some(handle) = &self.handle else { return };

        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("audio: no se encontró '{path}' ({e}), sin efecto de sonido");
                return;
            }
        };

        let source = match Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("audio: no se pudo decodificar '{path}': {e}");
                return;
            }
        };

        match Sink::try_new(handle) {
            Ok(sink) => {
                sink.append(source);
                sink.detach();
            }
            Err(e) => eprintln!("audio: no se pudo crear el sink de efecto: {e}"),
        }
    }
}
