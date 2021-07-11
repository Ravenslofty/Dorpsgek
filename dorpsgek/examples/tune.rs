use dorpsgek::Tune;
use revad::tape::Tape;

fn main() {
    let tape = Tape::new();
    let mut tune = Tune::new(&tape);
    tune.tune(&tape);
}
