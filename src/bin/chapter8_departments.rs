use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;
use std::collections::HashMap;

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', Cursive::quit);

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    departments.insert(
        String::from("Engineering"),
        vec!["Emily", "Ethan", "Evie", "Eva", "Erin"]
            .iter()
            .map(|s: &&str| s.to_string())
            .collect(),
    );
    departments.insert(
        String::from("Sales"),
        vec!["Sophie", "Scarlett", "Samuel", "Sienna", "Sebastian"]
            .iter()
            .map(|s: &&str| s.to_string())
            .collect(),
    );

    siv.set_user_data(departments);

    let department_buttons = LinearLayout::vertical()
        .child(Button::new("Create new", create_department))
        .child(Button::new("Remove", remove_department));

    let departments_view = create_departments_view(&mut siv);

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(departments_view)
                .child(DummyView)
                .child(department_buttons),
        )
        .title("Departments")
        .button("Quit", Cursive::quit),
    );

    siv.run();
}

fn create_departments_view(siv: &mut Cursive) -> Box<dyn View> {
    let mut select_view = SelectView::<String>::new();
    let departments = siv.user_data::<HashMap<String, Vec<String>>>().unwrap();
    select_view.add_all_str(departments.keys());
    let departments_view = select_view
        .on_submit(show_department)
        .with_name("departments")
        .scrollable()
        .fixed_size((40, 5));

    Box::new(departments_view)
}

fn show_department(s: &mut Cursive, department: &str) {}

fn create_department(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("departments", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });
        s.pop_layer();
    }
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("name")
                .fixed_width(40),
        )
        .title("Create department")
        .button("Ok", |s| {
            let name = s
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}

fn remove_department(s: &mut Cursive) {}
