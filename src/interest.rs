pub struct Interest {
    initial_capital: f32,
    interest_rate: f32,
    investment_time: u32,
}

impl Interest {
    pub fn new(initial_capital: f32, interest_rate: f32, investment_time: u32) -> Interest {
        Interest {
            initial_capital,
            interest_rate,
            investment_time,
        }
    }

    pub fn final_value(&self) -> f32 {
        self.initial_capital + self.total_interest()
    }

    pub fn total_interest(&self) -> f32 {
        self.initial_capital * self.interest_rate * self.investment_time as f32
    }
}
