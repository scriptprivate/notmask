use std::io::{self, Read, Write};

// component trait
trait Component {
    fn process(&mut self) -> io::Result<()>;
}

// input component
struct Reader<R: Read> {
    source: R,
    output: Vec<u8>,
}

impl<R: Read> Reader<R> {
    fn new(source: R) -> Self {
        Reader {
            source,
            output: Vec::new(),
        }
    }

    fn get_output(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.output)
    }
}

impl<R: Read> Component for Reader<R> {
    fn process(&mut self) -> io::Result<()> {
        let mut buffer = [0; 1];
        while self.source.read(&mut buffer)? != 0 {
            self.output.push(buffer[0]);
        }
        Ok(())
    }
}

// transformer component
struct Transformer {
    input: Vec<u8>,
    output: Vec<u8>,
}

impl Transformer {
    fn new() -> Self {
        Transformer {
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    fn set_input(&mut self, input: Vec<u8>) {
        self.input = input;
    }

    fn get_output(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.output)
    }

    fn transform(byte: u8) -> u8 {
        !byte // bitwise NOT operation
    }
}

impl Component for Transformer {
    fn process(&mut self) -> io::Result<()> {
        self.output = self.input.iter().map(|&b| Self::transform(b)).collect();
        Ok(())
    }
}

// output port
struct Writer<W: Write> {
    destination: W,
    input: Vec<u8>,
}

impl<W: Write> Writer<W> {
    fn new(destination: W) -> Self {
        Writer {
            destination,
            input: Vec::new(),
        }
    }

    fn set_input(&mut self, input: Vec<u8>) {
        self.input = input;
    }
}

impl<W: Write> Component for Writer<W> {
    fn process(&mut self) -> io::Result<()> {
        self.destination.write_all(&self.input)?;
        Ok(())
    }
}

// network coordinator
struct Network {
    reader: Reader<io::StdinLock<'static>>,
    transformer: Transformer,
    writer: Writer<io::StdoutLock<'static>>,
}

impl Network {
    fn new() -> Self {
        Network {
            reader: Reader::new(io::stdin().lock()),
            transformer: Transformer::new(),
            writer: Writer::new(io::stdout().lock()),
        }
    }

    fn run(&mut self) -> io::Result<()> {
        // process data through the network
        self.reader.process()?;

        // transfer data between components
        let data = self.reader.get_output();
        self.transformer.set_input(data);

        self.transformer.process()?;

        let transformed_data = self.transformer.get_output();
        self.writer.set_input(transformed_data);

        self.writer.process()?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut network = Network::new();
    network.run()
}
