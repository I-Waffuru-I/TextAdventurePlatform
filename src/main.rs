mod pkg;

use pkg::general::game_manager::GameManager;


fn main()
{
    let mut game_manager : GameManager = GameManager::new();
    game_manager.run();
}
