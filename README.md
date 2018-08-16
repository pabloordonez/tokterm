# tokterm - Terminal Toolkit
Tokterm started (and continues) as an exercise to learn Rust and while at it, do some fun ansi-based applications. I found that terminal or console programming can help to keep in focus the functional side more than other types of apps, and this is true as well when learning a new programming language.
The idea is to make a terminal toolkit that allows the user to create simple user interfaces, and ascii games. The toolkit should work on windows, linux and at least other unix.

Once the core library and the specific backends are ready, the idea is to implement a GUI library, and simple game engine.


# Current Status
Currently there are 3 backends with different degree of completion:
- Windows
- NCurses
- Termion

The most advanced backend is the window implementation, having control over the windows and much better input support. NCurses and Termion has limited input support and no window manipulation capabilities.

# Future Steps

##  Finishing Unix Backends
There still work to be done in Termion and NCurses implementations, but the work is to tedious and slow, so is being made at a slower pace.

## Canvas Support
The basic idea of treating console terminals are a normal bitmap, is to be able to do canvas drawing over the console with ascii chars and colors. The toolkit should allow for drawing methods like lines, circles, rectangles, etc.

## GUI Toolkit
Having buffer map capabilities plus event driven code, a GUI should be pretty straightforward to make, and great area to explore with Rust new philosophies. The idea is to have a basic extensible toolkit that allow for both creating simple UIs but also extending the work for other purposes.

## Game Engine
Like the previous point, the core engine allow us to create a simple game engine, and use the terminal to create games like roguelikes or other types of games.
