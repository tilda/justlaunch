use std::os::unix::net::{UnixListener, UnixStream};

fn figure_out_function(stream: UnixStream) {
    // NOTE: I haven't programmed anything to handle events yet
    // so this is going to be here for now.
    // It's temporary, okay? Okay.
    return;
}

fn send_back_error(error: u32, stream: UnixStream) {
    // I also haven't figured this out yet.
    return;
}

fn main() {
    let listener = UnixListener::bind("/tmp/justlaunchd.sock")
        .unwrap()
        .expect("Could not bind to socket");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| figure_out_function(stream));
            }

            Err(stream) => {
                thread::spawn(|| send_back_error(stream));
            }
        }
    }
}
