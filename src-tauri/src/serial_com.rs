use std::io;
use tauri::Window;

pub struct SerialCom {
    pub window: Window,
}
impl SerialCom {
    pub fn new(win: Window) -> SerialCom {
        return SerialCom { window: win };
    }

    pub fn start(&self) {
        println!("Starting serial Port");

        let mut port = serialport::new("/dev/ttyUSB0", 9600)
            .open()
            .expect("Failed to open serial port");

        let mut buffer: [u8; 1] = [0; 1];
        let mut cmd_line = String::new();
        loop {
            match port.read(&mut buffer) {
                Ok(bytes) => {
                    if bytes == 1 {
                        self.receive_byte(buffer[0], &mut cmd_line);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }

    fn receive_byte(&self, u: u8, line: &mut String) {
        if u == 124 {
            self.send_new_command(line);
            line.clear();
        } else {
            line.push(u as char);
        }
    }

    fn send_new_command(&self, command: &String) {
        println!("Received: {}", command);
        self.window
            .emit(
                "new-command",
                NewCommandPayload {
                    new_command: format!("Received: {}", command).into(),
                },
            )
            .unwrap();
    }
}

#[derive(Clone, serde::Serialize)]
struct NewCommandPayload {
    new_command: String,
}
