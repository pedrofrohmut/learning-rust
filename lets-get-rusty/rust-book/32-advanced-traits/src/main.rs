#![allow(dead_code)]
#![allow(unused_variables)]

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self)
    {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self)
    {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self)
    {
        println!("Up!")
    }
}

fn main()
{
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    // just like syntatic sugar for Human::fly(&person);
    person.fly();
}
