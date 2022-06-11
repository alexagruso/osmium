use std::env;
use std::io::{stdout, stdin, Write, Stdout};

use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

use osmium_core::buffer::Buffer;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut buffers: Vec<Buffer> = Vec::new();

    if args.len() == 0 {
        buffers.push(Buffer::new(None));
    } else {
        for file_path in args.iter() {
            let buffer = Buffer::new(Some(file_path.clone()));
            buffers.push(buffer);
        }
    }

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

    for event in stdin.events() {
        
    }
}
