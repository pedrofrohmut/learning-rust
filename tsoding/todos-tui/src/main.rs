use::ncurses::*;

const DEFAULT_PAIR: i16     = 0;
const HIGHLIGHTED_PAIR: i16 = 1;
const TODOS_INDEX: usize    = 0;
const DONES_INDEX: usize    = 1;

struct AppState<'a> {
    quit: bool,
    todos: Vec<&'a str>,
    dones: Vec<&'a str>,
    curr_list: usize,
    curr_row: usize,
}

fn main()
{
    let mut state = init_state();

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(DEFAULT_PAIR,     COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHTED_PAIR, COLOR_BLACK, COLOR_WHITE);

    while !state.quit {
        for (index, todo) in state.todos.iter().enumerate() {
            let pair = get_pair_for_todos(&state, index);
            attron(COLOR_PAIR(pair));
            mv(index as i32, 0);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }
        for (index, done) in state.dones.iter().enumerate() {
            let pair = get_pair_for_dones(&state, index);
            attron(COLOR_PAIR(pair));
            mv(index as i32, 30);
            addstr(*done);
            attroff(COLOR_PAIR(pair));
        }
        refresh();
        let key_pressed = getch();
        process_key_pressed(&mut state, key_pressed);
    }
    endwin();
}

fn init_state<'a>() -> AppState<'a>
{
    AppState {
        quit: false,
        todos: vec![
            "Write the todo app",
            "Buy bread",
            "Make a cup of tea",
        ],
        dones: vec![
            "Done 1",
            "Done 2",
            "Done 3",
        ],
        curr_list: TODOS_INDEX,
        curr_row: 0,
    }
}

fn get_pair_for_todos(state: &AppState, index: usize) -> i16
{
    if state.curr_list == TODOS_INDEX && state.curr_row == index {
        HIGHLIGHTED_PAIR
    } else {
        DEFAULT_PAIR
    }
}

fn get_pair_for_dones(state: &AppState, index: usize) -> i16
{
    if state.curr_list == DONES_INDEX && state.curr_row == index {
        HIGHLIGHTED_PAIR
    } else {
        DEFAULT_PAIR
    }
}

fn process_key_pressed(state: &mut AppState, key_pressed: i32)
{
    let key = key_pressed as u8 as char;
    match key {
        'q' => quit(state),
        'j' => down(state),
        'k' => up(state),
        'h' => right(state),
        'l' => left(state),
        _   => {},
    }
}

fn quit(state: &mut AppState) { state.quit = true; }

fn up(state: &mut AppState) { if state.curr_row > 0 { state.curr_row  -= 1; } }

fn down(state: &mut AppState) { if state.curr_row < state.todos.len() - 1 { state.curr_row  += 1; } }

fn right(state: &mut AppState) { if state.curr_list >= 1 { state.curr_list -= 1; } }

fn left(state: &mut AppState) { if state.curr_list < DONES_INDEX { state.curr_list += 1; }  }
