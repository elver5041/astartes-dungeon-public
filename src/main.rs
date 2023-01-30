#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Write};

enum ProgramScreen {
    Menu,
    Game,
    NewGame,
    LoadGame,
    Options,
}



fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    let sang_title_y = 1; //title y
    let sang_title_x = 6; //title x
    let sang_text_y = 2; //text y
    let sang_text_x = 8; //text x
    let sang_curs = 2; //how many chars the cursor goes before 

    let mut program_screen: ProgramScreen = ProgramScreen::Menu;
    
    'main: loop {
        match &program_screen {
            ProgramScreen::Menu => {
                execute!(stdout, cursor::Hide, Clear(ClearType::All)).unwrap();
                execute!(stdout, cursor::MoveTo(sang_title_x, sang_title_y), Print("Astares Dungeon by elver")).unwrap();
                let strs = ["New game", "Load game", "Options", "Quit game"];
                for i in 0..strs.len() {
                    execute!(stdout, cursor::MoveTo(sang_text_x, sang_title_y + sang_text_y + i as u16), Print(strs[i])).unwrap();
                }
                let mut cursor_menu = 0;
                loop {
                    execute!(stdout, cursor::MoveTo(sang_text_x-sang_curs, sang_title_y + sang_text_y + cursor_menu), Print(">")).unwrap();
                    match read().unwrap() {
                        Event::Key(KeyEvent {code: KeyCode::Up, modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('w'), modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('8'), modifiers: _ }) => {
                            execute!(stdout, cursor::MoveTo(sang_text_x-sang_curs, sang_title_y + sang_text_y + cursor_menu), Print(" ")).unwrap();
                            if cursor_menu > 0{
                                cursor_menu-=1;
                            }
                            execute!(stdout, cursor::MoveTo(sang_text_x-sang_curs, sang_title_y + sang_text_y + cursor_menu), Print(">")).unwrap();
                        },
                        Event::Key(KeyEvent {code: KeyCode::Down, modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('s'), modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('2'), modifiers: _ }) => {
                            execute!(stdout, cursor::MoveTo(sang_text_x-sang_curs, sang_title_y + sang_text_y + cursor_menu), Print(" ")).unwrap();
                            if cursor_menu + 1 < strs.len() as u16 {
                                cursor_menu += 1;
                            }
                            execute!(stdout, cursor::MoveTo(sang_text_x-sang_curs, sang_title_y + sang_text_y + cursor_menu), Print(">")).unwrap();
                        },
                        Event::Key(KeyEvent{code: KeyCode::Enter, modifiers: _}) |
                        Event::Key(KeyEvent{code: KeyCode::Char(' '), modifiers: _}) => {
                            match cursor_menu {
                                0 => program_screen = ProgramScreen::NewGame,
                                1 => program_screen = ProgramScreen::LoadGame,
                                2 => program_screen = ProgramScreen::Options,
                                3 => break 'main,
                                _ => (),
                            };
                            break
                        },
                        _ => (),
                    };
                }
            }
            ProgramScreen::Game => {
                todo!()
            }
            ProgramScreen::NewGame => {
                execute!(stdout, cursor::Hide, Clear(ClearType::All)).unwrap();
                execute!(stdout, cursor::MoveTo(1, 1), Print("game made")).unwrap();
                program_screen=ProgramScreen::Game;
            },
            ProgramScreen::LoadGame => {
                execute!(stdout, cursor::Hide, Clear(ClearType::All)).unwrap();
                execute!(stdout, cursor::MoveTo(1, 1), Print("game load")).unwrap();
                program_screen=ProgramScreen::Game;
            },
            ProgramScreen::Options => {
                todo!()
            },
        }
    }
    execute!(stdout, cursor::Hide, Clear(ClearType::All)).unwrap();
    execute!(stdout, cursor::MoveTo(1, 1), Print("Thanks for playing")).unwrap();
    disable_raw_mode().unwrap();
}
