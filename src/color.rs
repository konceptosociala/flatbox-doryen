use std::collections::HashMap;

pub use doryen_rs::Color;

#[derive(Default, Debug, Clone)]
pub struct RegisterColors {
    pub colors: HashMap<&'static str, Color>,
}

#[macro_export]
macro_rules! register_colors {
    {
        $([ $name:expr, $color:expr ]),+
    } => {
        {
            let mut colors = RegisterColors::default();
            $(
                colors.colors.insert($name, $color);
            )+
            colors
        }
    };
}