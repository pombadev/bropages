use monis::Monis;

pub enum SyntaxHighlight {
    On,
    Off,
}

pub enum Pager {
    On,
    Off,
}

pub struct Config {
    syntax: SyntaxHighlight,
    pager: Pager
}

impl Config {
    pub fn write_to_stdio(self, content: &mut str) {
        match self.pager {
            Pager::Off => {
                monis::Moins::run(content, None);
            },
            Pager::On => {
                println!("{}", content);
            }
        }
    }
}
