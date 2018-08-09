# **tokterm** Terminal Toolkit
Tokterm started (and continues) as an exercise to learn Rust and while at it, do some fun ansi-based applications. I found that terminal or console programming can help to keep in focus the functional side more than other types of apps, and this is true as well when learning a new programming language.
The idea is to make a terminal toolkit that allows the user to create simple user interfaces, and ascii games. The toolkit should work on windows, linux and at least other unix.

Once the core library and the specific backends are ready, the idea is to implement a GUI library, and simple game engine.


# Current Status
Currently the windows backend is being develop. Utilizes win32 apis to optimize the writing and input reading, to give much better performance than regular stdout. The core allow to manipulate the window, the terminal console, and keyboard and mouse input. Joystick is not planned in the near future, but if for some strange reason some other devs get interested in this project (are you crazy?) maybe we can add that.

# Future Steps

## Linux Support
After finishing the windows implementation, and maybe adding some tests, the linux support should be implemented to strengthen the interface, and give multi-platform support. Unix-like terminals are way more articulated than their windows counterpart, allowing many more colors and other settings, and there is still some decisions to take regarding to that better qualities. For now we are using the lower denominator as the main dictator of the project interface.


## GUI Toolkit
Having buffer map capabilities plus event driven code, a GUI should be pretty straightforward to make, and great area to explore with Rust new philosophies. The idea is to have a basic extensible toolkit that allow for both creating simple UIs but also extending the work for other purposes.

## Game Engine
Like the previous point, the core engine allow us to create a simple game engine, and use the terminal to create games like roguelikes or other types of games.
