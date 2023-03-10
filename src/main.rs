mod character;
mod config;

#[macro_use]
extern crate crossterm;

use character::{Character, Gender, CHAR_ATTS};
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
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
                execute!(stdout(), 
                    cursor::Hide, 
                    Clear(ClearType::All), 
                    cursor::MoveTo(sang_title_x, sang_title_y), 
                    Print("Astares Dungeon by elver ")
                    //Print(format!("{:?}",size().unwrap()))
                ).unwrap();
                let strs = ["New game", "Load game", "Options", "Quit game"];
                for i in 0..strs.len() {
                    execute!(stdout(), 
                        cursor::MoveTo(sang_text_x, cursor_y + i as u16), 
                        Print(strs[i])
                    ).unwrap();
                }
                loop {
                    execute!(stdout(), 
                        cursor::MoveTo(cursor_x, cursor_y + cursor_menu), 
                        Print(">"),
                        cursor::MoveTo(cursor_x, cursor_y + cursor_menu)
                    ).unwrap();
                    match read().unwrap() {
                        Event::Key(KeyEvent {code: KeyCode::Up,        modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('w'), modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('8'), modifiers: _ }) => {
                            if cursor_menu > 0{
                                execute!(stdout(),  
                                    Print(" ")
                                ).unwrap();
                                cursor_menu -=1 ;
                            }
                        },
                        Event::Key(KeyEvent {code: KeyCode::Down,      modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('s'), modifiers: _ }) |
                        Event::Key(KeyEvent {code: KeyCode::Char('2'), modifiers: _ }) => {
                            if cursor_menu + 1 < strs.len() as u16 {
                                execute!(stdout(), 
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
                match game_status {
                    GameStatus::Tutorial => {
                        let c = character_edit(None,None);
                        todo!("character made")
                    },
                    GameStatus::Explore => todo!("game not built yet"),
                    GameStatus::Combat => todo!(),
                    GameStatus::RandEvent => todo!(),
                    GameStatus::StoryEvent => todo!(),
                }
            },
            ProgramScreen::NewGame => {
                execute!(stdout(), 
                    Clear(ClearType::All), 
                    cursor::MoveTo(1, 1)
                ).unwrap();
                program_screen = ProgramScreen::Game(GameStatus::Tutorial);
            },
            ProgramScreen::LoadGame => {
                execute!(stdout(), 
                    cursor::Hide, 
                    Clear(ClearType::All), 
                    cursor::MoveTo(1, 1), 
                    Print("game load")
                ).unwrap();
                program_screen = ProgramScreen::Game(GameStatus::Explore);
            },
            ProgramScreen::Options => {
                todo!("no options yet")
            }
        }
    }
    execute!(stdout(), 
        cursor::Hide, 
        Clear(ClearType::All), 
        cursor::MoveTo(1, 1), 
        Print("Thanks for playing"),
        cursor::MoveTo(0, 4)
    ).unwrap();
    disable_raw_mode().unwrap();
}

fn character_edit(c: Option<character::Character>, pp: Option<u16>) -> character::Character {
    let c = match c {
        None => Character::new(),
        Some(a) => a
    };
    let sang_title_y = 1; //title y
    let sang_title_x = 6; //title x
    let sang_text_y = 2; //text y
    let sang_text_x = 8; //text x
    let sang_curs = 2; //how many chars the cursor goes before
    let cursor_x = sang_text_x - sang_curs;
    let cursor_y = sang_title_y + sang_text_y;
    let mut cursor_menu = 0;
    execute!(stdout(), 
        cursor::Hide, 
        Clear(ClearType::All), 
        cursor::MoveTo(sang_title_x, sang_title_y), 
        Print("Character editor")
    ).unwrap();
    for i in 0..CHAR_ATTS.len() {
        execute!(stdout(), 
            cursor::MoveTo(sang_text_x, cursor_y + i as u16), 
            Print(CHAR_ATTS[i]),
            Print(": "),
            Print(c.get_att_value(CHAR_ATTS[i]))
        ).unwrap();
    }
    loop {
        execute!(stdout(), 
            cursor::MoveTo(cursor_x, cursor_y + cursor_menu), 
            Print(">"),
            cursor::MoveTo(cursor_x, cursor_y + cursor_menu)
        ).unwrap();
        match read().unwrap() {
            Event::Key(KeyEvent {code: KeyCode::Up,        modifiers: _ }) |
            Event::Key(KeyEvent {code: KeyCode::Char('w'), modifiers: _ }) |
            Event::Key(KeyEvent {code: KeyCode::Char('8'), modifiers: _ }) => {
                if cursor_menu > 0{
                    execute!(stdout(),  
                        Print(" ")
                    ).unwrap();
                    cursor_menu -=1 ;
                }
            },
            Event::Key(KeyEvent {code: KeyCode::Down,      modifiers: _ }) |
            Event::Key(KeyEvent {code: KeyCode::Char('s'), modifiers: _ }) |
            Event::Key(KeyEvent {code: KeyCode::Char('2'), modifiers: _ }) => {
                if cursor_menu + 1 < CHAR_ATTS.len() as u16 {
                    execute!(stdout(), 
                        Print(" ")
                    ).unwrap();
                    cursor_menu += 1;
                }
            },
            Event::Key(KeyEvent{code: KeyCode::Enter,     modifiers: _}) |
            Event::Key(KeyEvent{code: KeyCode::Char(' '), modifiers: _}) => {
                todo!("choosing not yet implemented");
                break;
            },
            _ => ()
        }
    }
    c
}