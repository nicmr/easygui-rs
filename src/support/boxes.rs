use conrod;
use std;


use conrod::backend::glium::glium;


pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 420;

const MARGIN: conrod::Scalar = 30.0;
const SHAPE_GAP: conrod::Scalar = 50.0;
const TITLE_SIZE: conrod::FontSize = 42;
const SUBTITLE_SIZE: conrod::FontSize = 32;

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

/// Marker trait for functions in lib.rs
pub trait ConrodIds{

}
/// Contains text to be displayed in a corresponding Widget
pub trait TextContainer{
    fn title(&self) -> &str;
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

pub fn ynbox(ui: &mut conrod::UiCell, ids: &YNIds, app: &mut EmptyApp, text_container: &YNTextContainer) -> Option<bool>{
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    //use std::iter::once;


    



    //Canvas
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);

    //Title
    //const TITLE: &'static str = "YNBox Title"; //conrod example style
    widget::Text::new(&text_container.title).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);

    // Text
    // const TEXT: &'static str = "This is the sample text for the ynbox"; //cornod example style
    
    widget::Text::new(&text_container.text)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.text, ui);

    let button_side = 100.0;
    for _press in widget::Button::new()
        .label(&text_container.yesbutton)
        .mid_left_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.text, 60.0)
        .w_h(button_side, button_side)
        .set(ids.yesbutton, ui)
        {
            return Some(true);
        }
    for _press in widget::Button::new()
        .label(&text_container.nobutton)
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

pub fn msgbox(ui: &mut conrod::UiCell, ids: &MsgIds, app: &mut EmptyApp, text_container: &MsgTextContainer) -> Option<bool>{
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    //use std::iter::once;

    //Canvas
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);

    //Title
    //const TITLE: &'static str = "YNBox Title"; //conrod example style
    widget::Text::new(&text_container.title).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);

    // Text
    // const TEXT: &'static str = "This is the sample text for the ynbox"; //cornod example style
    
    widget::Text::new(&text_container.text)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.text, ui);

    let button_side = 100.0;
    for _press in widget::Button::new()
        .label(&text_container.okbutton)
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

widget_ids! {
    pub struct ListIds{
        canvas,
        title,
        text,
        list_select,
        confirm_button,
    }
}
impl ConrodIds for ListIds{

}

/// TODO:
/// Maybe the list items should not become part of this struct so we can work with a reference of the vec,
/// to ensure it is not mutated while the choice window is open(e.g., in a different thread)
/// and the returned usize index will be guaranteed to be valid
pub struct ListTextContainer{
    pub title: String,
    pub text: String,
    pub list_items: Vec<String>,
}
impl TextContainer for ListTextContainer{
    fn title(&self) -> &str {
        &self.title
    }
}
impl ListTextContainer{
    fn from_strs(title: &str, text: &str, list_items: &Vec<String>) -> ListTextContainer{
        ListTextContainer{ title: String::from(title), text: String::from(text), list_items: list_items.clone()}
    }
}

/// Determines whether only a single or multiple elements are selectable
/// A third enum variation is going to be added, allowing up to count(usize) to be added
/// Probably needs to be exposed in library interface
pub enum SelectType{
    Multiple, Single
}
pub fn listbox(ui: &mut conrod::UiCell, ids: &ListIds, app: &mut EmptyApp, text_container: &ListTextContainer, select_type: SelectType) -> Option<usize>{
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    //Canvas
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);

    //Title
    widget::Text::new(&text_container.title).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);

    //Text
    widget::Text::new(&text_container.text)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.text, ui);
    
    let num_items = text_container.list_items.len();

    //Turn into constants instead
    let item_h = 30.0;
    let font_size = item_h as conrod::FontSize / 2;

    let list_selected = std::collections::HashSet::new();

    
    //TODO: Turn this into generics instead
    let (mut events, scrollbar) = match select_type{
        SelectType::Multiple => {
            widget::ListSelect::multiple(num_items)
                .flow_down()
                .item_size(item_h)
                .scrollbar_next_to()
                .w_h(400.0, 230.0)
                .top_left_with_margins_on(ids.canvas, 40.0, 40.0)
                .set(ids.list_select, ui)
        }
        SelectType::Single => {
            widget::ListSelect::single(num_items)
                .flow_down()
                .item_size(item_h)
                .scrollbar_next_to()
                .w_h(400.0, 230.0)
                .top_left_with_margins_on(ids.canvas, 40.0, 40.0)
                .set(ids.list_select, ui)
        }
    };
    while let Some(event) = events.next(ui, |i| list_selected.contains(&i)) {
        use conrod::widget::list_select::Event;
        match event {

            // For the `Item` events we instantiate the `List`'s items.
            Event::Item(item) => {
                let label = &text_container.list_items[item.i];
                let (color, label_color) = match list_selected.contains(&item.i) {
                    true => (conrod::color::LIGHT_BLUE, conrod::color::YELLOW),
                    false => (conrod::color::LIGHT_GREY, conrod::color::BLACK),
                };
                let button = widget::Button::new()
                    .color(color)
                    .label(label)
                    .label_font_size(font_size)
                    .label_color(label_color);
                item.set(button, ui);
            },

            // The selection has changed.
            Event::Selection(selection) => {
                selection.update_index_set(&mut list_selected);
                println!("selected indices: {:?}", list_selected);
            },

            // The remaining events indicate interactions with the `ListSelect` widget.
            event => println!("{:?}", &event),
        }
    }
    if let Some(s) = scrollbar { s.set(ui)}

    Some(1)
}


