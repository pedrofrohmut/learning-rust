pub struct AverageCollection {
    list: Vec<i32>,
    average: f64
}

impl AverageCollection {
    pub fn add(&mut self, value: i32)
    {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32>
    {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }

    pub fn average(&self) -> f64 { self.average }

    fn update_average(&mut self)
    {
        let total = self.list.iter().sum::<i32>() as f64;
        let num_elements = self.list.len() as f64;
        self.average = total / num_elements;
    }
}

fn main()
{
    println!("Hello, world!");
}
