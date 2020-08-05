use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;
use std::collections::HashMap;

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', Cursive::quit);
    siv.set_user_data(create_user_data());
    create_departments_view(&mut siv);
    siv.run();
}

fn create_user_data() -> HashMap<String, Vec<String>> {
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
    departments
}

fn create_departments_view(s: &mut Cursive) {
    let department_buttons = LinearLayout::vertical()
        .child(Button::new("Create new", create_department))
        .child(Button::new("Remove", remove_department));

    let departments_view = create_departments_list(s);

    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(departments_view)
                .child(DummyView)
                .child(department_buttons),
        )
        .title("Departments")
        .button("Quit", Cursive::quit),
    );
}

fn create_departments_list(s: &mut Cursive) -> Box<dyn View> {
    let mut select_view = SelectView::<String>::new();
    let departments = s.user_data::<HashMap<String, Vec<String>>>().unwrap();
    let mut department_names: Vec<String> = Vec::new();
    for (k, _v) in departments {
        department_names.push(k.clone());
    }
    department_names.sort();
    select_view.add_all_str(department_names);
    let departments_view = select_view
        .on_submit(|s, d| show_department(s, d))
        .with_name("departments")
        .scrollable()
        .fixed_size((40, 5));

    Box::new(departments_view)
}

fn create_employees_view(s: &mut Cursive, department_name: &str) -> Box<dyn View> {
    let mut select_view = SelectView::<String>::new();
    let departments = s.user_data::<HashMap<String, Vec<String>>>().unwrap();
    select_view.add_all_str(departments.get(department_name).unwrap());
    let employees_view = select_view
        // .on_submit(show_department)
        .scrollable()
        .fixed_size((40, 10));

    let dep_name: String = department_name.to_string();
    let employee_buttons = LinearLayout::vertical().child(Button::new("Add new", move |s| {
        create_employee(s, &dep_name);
    }));
    // .child(Button::new("Remove", |s| remove_employee(s, department_name)));

    let layout = LinearLayout::horizontal()
        .child(employees_view)
        .child(DummyView)
        .child(employee_buttons);

    Box::new(layout)
}

fn show_department(s: &mut Cursive, department_name: &str) {
    s.pop_layer();
    let emp_view = create_employees_view(s, department_name);
    s.add_layer(
        Dialog::around(emp_view)
            .title(format!("Employees of {}", department_name))
            .button("Back", |s| {
                s.pop_layer();
                create_departments_view(s)
            }),
    );
}

fn create_department(s: &mut Cursive) {
    fn ok(s: &mut Cursive, department_name: &str) {
        s.with_user_data(|departments: &mut HashMap<String, Vec<String>>| {
            departments
                .entry(String::from(department_name))
                .or_insert(Vec::new());
        });
        s.pop_layer();
        create_departments_view(s);
    }
    s.pop_layer();
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

fn create_employee(s: &mut Cursive, department_name: &str) {
    fn ok(s: &mut Cursive, employee_name: &str, department_name: &str) {
        s.with_user_data(|departments: &mut HashMap<String, Vec<String>>| {
            let d = departments
                .entry(String::from(department_name))
                .or_insert(Vec::new());
            d.push(employee_name.to_string());
        });
        s.pop_layer();
        show_department(s, department_name);
    }
    let dep_name1 = department_name.to_string();
    let dep_name2 = department_name.to_string();
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(move |s, n| {
                    ok(s, n, &dep_name1);
                })
                .with_name("name")
                .fixed_width(40),
        )
        .title(format!("Add employee to {}", department_name))
        .button("Ok", move |s| {
            let name = s
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(s, &name, &dep_name2);
        })
        .button("Cancel", |s| {
            s.pop_layer();
            create_departments_view(s);
        }),
    );
}

fn remove_employee(s: &mut Cursive, department_name: &str) {}
