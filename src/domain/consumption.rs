pub struct Consumption {
    pub item: String,
    pub unit_price: u32,
    pub period_days: u32,
    pub frequency_per_period: u32,
    pub total_days: u32,
}

impl Consumption {
    pub fn yearly_total(&self) -> u32 {
        let periods = self.total_days / self.period_days;
        self.unit_price * self.frequency_per_period * periods
    }
}
