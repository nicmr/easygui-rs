use crate::constants::*;
use crate::eventloop::EventLoop;

use conrod::backend::glium::glium;

/// An app struct is supposed to representing the state of the window. The Empty state struct is to be used as a placeholder for stateless windows.
pub struct EmptyApp{

}

pub struct ConrodSettings{
    pub events_loop: glium::glutin::EventsLoop,
    pub event_loop: EventLoop,
    pub display: glium::Display,
    pub ui: conrod::Ui,
    pub image_map: conrod::image::Map<conrod::glium::Texture2d>,
    pub app: EmptyApp,
}

impl ConrodSettings{
    pub fn load_defaults(title: &str) -> ConrodSettings{

        let events_loop = glium::glutin::EventsLoop::new();

        //window and context builders, consumed to create display
        let window = glium::glutin::WindowBuilder::new().with_title(title).with_dimensions((WIN_W, WIN_H).into());
        let context = glium::glutin::ContextBuilder::new().with_vsync(true).with_multisampling(4);

        let display = glium::Display::new(window, context, &events_loop).unwrap();
        
        let ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).theme(theme()).build();

        let mut settings = ConrodSettings{

            //events_loop is a queue of occured events based on the backend,
            //event_loop handles the current event independent of the backend
            events_loop: events_loop,
            event_loop: EventLoop::new(),
            display: display,
            ui: ui,
            image_map: conrod::image::Map::new(), //possibly not needed, if no images used
            app: EmptyApp{},
        };

        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        settings.ui.fonts.insert_from_file(font_path).unwrap();

        settings
    }
}

/// Defines our default theme for all GUI windows
pub fn theme() -> conrod::Theme{
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme{
        name: String::from("easygui-default"),
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