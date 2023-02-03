mod character;

#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};
use std::io::{stdout, Write};
enum ProgramScreen {
    Menu,
    Game(GameStatus),
    NewGame,
    LoadGame,
    Options
}

enum GameStatus {
    Tutorial,
    Explore,
    Combat,
    RandEvent,
    StoryEvent
}

fn main() {
    let mut stdo = stdout();
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
                let mut cursor_menu = 0;
                let cursor_x = sang_text_x - sang_curs;
                let cursor_y = sang_title_y + sang_text_y;
                execute!(stdo, 
                    cursor::Hide, 
                    Clear(ClearType::All), 
                    cursor::MoveTo(sang_title_x, sang_title_y), 
                    Print("Astares Dungeon by elver ")
                    //Print(format!("{:?}",size().unwrap()))
                ).unwrap();
                let strs = ["New game", "Load game", "Options", "Quit game"];
                for i in 0..strs.len() {
                    execute!(stdo, 
                        cursor::MoveTo(sang_text_x, cursor_y + i as u16), 
                        Print(strs[i])
                    ).unwrap();
                }
                loop {
                    execute!(stdo, 
                        cursor::MoveTo(cursor_x, cursor_y + cursor_menu), 
                        Print(">"),
                        cursor::MoveTo(cursor_x, cursor_y + cursor_menu)
                    ).unwrap();
                    match read().unwrap() {
                        Event::Key(KeyEvent {code: KeyCode::Up,        modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('w'), modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('8'), modifiers: _ }) => {
                            if cursor_menu > 0{
                                execute!(stdo,  
                                    Print(" ")
                                ).unwrap();
                                cursor_menu -=1 ;
                            }
                        },
                        Event::Key(KeyEvent {code: KeyCode::Down,      modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('s'), modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('2'), modifiers: _ }) => {
                            if cursor_menu + 1 < strs.len() as u16 {
                                execute!(stdo, 
                                    Print(" ")
                                ).unwrap();
                                cursor_menu += 1;
                            }
                        },
                        Event::Key(KeyEvent{code: KeyCode::Enter,     modifiers: _}) |
                        Event::Key(KeyEvent{code: KeyCode::Char(' '), modifiers: _}) => {
                            match cursor_menu {
                                0 => program_screen = ProgramScreen::NewGame,
                                1 => program_screen = ProgramScreen::LoadGame,
                                2 => program_screen = ProgramScreen::Options,
                                3 => break 'main,
                                _ => ()
                            };
                            break
                        },
                        _ => ()
                    };
                }
            },
            ProgramScreen::Game(game_status) => {
                todo!()
            },
            ProgramScreen::NewGame => {
                execute!(stdo, 
                    cursor::Hide, 
                    Clear(ClearType::All), 
                    cursor::MoveTo(1, 1), 
                    Print("game made")
                ).unwrap();
                program_screen = ProgramScreen::Game(GameStatus::Tutorial);
            },
            ProgramScreen::LoadGame => {
                execute!(stdo, 
                    cursor::Hide, 
                    Clear(ClearType::All), 
                    cursor::MoveTo(1, 1), 
                    Print("game load")
                ).unwrap();
                program_screen = ProgramScreen::Game(GameStatus::Explore);
            },
            ProgramScreen::Options => {
                todo!()
            }
        }
    }
    execute!(stdo, 
        cursor::Hide, 
        Clear(ClearType::All), 
        cursor::MoveTo(1, 1), 
        Print("Thanks for playing"),
        cursor::MoveTo(0, 4)
    ).unwrap();
    disable_raw_mode().unwrap();
}
