



use conrod;
use std;


use conrod::backend::glium::glium;


pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 420;

pub struct YNApp{

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
    pub struct Ids{
        // The main canvas
        canvas,

        //The title, text and buttons
        title,
        text,
        yesbutton,
        nobutton,
    }
}

pub fn ynbox(ui: &mut conrod::UiCell, ids: &Ids, app: &mut YNApp){
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use std::iter::once;


    const MARGIN: conrod::Scalar = 30.0;
    const SHAPE_GAP: conrod::Scalar = 50.0;
    const TITLE_SIZE: conrod::FontSize = 42;
    const SUBTITLE_SIZE: conrod::FontSize = 32;



    //Canvas
    const TITLE: &'static str = "YNBox Title";
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);

    //Title
    widget::Text::new(TITLE).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);

    // Text
    const TEXT: &'static str = "This is the sample text for the ynbox";
    widget::Text::new(TEXT)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.text, ui);

    let button_side = 100.0;
    for _press in widget::Button::new()
        .label("Yesbutton")
        .mid_left_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.text, 60.0)
        .w_h(button_side, button_side)
        .set(ids.yesbutton, ui)
        {
            println!("This is where I would return true!")
        }
    for _press in widget::Button::new()
        .label("Nobutton")
        .mid_right_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.text, 60.0)
        .w_h(button_side, button_side)
        .set(ids.nobutton, ui)
        {
            println!("This is where I would return false!")
        }
    
}