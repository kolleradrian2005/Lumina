use super::{
    elements::{
        align::Align, column::Column, container::Container, gesture_detector::GestureDetector,
        padding::Padding, row::Row, text::Text,
    },
    ui_model_group::UiModelGroup,
};

pub trait GuiElement {
    fn collect_models(&self, max_size: (f32, f32)) -> UiModelGroup;
}

pub enum UiElement {
    Padding(Padding),
    Align(Align),
    Container(Container),
    Text(Text),
    Row(Row),
    Column(Column),
    GestureDetector(GestureDetector),
}

impl From<Padding> for Box<UiElement> {
    fn from(padding: Padding) -> Box<UiElement> {
        Box::new(UiElement::Padding(padding))
    }
}

impl From<Align> for Box<UiElement> {
    fn from(align: Align) -> Box<UiElement> {
        Box::new(UiElement::Align(align))
    }
}

impl From<Container> for Box<UiElement> {
    fn from(container: Container) -> Box<UiElement> {
        Box::new(UiElement::Container(container))
    }
}

impl From<Text> for Box<UiElement> {
    fn from(text: Text) -> Box<UiElement> {
        Box::new(UiElement::Text(text))
    }
}

impl From<Row> for Box<UiElement> {
    fn from(row: Row) -> Box<UiElement> {
        Box::new(UiElement::Row(row))
    }
}

impl From<Column> for Box<UiElement> {
    fn from(column: Column) -> Box<UiElement> {
        Box::new(UiElement::Column(column))
    }
}

impl From<GestureDetector> for Box<UiElement> {
    fn from(gesture_detector: GestureDetector) -> Box<UiElement> {
        Box::new(UiElement::GestureDetector(gesture_detector))
    }
}

// Unwrap functions
impl GuiElement for UiElement {
    fn collect_models(&self, max_size: (f32, f32)) -> UiModelGroup {
        match self {
            UiElement::Padding(padding) => padding.collect_models(max_size),
            UiElement::Align(align) => align.collect_models(max_size),
            UiElement::Container(container) => container.collect_models(max_size),
            UiElement::Text(text) => text.collect_models(max_size),
            UiElement::Row(row) => row.collect_models(max_size),
            UiElement::Column(column) => column.collect_models(max_size),
            UiElement::GestureDetector(gesture_detector) => {
                gesture_detector.collect_models(max_size)
            }
        }
    }
}

// Enums for layout

pub enum MainAxisAlignment {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub enum CrossAxisAlignment {
    Start,
    End,
    Center,
}

pub enum EdgeInsets {
    Zero,
    Each {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    }, // fromLTRB
    All(f32),
    Symmetric {
        vertical: f32,
        horizontal: f32,
    },
}

pub enum Alignment {
    Center,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}
