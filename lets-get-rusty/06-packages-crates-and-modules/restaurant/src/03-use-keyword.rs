#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// use rand::Rng;
// use rand::ErrorKind::Transient;
// use rand::CryptoRng;

// Using nested paths to merge all 'rand' use statements in one line
use rand::{Rng, ErrorKind::Transient, CryptoRng};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// Glob operator - bring all public members of io into scope
use std::io::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// pub here make hosting module available to external code
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant4()
{
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1, 101);
}


// Conflicting structs names

// Method 1 - using the package prefix
use std::fmt;
use std::io;

fn function1() -> fmt::Result { Ok(()) }

fn function2() -> io::Result<()> { Ok(()) }

// Method 2 - using aliases
use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result { Ok(()) }

fn function4() -> IoResult<()> { Ok(()) }
