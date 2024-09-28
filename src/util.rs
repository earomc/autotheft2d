use macroquad::prelude::*;

#[derive(Debug)]
pub struct SpiralIterator {
    direction: (isize, isize), // Current movement direction
    steps_remaining: usize,    // Steps remaining in the current direction
    layer: usize,              // Current layer of the spiral
    position: (isize, isize),  // Current position in the grid
    steps_in_layer: usize,     // Number of steps taken in the current layer
}

impl SpiralIterator {
    pub fn new(start: (usize, usize)) -> Self {
        SpiralIterator {
            direction: (1, 0),      // Start moving right
            steps_remaining: 1,     // Initial step count in the first direction
            layer: 1,               // Start with the first layer
            position: (start.0 as isize, start.1 as isize), // Start position
            steps_in_layer: 0,      // Steps taken in the current layer
        }
    }

    // Change direction: Right -> Down -> Left -> Up
    fn rotate_direction(&mut self) {
        self.direction = match self.direction {
            (1, 0)  => (0, 1),   // Right -> Down
            (0, 1)  => (-1, 0),  // Down -> Left
            (-1, 0) => (0, -1),  // Left -> Up
            (0, -1) => (1, 0),   // Up -> Right
            _ => (1, 0),         // Default to right (this should not happen)
        };
    }
}

impl Iterator for SpiralIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps_remaining == 0 {
            // Rotate the direction once we've used all steps in the current direction
            self.rotate_direction();
            
            // After every two direction changes, increase the steps for the next layer
            self.steps_in_layer += 1;
            if self.steps_in_layer % 2 == 0 {
                self.layer += 1;
            }
            
            // Set the number of steps to match the current layer size
            self.steps_remaining = self.layer;
        }

        // Update the current position by moving in the current direction
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;

        // Convert the current position back to `usize` for output, if the position is valid
        if self.position.0 >= 0 && self.position.1 >= 0 {
            let current_position = (self.position.0 as usize, self.position.1 as usize);

            // Decrease the remaining steps in the current direction
            self.steps_remaining -= 1;

            Some(current_position)
        } else {
            // If the position goes negative, we return None (or handle it differently depending on requirements)
            None
        }
    }
}

pub fn mouse_direction() -> Vec2 {
    let mouse_local = mouse_position_local();
    Vec2::new(mouse_local.x * screen_width(), mouse_local.y * screen_height()).normalize()
}