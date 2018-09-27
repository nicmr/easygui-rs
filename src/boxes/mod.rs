use conrod;
use std;
use traits::{ConrodIds, TextContainer};
use constants::{TITLE_SIZE, MARGIN};

// use conrod::backend::glium::glium;


/// An app struct is supposed to representing the state of the window. The Empty state struct is to be used a s a placeholder for stateless windows.
pub struct EmptyApp{

}

// TODO: add parameter to set name
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

pub fn ynbox(ui: &mut conrod::UiCell, ids: &YNIds, _app: &mut EmptyApp, text_container: &YNTextContainer) -> Option<bool>{
    
    use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
    // use conrod::Colorable //unused so far


    



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
    pub fn from_strings(title: String, text: String,okbutton: String) -> MsgTextContainer {
        MsgTextContainer{title, text, okbutton}
    }
}
impl TextContainer for MsgTextContainer{
    fn title(&self) -> &str {
        self.title.as_str()
    }
}

pub fn msgbox(ui: &mut conrod::UiCell, ids: &MsgIds, _app: &mut EmptyApp, text_container: &MsgTextContainer) -> Option<bool>{
    use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
    // use conrod::Colorable //unused so far

    //Canvas
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);

    //Title
    widget::Text::new(&text_container.title).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);

    // Text   
    widget::Text::new(&text_container.text)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.text, ui);

    //Ok button
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
    pub fn from_strs(title: &str, text: &str, list_items: &Vec<String>) -> ListTextContainer{
        ListTextContainer{ title: String::from(title), text: String::from(text), list_items: list_items.clone()}
    }
}

pub fn listbox_single(ui: &mut conrod::UiCell, ids: &ListIds, _app: &mut EmptyApp, text_container: &ListTextContainer) -> Option<usize>{

    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use conrod::widget::list_select;
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

    //TODO: Turn these into constants instead
    let item_h = 30.0;
    let font_size = item_h as conrod::FontSize / 2;

    let mut list_selected = std::collections::HashSet::new();

    //Multiple select list
    let (mut events, scrollbar) = list_select::ListSelect::multiple(num_items)
        .flow_down()
        .item_size(item_h)
        .scrollbar_next_to()
        .w_h(400.0, 230.0)
        //.top_left_with_margins_on(ids.canvas, 40.0, 40.0)
        .down_from(ids.text, 60.0)
        .set(ids.list_select, ui);
    
    //Confirm button
    let button_side = 100.0;
    for _press in widget::Button::new()
        .label("Confirm")
        //.mid_left_with_margin_on(ids.canvas, MARGIN)
        .mid_bottom()
        .right_from(ids.list_select, 60.0)
        .down_from(ids.text, 60.0)
        .w_h(button_side, button_side)
        .set(ids.confirm_button, ui)
        {
            return Some(1);
        }


    
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
    if let Some(s) = scrollbar { s.set(ui)};
    None
}

pub fn listbox_multiple(){
    use find_folder;
    use support;
    //use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::Surface;
    //implementation from conrod

    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 300;


    widget_ids! {
        struct Ids { canvas, list_select }
    }

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("ListSelect Demo")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // Construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // A unique identifier for each widget.
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // List of entries to display. They should implement the Display trait.
    let list_items = [
        "African Sideneck Turtle".to_string(),
        "Alligator Snapping Turtle".to_string(),
        "Common Snapping Turtle".to_string(),
        "Indian Peacock Softshelled Turtle".to_string(),
        "Eastern River Cooter".to_string(),
        "Eastern Snake Necked Turtle".to_string(),
        "Diamond Terrapin".to_string(),
        "Indian Peacock Softshelled Turtle".to_string(),
        "Musk Turtle".to_string(),
        "Reeves Turtle".to_string(),
        "Eastern Spiny Softshell Turtle".to_string(),
        "Red Ear Slider Turtle".to_string(),
        "Indian Tent Turtle".to_string(),
        "Mud Turtle".to_string(),
        "Painted Turtle".to_string(),
        "Spotted Turtle".to_string()
    ];

    // List of selections, should be same length as list of entries. Will be updated by the widget.
    let mut list_selected = std::collections::HashSet::new();

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate the conrod widgets.
        {
            use conrod::{widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

            let ui = &mut ui.set_widgets();

            widget::Canvas::new().color(conrod::color::BLUE).set(ids.canvas, ui);

            // Instantiate the `ListSelect` widget.
            let num_items = list_items.len();
            let item_h = 30.0;
            let font_size = item_h as conrod::FontSize / 2;
            let (mut events, scrollbar) = widget::ListSelect::multiple(num_items)
                .flow_down()
                .item_size(item_h)
                .scrollbar_next_to()
                .w_h(400.0, 230.0)
                .top_left_with_margins_on(ids.canvas, 40.0, 40.0)
                .set(ids.list_select, ui);

            // Handle the `ListSelect`s events.
            while let Some(event) = events.next(ui, |i| list_selected.contains(&i)) {
                use conrod::widget::list_select::Event;
                match event {

                    // For the `Item` events we instantiate the `List`'s items.
                    Event::Item(item) => {
                        let label = &list_items[item.i];
                        let (color, label_color) = match list_selected.contains(&item.i) {
                            true => (conrod::color::LIGHT_BLUE, conrod::color::YELLOW),
                            false => (conrod::color::LIGHT_GREY, conrod::color::BLACK),
                        };
                        let button = widget::Button::new()
                            .border(0.0)
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

            // Instantiate the scrollbar for the list.
            if let Some(s) = scrollbar { s.set(ui); }
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}