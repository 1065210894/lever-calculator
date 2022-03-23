#![windows_subsystem = "windows"]

use iced::*;

const FONT: Font = Font::External {name: "濑户字体", bytes: include_bytes!("../setofont.ttf")};

fn main() {
    let mut settings = iced::Settings::default();
    settings.window.size = (650, 150);
    <Counter as iced:: Sandbox>::run(settings);
}

#[derive(Debug, Default)]
struct Counter {
    original_money: text_input::State,
    original_money_str : String,

    leverage: text_input::State,
    leverage_str : String,

    market_price: text_input::State,
    market_price_str : String,

    latest_price: text_input::State,
    latest_price_str : String,

    leverage_type: text_input::State,
    leverage_type_str: String,

    profit: String,

    error: String
}

impl Counter {

    pub fn recalculation(&mut self) {
        let original_money_float = self.original_money_str.parse::<f32>().unwrap();
        let leverage_float = self.leverage_str.parse::<f32>().unwrap();
        let market_price_float = self.market_price_str.parse::<f32>().unwrap();
        let latest_price_float = self.latest_price_str.parse::<f32>().unwrap();

        let cwe = original_money_float * leverage_float;
        let count = cwe / market_price_float;

        match self.leverage_type_str.as_str() {
            "涨" => {
                let value = latest_price_float - market_price_float;
                let result :f32 = value * count;
                self.bc_check(result, original_money_float);
            }
            "跌"=>{
                let value = market_price_float - latest_price_float;
                let result :f32 = value * count;
                self.bc_check(result, original_money_float);
            }
            _ => {
                self.profit = String::new();
                self.error = String::from("杠杆类型不正确");
                return;
            }
        }
    }

    fn bc_check(&mut self, profit : f32, original_money_float : f32) {
        if (profit + original_money_float) <= 0 as f32 {
            self.profit = String::from("爆仓了！")
        } else {
            self.profit = profit.to_string();
        }
    }

    pub fn check_param(&mut self) {
        self.profit = String::new();
        if &self.original_money_str == "" {
            self.error = String::from("本金数据不正确");
            return;
        }

        if &self.leverage_str == "" {
            self.error = String::from("杠杆数据不正确");
            return;
        }

        if &self.market_price_str == "" {
            self.error = String::from("市价数据不正确");
            return;
        }

        if &self.latest_price_str == "" {
            self.error = String::from("最新价数据不正确");
            return;
        }

        if &self.leverage_type_str == "" {
            self.error = String::from("杠杆类型数据不正确");
            return;
        }

        self.error = String::new();
        self.recalculation();
    }
}

#[derive(Debug, Clone)]
enum Msg{
    OriginalMoney(String),
    Leverage(String),
    MarketPrice(String),
    LatestPrice(String),
    LeverageType(String)
}

impl Sandbox for Counter {
    type Message = Msg;

    fn new() -> Self {
        Default::default()
    }

    fn title(&self) -> String {
        String::from("杠杆计算器")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Msg::OriginalMoney(value) => {
               self.original_money_str = value
            },
            Msg::LatestPrice(value)=>{
                self.latest_price_str = value
            },
            Msg::Leverage(value) => {
                self.leverage_str = value
            },
            Msg::MarketPrice(value)=>{
                self.market_price_str = value
            },
            Msg::LeverageType(value)=>{
                self.leverage_type_str = value
            }
        }
        self.check_param();
    }

    fn view(&mut self) -> Element<Self::Message> {

        let original_money = TextInput::
        new(&mut self.original_money, "本金",
            &mut self.original_money_str,move |value : String| -> Msg {Msg::OriginalMoney(value)})
            .font(FONT)
            .padding(5)
            .width(Length::Units(100));

        let leverage = TextInput::
        new(&mut self.leverage, "杠杆倍数",
            &mut self.leverage_str, move |value : String| -> Msg {Msg::Leverage(value)})
            .font(FONT)
            .padding(5)
            .width(Length::Units(100));

        let market_price = TextInput::
        new(&mut self.market_price, "市价",
            &mut self.market_price_str, move |value : String| -> Msg {Msg::MarketPrice(value)})
            .font(FONT)
            .padding(5)
            .width(Length::Units(100));

        let latest_price = TextInput::
        new(&mut self.latest_price, "最新价",
            &mut self.latest_price_str, move |value : String| -> Msg {Msg::LatestPrice(value)})
            .font(FONT)
            .padding(5)
            .width(Length::Units(100));

        let leverage_type = TextInput::
        new(&mut self.leverage_type, "(涨/跌)",
            &mut self.leverage_type_str, move |value : String| -> Msg {Msg::LeverageType(value)})
            .font(FONT)
            .padding(5)
            .width(Length::Units(100));

        let profit = Text::new("盈利：".to_owned() + self.profit.as_str()).font(FONT);
        let error = Text::new(self.error.as_str()).font(FONT);

        let content = Column::new()
            .push(Text::new("填写数据：").font(FONT).size(18))
            .push(Row::new()
                .push(original_money).padding(10)
                .push(leverage).padding(10)
                .push(market_price).padding(10)
                .push(latest_price).padding(10)
                .push(leverage_type).padding(10)
            )
            .push(profit)
            .push(error);

        Container::new(content).height(Length::Fill).height(Length::Fill).into()
    }
}