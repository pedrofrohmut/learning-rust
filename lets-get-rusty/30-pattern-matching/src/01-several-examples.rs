#![allow(dead_code)]
#![allow(unused_variables)]

fn main()
{
    // ========================================================================
    //                          Match Expressions
    //
    //   When using the match expressions you must always cover all cases. But
    // you can use the _ to match everything else or even give it a name
    // ========================================================================
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
        Thai,
    }

    let language = Language::English;

    match language {
        Language::English  => println!("Hello, World!"),
        Language::Spanish  => println!("Hola, Mundo!"),
        Language::Russian  => println!("Привет, мир!"),
        Language::Japanese => println!("こんにちは世界！"),
        other             => println!("Unsupported language! {:?}", other)
    }

    // ========================================================================
    //                  Conditionals if let Expressions
    //
    //   If-let expression is when you want to match a variable but you only
    //  care about one case
    //
    //   The downside of if-let expressions is that the compiler does not
    // enforce that the code exaustive match all cases
    // ========================================================================
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }

    // ========================================================================
    //                  While let conditional loops
    //
    //   Runs the loop while the pattern specified continues to match
    // ========================================================================
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // ========================================================================
    //                              for loops
    //
    //   Iter + Enumerate returns a Enumeration tuple with the (index, value)
    // and you can use the pattern matching to destruct the returned tuple
    // ========================================================================
    let vec = vec!['a', 'b', 'c'];

    for (index, value) in vec.iter().enumerate() {
        println!("[{}] = {}", index, value);
    }

    // ========================================================================
    //                            let Statements
    //
    //   Pattern matching can be use in let statements to destruct tuples
    // in variables
    //
    //   You can use underscores to ignore some values too like 'let (x, y, _)'
    // ========================================================================
    let x = 5;

    // let PATTERN = EXPRESSION

    let (x, y, z) = (1, 2, 3);

    // ========================================================================
    //                        Function Parameters
    // ========================================================================
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32))
{
    println!("Current location: ({}, {})", x, y);
}
