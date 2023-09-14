use std::str::FromStr;
use std::num::{ParseIntError, ParseFloatError};
use thiserror::Error;

pub type Cells = u32;
pub type Percent = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Absolute(Cells),
    Relative(Percent)
}

impl Size {
    pub fn new(s: &str) -> Result<Self, ParseSizeError> {
        Size::from_str(s)
    }

    pub fn get(&self) -> Option<u32> {
        match self {
            Size::Absolute(s) => Some(*s),
            _ => None,
        }
    }

    pub fn absolute(size: Cells) -> Self {
        Size::Absolute(size)
    }

    pub fn relative(size: Percent) -> Self {
        Size::Relative(size)
    }

    pub fn calc_absolute(&self, container_size: u32) -> Self {
        match self {
            Size::Absolute(_) => self.clone(),
            Size::Relative(value) => {
                Size::Absolute(((container_size as f32) * value / 100.0) as u32)
            }
        }
    }

    pub fn calc_relative(&self, container_size: u32) -> Self {
        match self {
            Size::Relative(_) => self.clone(),
            Size::Absolute(value) => {
                Size::Relative((*value as f32) / (container_size as f32) * 100.0)
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseSizeError {
    #[error("Cannot parse absolute size")]
    AbsoluteSize(#[from] ParseIntError),
    #[error("Cannot parse relative size")]
    RelativeSize(#[from] ParseFloatError),
    #[error("Invalid expression: `{0}`")]
    InvalidExpression(String),
}

impl FromStr for Size {
    type Err = ParseSizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((v, _)) = s.split_once("px") {
            Ok(Self::Absolute(v.parse::<u32>()?))
        } else if let Some((v, _)) = s.split_once("%") {
            Ok(Self::Relative(v.parse::<f32>()?))
        } else {
            Err(ParseSizeError::InvalidExpression(s.to_owned()))
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Offset {
    pub x: u32,
    pub y: u32,
}