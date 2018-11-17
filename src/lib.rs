#[cfg(test)]
mod tests;

mod eventloop;
mod constants;
mod gui;

/// Creates a dialog box with a text message and a confirm button, and returns the user's response.
/// 
/// Returns `Some(true)` if the user presses the confirm button.
/// 
/// Returns `None` if the user closes the window without making a choice.
pub fn msgbox(title: &str, msg: &str, button_text: &str) -> Option<bool>{ 
    gui::msgbox::msgbox(title, msg, button_text)
}

/// Creates a dialog box with a text message, a confirm and decline button, and returns the user's response.
/// 
/// Returns `Some(true)` if the user selects the confirm button, `Some(false)` if they select the decline button. 
/// 
/// Returns `None` if the user closes the window without making a choice.
pub fn ynbox(title: &str, msg: &str, yes_button: &str, no_button: &str) -> Option<bool>{
    gui::ynbox::ynbox(title, msg, yes_button, no_button)
}

/// Creates a dialog box with a text message, a list of text items and a confirm button, and returns an index of the user's choice of items in the list.
/// 
/// Not yet supported: The list will allow selection of multiple items if you pass true as the `multiselect` parameter.
/// 
/// Returns `Some(usize)`, where usize is the index of the item the user selects, when the user confirms their choice.
/// 
/// Returns `None` if the user closes the window without making a choice.
pub fn listbox(title: &str, msg: &str, list: &Vec<String>, multiselect: bool) -> Option<usize>{
    if multiselect{
        //stub
        //multiselect listbox implementation call
        None
    }else{
        gui::listbox::listbox_single(title, msg, list)
    }
}