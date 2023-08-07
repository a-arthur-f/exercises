use std::ops::RangeInclusive;

pub struct Table {
    multiplier: f32,
    table: Vec<f32>,
}

impl Table {
    pub fn gen(multiplier: f32, range: RangeInclusive<usize>) -> Table {
        let mut table: Vec<f32> = vec![];

        for multiplicand in range {
            table.push(multiplier * multiplicand as f32);
        }

        Table { multiplier, table }
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for (index, product) in self.table.iter().enumerate() {
            string.push_str(format!("{} x {} = {}\n", self.multiplier, index, product).as_str());
        }

        write!(f, "{}", string)
    }
}
