#![allow(unused_variables)]
#![allow(dead_code)]

struct ImportantExcerpt<'a> {
    // When you use references in a struct you must use lifetime anotations
    // to make sure that the struct reference field does not outlive the
    // lifetime of the struct it belongs to
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // By rule 3 you dont need
    fn return_part(&self, announcement: &str) -> &str
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main()
{
    let novel = String::from("Call me Ismael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find ");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
