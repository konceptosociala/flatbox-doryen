// use despero::{WindowBuilder, Despero};
// use doryen_rs::{AppOptions, Engine, DoryenApi, UpdateEvent};

// impl From<WindowBuilderWrapper> for AppOptions {
//     fn from(source: WindowBuilderWrapper) -> Self {
//         AppOptions::default()
//     }
// }

// #[derive(Default, Clone, Debug)]
// pub struct WindowBuilderWrapper {
//     pub inner: WindowBuilder,
// }

// impl WindowBuilderWrapper {
//     pub fn new() -> Self {
//         WindowBuilderWrapper::default()
//     }
// }

// impl From<WindowBuilder> for WindowBuilderWrapper {
//     fn from(source: WindowBuilder) -> Self {
//         WindowBuilderWrapper { inner: source }
//     }
// }

// impl Engine for Despero {
//     fn init(&mut self, api: &mut dyn DoryenApi) {
//         api.con().register_color("white", (255, 255, 255, 255));
//         api.con().register_color("red", (255, 92, 92, 255));
//         api.con().register_color("blue", (192, 192, 255, 255));
//     }
//     fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
//         let input = api.input();
//         if input.key("Left") {
//             self.player_pos.0 = (self.player_pos.0 - 1).max(1);
//         } else if input.key("Right") {
//             self.player_pos.0 = (self.player_pos.0 + 1).min(CONSOLE_WIDTH as i32 - 2);
//         }
//         if input.key("Up") {
//             self.player_pos.1 = (self.player_pos.1 - 1).max(1);
//         } else if input.key("Down") {
//             self.player_pos.1 = (self.player_pos.1 + 1).min(CONSOLE_HEIGHT as i32 - 2);
//         }
//         self.mouse_pos = input.mouse_pos();

//         // capture the screen
//         if input.key("LCtrl") && input.key_pressed("S") {
//             self.screenshot_idx += 1;
//             return Some(UpdateEvent::Capture(format!(
//                 "screenshot_{:03}.png",
//                 self.screenshot_idx
//             )));
//         }

//         None
//     }
//     fn render(&mut self, api: &mut dyn DoryenApi) {
//         let con = api.con();
//         con.rectangle(
//             0,
//             0,
//             CONSOLE_WIDTH,
//             CONSOLE_HEIGHT,
//             Some((128, 128, 128, 255)),
//             Some((0, 0, 0, 255)),
//             Some('.' as u16),
//         );
//         con.area(
//             10,
//             10,
//             5,
//             5,
//             Some((255, 64, 64, 255)),
//             Some((128, 32, 32, 255)),
//             Some('&' as u16),
//         );
//         con.ascii(self.player_pos.0, self.player_pos.1, '@' as u16);
//         con.fore(self.player_pos.0, self.player_pos.1, (255, 255, 255, 255));
//         con.print_color(
//             (CONSOLE_WIDTH / 2) as i32,
//             (CONSOLE_HEIGHT - 1) as i32,
//             "#[red]arrows#[white] : move - #[red]CTRL-S#[white] : save screenshot",
//             TextAlign::Center,
//             None,
//         );
//         con.print_color(
//             (CONSOLE_WIDTH / 2) as i32,
//             (CONSOLE_HEIGHT - 3) as i32,
//             &format!(
//                 "#[white]Mouse coordinates: #[red]{}, {}",
//                 self.mouse_pos.0, self.mouse_pos.1
//             ),
//             TextAlign::Center,
//             None,
//         );
//         con.print_color(
//             5,
//             5,
//             "#[blue]This blue text contains a #[red]red#[] word",
//             TextAlign::Left,
//             None,
//         );
//         con.back(
//             self.mouse_pos.0 as i32,
//             self.mouse_pos.1 as i32,
//             (255, 255, 255, 255),
//         );
//     }
// }