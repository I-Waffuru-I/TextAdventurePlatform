mod pkg;

use pkg::general::current_game_state::CurrentGameState;
use pkg::general::game_manager::GameManager;


fn main()
{
    let mut game_manager : GameManager = GameManager::new();
    let mut running: bool = true;
    while running {
        run(&mut game_manager, &mut running);
    }

}

fn run(game_manager : &mut GameManager, prog_is_running : &mut bool)  
{
    match game_manager.get_current_game_state() {
        CurrentGameState::GameIsStopping => {
            game_manager.stop_program();
            *prog_is_running = false;
        }
        CurrentGameState::GameStarting => {
            game_manager.start_game();
        }
        CurrentGameState::GameRunning => {
            game_manager.continue_playing_game();
        }
        CurrentGameState::AwaitingKeyPress => {
            game_manager.await_key_press();    
        }
        CurrentGameState::MenuOpen => {
            game_manager.open_menu();
        }
        CurrentGameState::StoryIsDone => {
            game_manager.await_key_press();
            game_manager.open_menu();
        }
    }
}
