pub trait Draw {
    fn draw(&self);
}

// 1. Use Generics (T: Draw) for Screen when you dont need to be flexible since
// the performance is better then using Box<dyn Draw>
//
// 2. With Box<dyn Draw> you can store different type of instances that implement
// the Draw trait in the same collection. Like Buttons, TextFiels, SelectBoxes, etc
// in a single collection
pub struct Screen {
    // dyn Draw: polymorphism to accept any type that implements the Draw trait
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self)
    {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 1. Using generics here makes Screen.components locked to be of a single type
// and not for any type that implements Draw
//
// 2. Using generics Screen needs a different collection for each type. Like
// buttons, select_boxes, text_fields, etc
pub struct GenercisScreen<T> where T: Draw {
    pub components: Vec<T>
}

impl<T> GenercisScreen<T> where T: Draw {
    pub fn run(&self)
    {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {}
}
