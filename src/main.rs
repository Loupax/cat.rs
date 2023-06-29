use std::{
    env,
    fs::File,
    io::{self, BufReader, Read, Write}
};

use atty::Stream;

const CHUNK: usize = 1024;
fn main()->io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let files = &args[1..];
    
    if atty::is(Stream::Stdin){
        for i in files{
            let file = File::open(i)?;
            let mut reader = BufReader::new(file);

            output(&mut reader)?;
        }
        return Ok(());
    }
    
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    output(&mut reader)

}

fn output<R: Read>(reader: &mut R) -> io::Result<()>{
    loop{
        let mut buf = vec![0u8; CHUNK];
        let len = reader.read(&mut buf)?;
        if len < CHUNK {
            buf.truncate(len);
            io::stdout().write(&buf)?;
            break
        } else {
            io::stdout().write(&buf)?;
        }
    }
    Ok(())
}
