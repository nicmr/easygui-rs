use conrod::backend::glium::glium;
use conrod::glium::Surface;
use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
use crate::constants::*; //unused so far
                         //use conrod::Colorable; //unused so far
use conrod;
use conrod::widget_ids;

use crate::gui::conrod::ConrodSettings;

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

pub fn msgbox(title: &str, msg: &str, button_text: &str) -> Option<bool> {
    let mut conset = ConrodSettings::load_defaults(title);
    let ids = MsgIds::new(conset.ui.widget_id_generator());

    let _app = {};

    let mut renderer = conrod::backend::glium::Renderer::new(&conset.display).unwrap();
    'ynlabel: loop {
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
                    } => break 'ynlabel,
                    _ => (),
                },
                _ => (),
            }
        }

        {
            let ui_cell = &mut conset.ui.set_widgets();

            //Canvas
            widget::Canvas::new()
                .pad(MARGIN)
                .scroll_kids_vertically()
                .set(ids.canvas, ui_cell);

            //Title
            widget::Text::new(title)
                .font_size(TITLE_SIZE)
                .mid_top_of(ids.canvas)
                .set(ids.title, ui_cell);

            // Text
            widget::Text::new(msg)
                .padded_w_of(ids.canvas, MARGIN)
                .down(60.0)
                .align_middle_x_of(ids.canvas)
                .center_justify()
                .line_spacing(5.0)
                .set(ids.text, ui_cell);

            //Ok button
            let button_side = 100.0;
            for _press in widget::Button::new()
                .label(button_text)
                //.mid_left_with_margin_on(ids.canvas, MARGIN)
                .mid_bottom()
                .down_from(ids.text, 50.0)
                .w_h(button_side, button_side)
                .set(ids.okbutton, ui_cell)
            {
                return Some(true);
            }
        }

        if let Some(primitives) = conset.ui.draw_if_changed() {
            renderer.fill(&conset.display, primitives, &conset.image_map);
            let mut target = conset.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer
                .draw(&conset.display, &mut target, &conset.image_map)
                .unwrap();
            target.finish().unwrap();
        }
    }

    //UI gets exited without user making a choice
    None
}
