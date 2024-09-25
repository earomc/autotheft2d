pub struct Gearbox {
    gears: Vec<Gear>,
    current_gear: usize,
}

impl Gearbox {
    pub fn gear_ratio(&self) -> f32 {
        self.current_gear().ratio
    }
    pub fn current_gear(&self) -> &Gear {
        &self.gears[self.current_gear]
    }
    
    pub fn current_gear_num(&self) -> usize {
        self.current_gear
    }
    
    /// Shifts to gear
    /// `Panics` If gear does not exist.
    pub fn shift_to(&mut self, gear: usize) -> Result<(), String> {
        if gear >= self.gears.len() {
            return Err(format!("Gear {} too high. Supporting gears from 0 to {}", gear, self.gears.len() - 1));
        }
        self.current_gear = gear;
        Ok(())
    }
    
    pub fn shift_up(&mut self) -> Result<(), String> {
        self.shift_to(self.current_gear + 1)
    }
    
    pub fn shift_down(&mut self) -> Result<(), String> {
        if self.current_gear == 0 {
            return Err(format!("Error shifting down when already in lowest gear"));
        }
        self.shift_to(self.current_gear - 1)
    }

    pub fn six_step() -> Self {
        Gearbox {
            gears: vec![
                Gear::new(8.),
                Gear::new(2.),
                Gear::new(1.4),
                Gear::new(1.),
                Gear::new(0.8),
                Gear::new(0.6),
            ],
            current_gear: 0,
        }
    }
}

pub struct Gear {
    pub ratio: f32,
}

impl Gear {
    fn new(ratio: f32) -> Self {
        Gear { ratio }
    }
}
