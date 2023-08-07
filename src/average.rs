pub struct Student {
    grades: Vec<Grade>,
}

pub struct Grade(pub f32);

impl Student {
    pub fn new() -> Student {
        Student { grades: vec![] }
    }

    pub fn new_grade(&mut self, grade: Grade) {
        self.grades.push(grade);
    }

    pub fn get_average(&self) -> f32 {
        let mut total: f32 = 0f32;

        for grade in self.grades.iter() {
            let Grade(grade) = grade;
            total += grade;
        }

        total / self.grades.len() as f32
    }
}
