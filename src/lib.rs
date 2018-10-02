#[cfg(test)]
mod tests {
    use feature;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn conrod_test() {
        feature::example();
    }

    #[test]
    fn ynbox_test() {
        let choice = feature::ynbox("ynbox test title", "select your choice", "Ok", "Cancel");
        println!("choice is {:?}", choice);
    }

    #[test]
    fn msgbox_test() {
        let confirmation = feature::msgbox("msgbox test title", "Please confirm this", "Ok");
        println!("confirmation is {:?}", confirmation);
    }
    
    #[test]
    fn list_test() {
        let list_items = vec! [ String::from("Dog"), String::from("Cat"), String::from("Elephant")];
        let choice  = feature::listbox("Animals", "Choose your favourite animal", &list_items);
        println!("{:?}", choice);
    }

    #[test]
    fn other_list_test(){
        let list = vec! [ String::from("Dog"), String::from("Cat"), String::from("Elephant")];
        feature::listbox_multiple(&list);
    }
}


extern crate find_folder;
#[macro_use] extern crate conrod;
mod support;
mod boxes;
mod traits;
mod constants;

pub mod feature{

    extern crate find_folder;
    extern crate image;
    use support;
    use boxes;
    use traits::{TextContainer};
    use std;
    use conrod;
    use constants;
    use conrod::backend::glium::glium::{self, Surface};

    // The initial width and height in "points".
    use constants::{WIN_H, WIN_W};

    pub fn example() {

        // Build the window.
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("Conrod with glium!")
            .with_dimensions((WIN_W, WIN_H).into());
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // Construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).theme(support::theme()).build();

        // The `widget::Id` of each widget instantiated in `support::gui`.
        let ids = support::Ids::new(ui.widget_id_generator());

        // Add a `Font` to the `Ui`'s `font::Map` from file.
        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        // Load the Rust logo from our assets folder to use as an example image.
        fn load_rust_logo(display: &glium::Display) -> glium::texture::Texture2d {
            let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
            let path = assets.join("images/rust.png");
            let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
            let image_dimensions = rgba_image.dimensions();
            let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
            let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
            texture
        }

        let mut image_map = conrod::image::Map::new();
        let rust_logo = image_map.insert(load_rust_logo(&display));

        // A demonstration of some app state that we want to control with the conrod GUI.
        let mut app = support::DemoApp::new(rust_logo);

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        //
        // Internally, the `Renderer` maintains:
        // - a `backend::glium::GlyphCache` for caching text onto a `glium::texture::Texture2d`.
        // - a `glium::Program` to use as the shader program when drawing to the `glium::Surface`.
        // - a `Vec` for collecting `backend::glium::Vertex`s generated when translating the
        // `conrod::render::Primitive`s.
        // - a `Vec` of commands that describe how to draw the vertices.
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // Start the loop:
        //
        // - Poll the window for available events.
        // - Update the widgets via the `support::gui` fn.
        // - Render the current state of the `Ui`.
        // - Repeat.
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

            // Instantiate a GUI demonstrating every widget type provided by conrod.
            support::gui(&mut ui.set_widgets(), &ids, &mut app);

            // Draw the `Ui`.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }

    pub fn ynbox(title: &str, text: &str, yesbutton: &str, nobutton: &str) -> Option<bool>{
        let yntextcontainer = boxes::YNTextContainer::from_strs(title, text, yesbutton, nobutton);
        let mut conset = ConrodSettings::load_defaults(yntextcontainer);
        let ids = boxes::YNIds::new(conset.ui.widget_id_generator());

        let mut renderer = conrod::backend::glium::Renderer::new(&conset.display).unwrap();

        'ynlabel: loop {
            for event in conset.event_loop.next(&mut conset.events_loop) { 
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &conset.display) {
                    conset.ui.handle_event(event);
                    conset.event_loop.needs_update();
                }
                match event {
                    //handle all events that request closing of the application

                    glium::glutin::Event::WindowEvent {event, ..} => match event {
                        glium::glutin::WindowEvent::CloseRequested |
                        glium::glutin::WindowEvent::KeyboardInput { 
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => break 'ynlabel,
                        _ => (),
                    }
                    _ => (),
                }

            }
            if let Some(response) = boxes::ynbox(&mut conset.ui.set_widgets(), &ids, &mut conset.app, &conset.text_container){
                return Some(response);
            }

            if let Some(primitives) = conset.ui.draw_if_changed() {
                renderer.fill(&conset.display, primitives, &conset.image_map); //possilby not needed, no images used
                let mut target = conset.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&conset.display, &mut target, &conset.image_map).unwrap();
                target.finish().unwrap();
            }
        }
        //UI gets exited without user making a choice
        None
    }

    pub fn msgbox(title: &str, text: &str, okbutton: &str) -> Option<bool>{

        let msgtextcontainer = boxes::MsgTextContainer::from_strs(title, text, okbutton);
        let mut conset = boxes::ConrodSettings::load_defaults(msgtextcontainer);
        let ids = boxes::MsgIds::new(conset.ui.widget_id_generator());
        boxes::msgbox(&conset.text_container)   
    }

    pub fn listbox(title: &str, text: &str, list: &Vec<String>) -> Option<usize>{
        let list_text_container = boxes::ListTextContainer::from_strs(title, text, list);
        let mut conset = ConrodSettings::load_defaults(list_text_container);
        let ids = boxes::ListIds::new(conset.ui.widget_id_generator());

        let mut renderer = conrod::backend::glium::Renderer::new(&conset.display).unwrap();
        'main: loop {
            for event in conset.event_loop.next(&mut conset.events_loop) { 
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &conset.display) {
                    conset.ui.handle_event(event);
                    conset.event_loop.needs_update();
                }
                match event {
                    //handle all events that request closing of the application
                    //use glium::glutin;

                    glium::glutin::Event::WindowEvent {event, ..} => match event {
                        glium::glutin::WindowEvent::CloseRequested |
                        glium::glutin::WindowEvent::KeyboardInput { 
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => break 'main,
                        _ => (),
                    }
                    _ => (),
                }

            }
            if let Some(response) = boxes::listbox_single(&mut conset.ui.set_widgets(), &ids, &mut conset.app, &conset.text_container){
                return Some(response);
            }

            if let Some(primitives) = conset.ui.draw_if_changed() {
                renderer.fill(&conset.display, primitives, &conset.image_map); //possilby not needed, no images used
                let mut target = conset.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&conset.display, &mut target, &conset.image_map).unwrap();
                target.finish().unwrap();
            }
        }
        

        None
    }
    pub fn listbox_multiple(list: &Vec<String>){
        let text_container = boxes::ListTextContainer::from_strs("sometitle", "sometext", list);
        boxes::listbox_multiple(&text_container);
    }



    struct ConrodSettings<T: TextContainer>{
        text_container: T,
        events_loop: glium::glutin::EventsLoop,
        event_loop: support::EventLoop,
        display: glium::Display,
        ui: conrod::Ui,
        image_map: conrod::image::Map<conrod::glium::Texture2d>,
        app: boxes::EmptyApp,
    }
    impl<T> ConrodSettings<T> where T: TextContainer{
        /// Returns a struct that contains the default settings to open a conrod window with
        pub fn load_defaults(textcontainer: T) -> ConrodSettings<T>{

            use constants::{WIN_H, WIN_W};


            let events_loop = glium::glutin::EventsLoop::new();

            //window and context builders, consumed to create display
            let window = glium::glutin::WindowBuilder::new().with_title(textcontainer.title()).with_dimensions((WIN_W, WIN_H).into());
            let context = glium::glutin::ContextBuilder::new().with_vsync(true).with_multisampling(4);

            let display = glium::Display::new(window, context, &events_loop).unwrap();
            
            let ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).theme(boxes::theme()).build();

            let mut settings = ConrodSettings{
                text_container: textcontainer,

                //events_looop is a queue of occuredevents based on the backend,
                //event_loop handles the current event independent of the backend
                events_loop: events_loop,

                event_loop: support::EventLoop::new(),

                display: display,

                ui: ui,

                image_map: conrod::image::Map::new(), //possibly not needed, if no images used

                app: boxes::EmptyApp{},
            };

            let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
            let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
            settings.ui.fonts.insert_from_file(font_path).unwrap();

            settings
        }
    }

}
