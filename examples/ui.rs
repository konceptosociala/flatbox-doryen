use sonja::prelude::*;
use sonja_doryen::prelude::*;

const TEXT_COLOR: Color = (0, 0, 0, 255);
const BG_COLOR: Color = (200, 200, 200, 255);

fn main() {
    Sonja::init(WindowBuilder::default())
        .default_systems()
        .apply_extension(DoryenExtension)
        .add_render_system(ui_system)
        .run();  
}

fn ui_system(mut ctx: Write<GuiContext>) {
    gui::SidePanel::right()
        .color(BG_COLOR)
        .show(&mut ctx, |ui| 
    {
        ui.add(
            gui::Label::new("Hello")
                .align(gui::TextAlign::Left)
                .fore_color(TEXT_COLOR)
                .back_color(BG_COLOR)
        );

        ui.add(
            gui::Label::new("pretty")
                .align(gui::TextAlign::Center)
                .fore_color(TEXT_COLOR)
                .back_color(BG_COLOR)
        );

        ui.add(
            gui::Label::new("World")
                .align(gui::TextAlign::Right)
                .fore_color(TEXT_COLOR)
                .back_color(BG_COLOR)
        );
    });
}