use sonja::prelude::*;
use sonja_doryen::prelude::*;

fn main() {
    Sonja::init(WindowBuilder::default())
        .default_systems()
        .apply_extension(DoryenExtension)
        .add_setup_system(init)
        .run();  
}

fn init(
    events: Read<Events>,
    mut cmd: Write<CommandBuffer>
){
    let mut color_events = events.get_handler::<RegisterColors>().unwrap();

    color_events.send(register_colors! {
        ["white", (255, 255, 255, 255)],
        ["red", (255, 92, 92, 255)],
        ["blue", (192, 192, 255, 255)]
    });
}