#[test]
pub fn msgbox_test() {
    use crate::msgbox;
    let response = msgbox("msgbox_title", "Please confirm", "Confirm");
    println!("{:?}", response);
}

#[test]
pub fn ynbox_test() {
    use crate::ynbox;
    let response = ynbox("ynbox_title", "Please select yes or no", "Yes", "No");
    println!("{:?}", response);
}

#[test]
pub fn listbox_test() {
    use crate::listbox;
    let list = vec![String::from("Dog"), String::from("Dog"), String::from("Elephant")];
    let response = listbox("listbox_title", "Please select an item from the list", &list, false);
    if let Some(index) = response {
        println!("{:?}", list[index]);
    } else {
        println!("failed to select an item");
    }
    println!("test finished ");
}

