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
        assert_eq!(2 + 2, 4);
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
}
// #[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;
// #[cfg(all(feature="winit", feature="glium"))] mod support;

#[macro_use] extern crate conrod;
mod support;

mod feature {
    extern crate find_folder;
    extern crate image;
    use conrod;
    use conrod::backend::glium::glium::{self, Surface};
    use support;
    use std;

    // The initial width and height in "points".
    const WIN_W: u32 = support::WIN_W;
    const WIN_H: u32 = support::WIN_H;

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
        let settings = support::boxes::YNSettings{
            title, text, yesbutton, nobutton
        };
        //build event loop, window, context, display
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions((WIN_W, WIN_H).into());
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // build ui
        let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).theme(support::boxes::theme()).build();


        let ids = support::boxes::YNIds::new(ui.widget_id_generator());

        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();


        let mut image_map: conrod::image::Map<conrod::glium::Texture2d> = conrod::image::Map::new(); //possilby not needed, no images useds

        let mut app = support::boxes::EmptyApp{};

        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();


        let mut event_loop = support::EventLoop::new();
        'ynlabel: loop {
            for event in event_loop.next(&mut events_loop) { 
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
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
            if let Some(response) = support::boxes::ynbox(&mut ui.set_widgets(), &ids, &mut app, settings){
                return Some(response);
            }

            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map); //possilby not needed, no images used
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
        //UI gets exited without user making a choice
        None
    }

    pub fn msgbox(title: &str, text: &str, okbutton: &str) -> Option<bool>{
        let settings = support::boxes::MsgSettings{
            title, text, okbutton,
        };
        //build event loop, window, context, display
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions((WIN_W, WIN_H).into());
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // build ui
        let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).theme(support::boxes::theme()).build();


        let ids = support::boxes::MsgIds::new(ui.widget_id_generator());

        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();


        let mut image_map: conrod::image::Map<conrod::glium::Texture2d> = conrod::image::Map::new(); //possilby not needed, no images useds

        let mut app = support::boxes::EmptyApp{};

        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();


        let mut event_loop = support::EventLoop::new();
        'ynlabel: loop {
            for event in event_loop.next(&mut events_loop) { 
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
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
                        } => break 'ynlabel,
                        _ => (),
                    }
                    _ => (),
                }

            }
            if let Some(response) = support::boxes::msgbox(&mut ui.set_widgets(), &ids, &mut app, settings){
                return Some(response);
            }

            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map); //possilby not needed, no images used
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
        //UI gets exited without user making a choice
        None
    }
}