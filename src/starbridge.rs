use crossbeam_channel::{unbounded, Receiver, Sender};

enum StarBridgeSource {
    Serial {
        com_port: String,
        baudrate: u32,
    },
    LocalFile {
        // TODO: maybe a Path
        filename: String,
    },
}

struct StarBridgeManager {
    tx_source: Sender<Vec<u8>>,
    rx_sink: Receiver<Vec<u8>>,
    source: Option<StarBridgeSource>,
}

impl StarBridgeManager {

    pub fn new() -> Self {
        let (tx, rx) = unbounded::<Vec<u8>>();

        Self {
            tx_source: tx,
            rx_sink: rx,
            source: None
        }
    }

    pub fn with_configuration(&mut self) {
        // TODO: parse a json configuration with 
        //      - Serial and COM port
        //      - File and filename/path

    }
}
