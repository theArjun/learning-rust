use num2words::Num2Words;
use titlecase::Titlecase;

fn main() {
    let number = 1456;
    let words = Num2Words::new(number).to_words();
    match words {
        Ok(value) => {
            println!("{:?}", value.titlecase());
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
