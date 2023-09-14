use crate::gui::{GuiContext, Size, Ui, Widget, WidgetType, Offset};
use crate::utils::{
    console::RawConsole,
    doryen::Color,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Container {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct SidePanel {
    local_ui: Ui,
    side: Side,
    width: Size,
    color: Color,
    with_frame: bool,
}

impl SidePanel {
    pub fn new(side: Side) -> Self {
        SidePanel {
            local_ui: Ui::new_vertical(),
            side,
            width: Size::Absolute(15),
            color: (0, 0, 0, 255),
            with_frame: false,
        }
    }

    pub fn left() -> Self {
        SidePanel::new(Side::Left)
    }

    pub fn right() -> Self {
        SidePanel::new(Side::Right)
    }

    pub fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_frame(mut self, with_frame: bool) -> Self {
        self.with_frame = with_frame;
        self
    }

    pub fn show(
        mut self,
        ctx: &mut GuiContext,
        inner_content: impl Fn(&mut Ui),
    ){
        inner_content(&mut self.local_ui);

        let screen = ctx.get_screen_size();
        self.local_ui.set_container(self.to_container(screen));

        ctx.ui.add(self);
    }
}

impl Widget for SidePanel {
    fn render(&self, console: &mut RawConsole, ctx: &GuiContext, _: Offset) {
        let container = self.to_container(ctx.get_screen_size());

        console.area(
            container.x as i32,
            container.y as i32,
            container.w,
            container.h,
            Some(self.color),
            Some(self.color),
            Some(' ' as u16),
        );
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Static
    }

    fn to_container(&self, screen: Container) -> Container {
        let h = screen.h;
        let w = self.width.calc_absolute(screen.w).get().unwrap();

        let (x, y) = match self.side {
            Side::Left => (0, 0),
            Side::Right => (screen.w - w + 1, screen.h - h + 1),
        };

        Container { x, y, w, h }
    }

    fn local_ui(&self) -> Option<&Ui> {
        Some(&self.local_ui)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopBottomSide {
    Top,
    Bottom,
}

pub struct TopBottomPanel {
    pub(crate) side: TopBottomSide,
    pub(crate) height: Size,
    pub(crate) color: Color,
}

impl TopBottomPanel {
    pub fn new(side: TopBottomSide) -> Self {
        TopBottomPanel {
            side,
            height: Size::Absolute(2),
            color: (200, 200, 200, 255),
        }
    }

    pub fn bottom() -> Self {
        TopBottomPanel::new(TopBottomSide::Bottom)
    }

    pub fn top() -> Self {
        TopBottomPanel::new(TopBottomSide::Top)
    }

    pub fn with_height(mut self, height: Size) -> Self {
        self.height = height;
        self
    }

}

impl Widget for TopBottomPanel {
    fn render(&self, console: &mut RawConsole, ctx: &GuiContext, _: Offset) {
        todo!();
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Static
    }

    fn to_container(&self, screen: Container) -> Container {
        // let h = screen.h;
        // let w = self.width.calc_absolute(screen.w).get().unwrap();

        // let (x, y) = match self.side {
        //     TopBottomSide::Top => (0, 0),
        //     TopBottomSide::Bottom => (screen.w - w, screen.h - h),
        // };

        // Container { x, y, w, h }
        todo!();
    }

    fn local_ui(&self) -> Option<&Ui> {
        todo!();
    }
}