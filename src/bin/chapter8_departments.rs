use cursive::traits::*;
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, SelectView,
};
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', Cursive::quit);

    let department_buttons = LinearLayout::vertical()
        .child(Button::new("Create new", create_department))
        .child(Button::new("Remove", remove_department));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(departments_view())
                .child(DummyView)
                .child(department_buttons),
        )
        .title("Departments")
        .button("Quit", Cursive::quit),
    );

    siv.run();
}

fn departments_view() -> Box<dyn View> {
    let mut select_view = SelectView::<String>::new();
    select_view.add_all_str(
        [
            String::from("Engineering"),
            String::from("Sales"),
        ]
        .iter(),
    );
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
