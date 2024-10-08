use debugger::Debugger;
use error::EmulationError;
use gameboy::Gameboy;
use input::{handle_input, GBInputState};
use pollster::FutureExt;

#[allow(dead_code)]
mod cpu;
mod debugger;
#[allow(dead_code)]
mod decoding;
mod error;
mod gameboy;
mod input;
#[allow(non_contiguous_range_endpoints)]
mod memory;
#[allow(dead_code)]
mod renderer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage : gbemulator <rom file>");
        return;
    }
    env_logger::init();

    if let Err(e) = run(args) {
        println!("Error : {e}");
    }
}

fn run(args: Vec<String>) -> Result<(), EmulationError> {
    let rom = std::fs::read(&args[1]).unwrap();
    let mut console = Gameboy::new(rom);

    let flag_paused = args.iter().any(|a| a.eq("-p"));
    let mut debugger = Debugger::new(flag_paused);

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(
            640, // this is x4 the gameboy's resolution
            576,
            "Koholint Gameboy Emulator",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    let mut renderer = renderer::Renderer::new(&mut window).block_on();

    //
    let mut dots = 0;
    const DOTS_IN_FRAME: u64 = 70224;
    let mut frame_start = std::time::Instant::now();
    let mut input = GBInputState::default();
    while !renderer.window().should_close() {
        handle_input(&mut glfw, &mut renderer, &events, &mut debugger, &mut input);
        while dots < DOTS_IN_FRAME {
            dots += debugger.step(&mut console)?;
            console.update_input(&input);
        }
        dots = 0;

        // FIXME : rendering one big frame at 60hz is not accurate enough :
        // many games modify stuff mid-frame to create effects
        // for good accuracy, the frame needs to be drawn line-by-line

        renderer.render(&console).unwrap();

        while frame_start.elapsed().as_millis() < 16 {}
        frame_start = std::time::Instant::now();

        /* if fps_start.elapsed().as_millis() >= 1000 {
            println!("FPS : {frames}");
            frames = 0;
            fps_start = std::time::Instant::now();
        } */
    }
    Ok(())
}
