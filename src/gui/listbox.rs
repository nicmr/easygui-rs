use crate::constants::*;
use conrod::backend::glium::glium;
use conrod::{widget, Labelable, Positionable, Sizeable, Widget, Colorable, Borderable};
use conrod::glium::Surface; //unused so far
//use conrod::Colorable; //unused so far
use conrod;
use conrod::widget_ids;

use crate::gui::conrod::ConrodSettings;

widget_ids! {
    pub struct ListIds{
        canvas,
        title,
        text,
        list_select,
        confirm_button,
    }
}
pub fn listbox_single(title: &str, text: &str, list: &Vec<String>) -> Option<usize>{


    // Required conrod settings
    let mut conset = ConrodSettings::load_defaults(title);
    let ids = ListIds::new(conset.ui.widget_id_generator());
    let mut renderer = conrod::backend::glium::Renderer::new(&conset.display).unwrap();
    
    // Specific to listboxs
    let _app = {};
    let mut list_selected = std::collections::HashSet::new();
    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();


    

    'main: loop {
        for event in conset.event_loop.next(&mut conset.events_loop) {
            if let Some(event) =
                conrod::backend::winit::convert_event(event.clone(), &conset.display)
            {
                conset.ui.handle_event(event);
                conset.event_loop.needs_update();
            }
            match event {
                //handle all events that request closing of the application
                //use glium::glutin;
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::CloseRequested
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
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
            let ui_cell = &mut conset.ui.set_widgets();

            widget::Canvas::new().color(conrod::color::BLUE).set(ids.canvas, ui_cell);

            // Instantiate the `ListSelect` widget.
            let num_items = list.len();
            let item_h = 30.0;
            let font_size = item_h as conrod::FontSize / 2;
            let (mut events, scrollbar) = widget::ListSelect::multiple(num_items)
                .flow_down()
                .item_size(item_h)
                .scrollbar_next_to()
                .w_h(400.0, 230.0)
                .top_left_with_margins_on(ids.canvas, 40.0, 40.0)
                .set(ids.list_select, ui_cell);

            for _press in widget::Button::new()
            .label("Confirm")
            //.mid_left_with_margin_on(ids.canvas, MARGIN)
            .mid_bottom()
            .right_from(ids.list_select, 60.0)
            //.down_from(ids.text, 60.0)
            .w_h(100.0, 100.0)
            .set(ids.confirm_button, ui_cell)
            {
                if list_selected.len() == 0 {
                    println!("please select an item before continuing");
                }else{
                    return Some(list_selected.into_iter().nth(0)?);
                }                
            }
            // Handle the `ListSelect`s events.
            while let Some(event) = events.next(ui_cell, |i| list_selected.contains(&i)) {
                use conrod::widget::list_select::Event;
                match event {

                    // For the `Item` events we instantiate the `List`'s items.
                    Event::Item(item) => {
                        let label = &list[item.i];
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
                        item.set(button, ui_cell);
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
            if let Some(s) = scrollbar { s.set(ui_cell); }
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = conset.ui.draw_if_changed() {
            renderer.fill(&conset.display, primitives, &image_map);
            let mut target = conset.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&conset.display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
    None
}