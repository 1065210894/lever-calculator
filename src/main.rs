use iced::*;

const FONT: Font = Font::External {name: "濑户字体", bytes: include_bytes!("../setofont.ttf")};

fn main() {
    <Counter as iced:: Application>::run(Settings::default());
}

#[derive(Debug, Default)]
struct  Counter {
    v: i32,
    inc: button::State,
    dec: button::State
}

#[derive(Debug, Clone)]
enum Msg{
    Inc,
    Dec
}

impl Sandbox for Counter {
    type Message = Msg;

    fn new() -> Self {
        Default::default()
    }

    fn title(&self) -> String {
        String::from("Hello Rust!")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Msg::Inc => self.v += 1,
            Msg::Dec => self.v -= 1
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(30)
            .push(Button::new(&mut self.inc, Text::new("上升").font(FONT)).on_press(Msg::Inc))
            .push(Text::new(self.v.to_string()))
            .push(Button::new(&mut self.dec, Text::new("下降").font(FONT)).on_press(Msg::Dec))
            .into()
    }
}