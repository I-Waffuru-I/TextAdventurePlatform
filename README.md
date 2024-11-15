# Text Adventure Platform

Yo!
Was going through the Rust Book a few weeks ago and decided to make this little framework for text-based games, from scratch. It's *far* from finished of course, but I thought I'd make this public anyway as a proof of concept.

## Building
As with any Rust project, `cargo` does wonders. If running directly through cargo, you won't need the following step, but if you want to export the binary along with data (for example, to .zip and send to a friend) you'll need to add a few things.

The structure of any exported project is quite simple. Next to the executable, you need a `/data` directory. When booting, the program looks for any *.toml* files in this data directory, so make sure to include it! 
```
parent_dir 
 |--cyoa.exe (binary)
 |--/data/
        |--story_file.toml
        |--story_file2.toml
```

## Creating a story
As mentioned right before, the `/data` directory contains all the information for the game to run. It's irrelevant whether you're using cargo or an executable.
There's two files required per game: one *configuration* and one *data*. Inside `/data`, you can have as many story files as you want.

### Configuration file
The config file must be *.toml*, as mentioned previously. At launch, it gets read into a struct:
```rs
struct Game {
    main_file_path : String, // The path FROM THE EXECUTABLE to find the text file.
    characters : Vec<GameCharacter>,
}

struct GameCharacter {
    full_name : String, // Displayed name in-game
    short_name : String, // Used behind the scenes, a very short (1-2 chars) string.
    c_r : u8, // Three integers for the RGB colour attributed to the character. Their name is painted in this colour.
    c_g : u8,
    c_b : u8,
}
```
Here's an example of very simple config file.
```toml
main_file_path = "data/sample.txt"

[[characters]]
full_name = "Waffuru"
short_name = "w"
c_r = 37
c_g = 150 
c_b = 190

[[characters]]
full_name = "You"
short_name = "u"
c_r = 206
c_g = 115
c_b = 45
```

### Data File
This is the bread and butter of your story. Every line of the file is read into memory and ran through, one per one. By default, it increments by one *but can specified otherwise through commands*.

There's two formats to keep in mind.
1. `id;short_name;text to display`
2. `id;command(params)`

I think it's pretty self-explanatory. The `id` increments over each line and is used to reference other lines in commands. It's not an ideal format, but I'm working on a way to facilitate creating these files, meaning you have to worry much less about these. This might be resolved in the future, but for now, it's a pain to use.

As mentioned earlier, `short_name` is used to reference the character that's speaking and get their colours (and eventually, other details, if required). The idea behind it is that eventually, you'll be able to specify how to display their name in full. If the protagonist just met them, they won't know their name yet and could be shown as '???'.

> The shortname '*n*' is reserved for the narrator and may not be overwritten!

`text_to_display` is, drop dead simple, what the character is saying. I plan on adding parsing and styling functionalities, but again, that's not here yet. Working with `\n\r` to create multi-line text doesn't even work since it's read as a raw string xD

Lastly, `command` and `params` are the ways I implemented to execute actions. Currently, there are three (two of which I plan to combine into one to avoid code repetition):
- ch2 : `ch2(What option?,Opt 1,Opt 2, 20, 30)` : Allows to display a question/header and pick between two options. Depending which is selected, it moves to the provided line.
- ch3 :  `ch3(What option?,Opt 1, Opt 2, Opt3, 20, 30, 40)` : Same thing, but with 3 options.
- mv  : `mv(20)` : Moves the current line index to to the given


## What now?
And with that, you've seen about everything there is to see! As I said, it's a super simple framework that can't handle *immense* stories (because, well, it loads *everything* into memory at once, which is something I know how to optimize but didn't feel like it yet). There's a ton of ugly code, lots of things I want to severely refactor and even some files that aren't even being used, namely `command.rs`. I was originally designing a system to pass along functions instead of handling them with the CommandExecuter directly.

Anyway, enough rambling from me. Thanks for going through this explanation.  
Have fun with it! :)