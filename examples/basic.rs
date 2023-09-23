use flatbox::prelude::*;
use flatbox_doryen::prelude::*;

fn main() {
    Flatbox::init(WindowBuilder::default())
        .default_systems()
        .apply_extension(DoryenExtension)
        .add_setup_system(init)
        .add_system(update)
        .add_render_system(render)
        .run();  
}

fn init(mut console: Write<RawConsole>){
    console.register_color("white", (255, 255, 255, 255));
    console.register_color("red", (255, 92, 92, 255));
    console.register_color("blue", (192, 192, 255, 255));
}

fn update(console: Read<RawConsole>){
    debug!("{:?}", *console)
}

fn render(mut console: Write<RawConsole>) {
    console.area(
        10,
        10,
        5,
        5,
        Some((255, 64, 64, 255)),
        Some((128, 32, 32, 255)),
        Some('&' as u16),
    );
}