use conrod;
use std;


use conrod::backend::glium::glium;


pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 420;

pub struct EmptyApp{

}

pub fn theme() -> conrod::Theme{
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme{
        name: String::from("ynbox"),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}



widget_ids! {
    pub struct YNIds{
        // The main canvasls
        canvas,

        //The title, text and buttons
        title,
        text,
        yesbutton,
        nobutton,
    }
}

// #[derive(Debug, Clone, Copy)]
// pub struct YNTextContainer<'a> {
//     pub title: &'a str,
//     pub text: &'a str,
//     pub yesbutton: &'a str,
//     pub nobutton: &'a str,
// }
#[derive(Debug, Clone)]
pub struct YNTextContainer{
    pub title: String,
    pub text: String,
    pub yesbutton: String,
    pub nobutton: String,
}
impl YNTextContainer{
    pub fn from_strs(title: &str, text: &str, yesbutton: &str, nobutton: &str) -> YNTextContainer{
        YNTextContainer::from_strings(title.to_owned(), text.to_owned(), yesbutton.to_owned(), nobutton.to_owned())
    }
    pub fn from_strings(title: String, text: String, yesbutton: String, nobutton: String) -> YNTextContainer{
        YNTextContainer{title, text, yesbutton, nobutton}
    }
}
impl TextContainer for YNTextContainer{
    fn title(&self) -> &str{
        &self.title
    }
}

pub fn ynbox(ui: &mut conrod::UiCell, ids: &YNIds, app: &mut EmptyApp, settings: &YNTextContainer) -> Option<bool>{
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    //use std::iter::once;


    const MARGIN: conrod::Scalar = 30.0;
    const SHAPE_GAP: conrod::Scalar = 50.0;
    const TITLE_SIZE: conrod::FontSize = 42;
    const SUBTITLE_SIZE: conrod::FontSize = 32;



    //Canvas
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);

    //Title
    //const TITLE: &'static str = "YNBox Title"; //conrod example style
    widget::Text::new(&settings.title).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);

    // Text
    // const TEXT: &'static str = "This is the sample text for the ynbox"; //cornod example style
    
    widget::Text::new(&settings.text)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.text, ui);

    let button_side = 100.0;
    for _press in widget::Button::new()
        .label(&settings.yesbutton)
        .mid_left_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.text, 60.0)
        .w_h(button_side, button_side)
        .set(ids.yesbutton, ui)
        {
            return Some(true);
        }
    for _press in widget::Button::new()
        .label(&settings.nobutton)
        .mid_right_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.text, 60.0)
        .w_h(button_side, button_side)
        .set(ids.nobutton, ui)
        {
            return Some(false);
        }

    None 
}



widget_ids! {
    pub struct MsgIds{
        // The main canvasls
        canvas,

        //The title, text and buttons
        title,
        text,
        okbutton,
    }
}
impl ConrodIds for MsgIds{

}

#[derive(Debug, Clone)]
pub struct MsgTextContainer{
    pub title: String,
    pub text: String,
    pub okbutton: String,
}
impl MsgTextContainer{

    // add these constructors to textcontainer traits instead
    pub fn from_strs(title: &str, text: &str, okbutton: &str) -> MsgTextContainer {
        MsgTextContainer{ title: String::from(title), text: String::from(text), okbutton: String::from(okbutton)}
    }
    pub fn from_Strings(title: String, text: String,okbutton: String) -> MsgTextContainer {
        MsgTextContainer{title, text, okbutton}
    }
}
impl TextContainer for MsgTextContainer{
    fn title(&self) -> &str {
        self.title.as_str()
    }
}

pub fn msgbox(ui: &mut conrod::UiCell, ids: &MsgIds, app: &mut EmptyApp, settings: &MsgTextContainer) -> Option<bool>{
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    //use std::iter::once;


    const MARGIN: conrod::Scalar = 30.0;
    const SHAPE_GAP: conrod::Scalar = 50.0;
    const TITLE_SIZE: conrod::FontSize = 42;
    const SUBTITLE_SIZE: conrod::FontSize = 32;



    //Canvas
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);

    //Title
    //const TITLE: &'static str = "YNBox Title"; //conrod example style
    widget::Text::new(&settings.title).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);

    // Text
    // const TEXT: &'static str = "This is the sample text for the ynbox"; //cornod example style
    
    widget::Text::new(&settings.text)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.text, ui);

    let button_side = 100.0;
    for _press in widget::Button::new()
        .label(&settings.okbutton)
        //.mid_left_with_margin_on(ids.canvas, MARGIN)
        .mid_bottom()
        .down_from(ids.text, 60.0)
        .w_h(button_side, button_side)
        .set(ids.okbutton, ui)
        {
            return Some(true);
        }
    None 
}



pub trait ConrodIds{

}
pub trait TextContainer{
    fn title(&self) -> &str;
}
