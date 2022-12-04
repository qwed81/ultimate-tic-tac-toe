use super::{Opponent, UltimateTTTMove};
use serde::{de::DeserializeOwned, Serialize};
use std::io::{self, BufRead, Write};

pub struct NetworkOpponent<W, R>
where
    W: Write,
    R: BufRead,
{
    writer: W,
    reader: R,
}

impl<W, R> NetworkOpponent<W, R>
where
    W: Write,
    R: BufRead,
{
    pub fn new(writer: W, reader: R) -> Self {
        NetworkOpponent { writer, reader }
    }
}

impl<W, R> Opponent for NetworkOpponent<W, R>
where
    W: Write,
    R: BufRead,
{
    fn send_move(&mut self, ttt_move: UltimateTTTMove) -> Result<(), ()> {
        match to_writer(&mut self.writer, &ttt_move) {
            Ok(()) => Ok(()),
            Err(_) => Err(()),
        }
    }

    fn receive_move(&mut self) -> Result<UltimateTTTMove, ()> {
        let ttt_move_result = from_reader(&mut self.reader);
        match ttt_move_result {
            Ok(obj) => obj,
            Err(_) => Err(()),
        }
    }
}

// write objects split by the new line character
fn to_writer<W>(mut writer: W, obj: &impl Serialize) -> io::Result<()>
where
    W: Write,
{
    let obj_json = serde_json::to_string(obj).unwrap();
    writer.write(obj_json.as_bytes())?;
    writer.write("\n".as_bytes())?;
    writer.flush()?;

    Ok(())
}

// split json on the new line character
fn from_reader<R, D>(mut reader: R) -> io::Result<D>
where
    R: BufRead,
    D: DeserializeOwned,
{
    // keep reading input until it hits a new line, that
    // is the end of this object
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let obj = serde_json::from_str::<D>(&buffer)?;
    Ok(obj)
}
