use std::time::{Duration, Instant};
use tokterm_core::drawing::cell::Cell;
use tokterm_core::drawing::cell_buffer::CellBuffer;
use tokterm_core::drawing::color::Color;
use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::drawing::size_2d::Size2d;
use tokterm_core::events::event::KeyboardEvent;
use tokterm_core::events::event::MouseEvent;
use tokterm_core::events::event::{Event, KeyboardEventType, MouseEventType};
use tokterm_core::system::application::Application;
use tokterm_core::Result;

pub fn execute(application: &mut Application) -> Result<()> {
    {
        ///////////////////////////////////////////////////
        //  NOT IMPLEMENTED ON UNIX
        ///////////////////////////////////////////////////
        //let window = application.get_window();
        //window.set_window_position(Point2d::empty())?;
        //window.set_window_size(Size2d::new(800, 600))?;
        //let mouse = application.get_mouse();
        //mouse.show_cursor(false)?;
        ///////////////////////////////////////////////////

        let terminal = application.get_terminal();
        terminal.clear()?;
        terminal.set_cursor(Point2d::empty())?;
        terminal.set_cursor_visibility(false)?;
    }

    let mut buffer = CellBuffer::new(Cell::new_default('X'), Size2d::empty());
    let mut fps = 0;
    let mut frames = 0;
    let mut duration = Duration::from_micros(0);

    loop {
        let now = Instant::now();
        frames += 1;

        // checks the size and resize the buffer if required.
        buffer.resize(Cell::new('X', Color::Blue, Color::Black), application.get_terminal().get_console_size()?);

        // process native events.
        application.listen_events()?;

        // while there is events in the event queue, process them.
        while let Some(event) = application.get_mut_event_queue().get_event() {
            match event {
                Event::Mouse(mouse) => process_mouse_events(mouse, &mut buffer),
                Event::Keyboard(keyboard) => process_keyboard_events(keyboard, &mut buffer),
                _ => (),
            };
        }

        // checks the app stats and draw them in the stat bar.
        draw_stats(application, &mut buffer, fps)?;

        // blits the buffer onto the terminal console.
        application.get_terminal().write(&buffer)?;

        // checks the frames.
        duration += now.elapsed();

        if duration.as_secs() > 1 {
            duration = Duration::from_micros(0);
            fps = frames;
            frames = 0;
        }
    }
}

fn draw_stats(application: &Application, buffer: &mut CellBuffer, fps: i32) -> Result<()> {
    let text_background = Cell::new(' ', Color::Red, Color::DarkGrey);
    let separator = Cell::new('¯', Color::Blue, Color::Black);
    let terminal = application.get_terminal();
    let console_size = terminal.get_console_size()?;

    buffer.repeat_cell(text_background, Point2d::new(0, 0), console_size.width);
    buffer.repeat_cell(separator, Point2d::new(0, 1), console_size.width);
    buffer.write_str(
        &format!(
            "FPS: {}   Console({}, {})",
            fps,
            console_size.width,
            console_size.height,
        ),
        Point2d::empty(),
        Color::Green,
        Color::Red,
    );

    Ok(())
}

fn process_mouse_events(mouse: MouseEvent, buffer: &mut CellBuffer) {
    if mouse.event_type == MouseEventType::MouseMove || mouse.event_type == MouseEventType::Click {
        if mouse.left_button {
            buffer.set(mouse.position, Cell::new('░', Color::White, Color::Black));
        }

        if mouse.right_button {
            buffer.set(mouse.position, Cell::new_default(' '));
        }
    }

    if mouse.event_type == MouseEventType::HorizontalWheel {
        buffer.write_str(
            &format!("{}", mouse.wheel_delta),
            Point2d::new(0, 2),
            Color::White,
            Color::DarkBlue,
        );
    }
}

fn process_keyboard_events(keyboard: KeyboardEvent, buffer: &mut CellBuffer) {
    let down = if keyboard.event_type == KeyboardEventType::KeyDown {
        "down"
    } else {
        "up"
    };

    buffer.write_str(
        &format!("{:?} {}", keyboard.key, down),
        Point2d::new(0, 2),
        Color::White,
        Color::DarkBlue,
    );

    buffer.write_str(
        if keyboard.left_shift {
            "left shift down"
        } else {
            "left shift up  "
        },
        Point2d::new(0, 3),
        Color::White,
        Color::DarkBlue,
    );

    buffer.write_str(
        if keyboard.left_menu {
            "left alt down"
        } else {
            "left alt up  "
        },
        Point2d::new(0, 4),
        Color::White,
        Color::DarkBlue,
    );

    buffer.write_str(
        if keyboard.left_control {
            "left control down"
        } else {
            "left control up  "
        },
        Point2d::new(0, 5),
        Color::White,
        Color::DarkBlue,
    );

    buffer.write_str(
        if keyboard.right_shift {
            "right shift down"
        } else {
            "right shift up  "
        },
        Point2d::new(20, 3),
        Color::White,
        Color::DarkBlue,
    );

    buffer.write_str(
        if keyboard.right_menu {
            "right alt down"
        } else {
            "right alt up  "
        },
        Point2d::new(20, 4),
        Color::White,
        Color::DarkBlue,
    );

    buffer.write_str(
        if keyboard.right_control {
            "right control down"
        } else {
            "right control up  "
        },
        Point2d::new(20, 5),
        Color::White,
        Color::DarkBlue,
    );
}
