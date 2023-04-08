use eframe::{App, CreationContext, Frame};
use egui::{Context, TopBottomPanel, CentralPanel, ComboBox, TextEdit, Ui, ScrollArea, Layout, Label, TextStyle, FontDefinitions, FontFamily, Separator};

use crate::{models::{Role, User, Employee, NewEmployee}, get_employees, insert_employee};

#[derive(Default)]
pub struct POS {
    get_employee: Employee,
    employee: NewEmployee,
    user: User,
    role: Role,
    toggle: Toggle,
}

impl POS {
    pub fn new(cc: &CreationContext<'_>) -> Self { Self::default()}

    pub fn add_employee_screen(&mut self, ctx: &Context, frame: &mut Frame) {
        self.render_top_panel(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            ui.label("First Name:");
            ui.text_edit_singleline(&mut self.employee.first_name);
            ui.label("Last Name: ");
            ui.text_edit_singleline(&mut self.employee.last_name);
            ui.label("Email: ");
            ui.text_edit_singleline(&mut self.employee.email);
            ui.label("Choose Position");
            ComboBox::new("new box", "").selected_text(format!("{:?}", self.employee.position))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.employee.position, Role::CEO, "CEO");
                    ui.selectable_value(&mut self.employee.position, Role::SUPERVISOR, "SUPERVISOR");
                    ui.selectable_value(&mut self.employee.position, Role::ANALYST, "ANALYST");
                    ui.selectable_value(&mut self.employee.position, Role::HR, "HR EMPLOYEE");
                    ui.selectable_value(&mut self.employee.position, Role::UNEMPLOYED, "UNEMPLOYED");
                });
            ui.add_space(5.);
            let default_pay_label = ui.selectable_label(false, format!("Default Salary: ${}", self.employee.position.starting_pay().unwrap_or(0)));
                if default_pay_label.clicked() { self.employee.pay = self.employee.position.starting_pay(); }
            let submit_btn = ui.button("Submit");
                if submit_btn.clicked() {
                    self.employee.pay = self.employee.position.starting_pay();
                    self.employee.employed =  true;
                    let new_emp = self.employee.clone();
                    insert_employee(new_emp);
                    self.employee.make_default();
                }
        });
    }

    pub fn login_screen(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {ui.heading("COMPANY LOGIN SCREEN"); });
            ui.vertical_centered(|ui| {
                ui.label("USERNAME: ");
                ui.text_edit_singleline(&mut self.user.username);
                ui.add_space(10.);
                ui.label("PASSWORD");
                let pass = TextEdit::singleline(&mut self.user.password).password(true);
                ui.add(pass);
            })
        });
    }

    pub fn menu_screen(&mut self, ctx: &Context, frame: &mut Frame) {
        self.render_top_panel(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(egui::Align::Center), |ui| {
                let add_employee = ui.selectable_label(false, "Add an employee");
                if add_employee.clicked() {
                    self.toggle.add_employee = true;
                }
            let list_employees = ui.selectable_label(false, "Employee list");
                if list_employees.clicked() {
                        self.toggle.list_employees =  true;
                }
            })


        });
    }

    pub fn employee_list_screen(&mut self, ctx: &Context, frame: &mut Frame) {
        self.render_top_panel(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| { ui.heading("COMPANY EMPLOYEES")});
        
        ScrollArea::new([true,true]).auto_shrink([false,false]).show( ui, |ui| {
            self.render_employees(ui)
        });
    });
    }

    pub fn render_employees(&self, ui: &mut Ui) {
        for emp in &self.toggle.employees {
            let id = format!("EMPLOYEE ID: #{}", emp.id.unwrap_or_default());
            let name = format!("Name: {} {}", emp.first_name,emp.last_name);
            let email = format!("EMAIL: {}", emp.email);
            let position = format!("POSITION: {}", emp.position);
            let pay = format!("SALARY: $ {}", emp.pay.unwrap_or_default());
            let employed = format!("CURRENTLY EMPLOYED: {}", emp.employed);
            ui.label(id);
            ui.label(name);
            ui.label(email);
            ui.label(position);
            ui.label(pay);
            ui.label(employed);
            ui.add(Separator::default());
            ui.add_space(25.);
        }
    }

    pub fn render_top_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("Consistent Top Panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.add(Label::new(egui::RichText::new("COMPANY LOGO").heading()));
                    ui.add_space(10.);
                    let back_btn = ui.button("Main Menu");
                    if self.toggle.add_employee == true && back_btn.clicked() { self.toggle.add_employee = false }
                    else if self.toggle.list_employees == true && back_btn.clicked() { self.toggle.list_employees = false }
                });

                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    let close_btn = ui.button("❌");
                        if close_btn.clicked() { frame.close()}
                    let refresh_btn = ui.button("🔄");
                        if refresh_btn.clicked() {if self.toggle.add_employee == true { self.employee.make_default();}
                                                  else if self.toggle.list_employees == true { self.toggle.employees = get_employees() }
                                                 }
                    let theme_btn = ui.button("⚙");
                        if theme_btn.clicked() { () }
                                                 
                })
            })
        });
    }

    pub fn configure_fonts(&mut self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();
        ctx.set_fonts(font_def)
    }

}


impl App for POS {
     fn update(&mut self, ctx: &Context, frame: &mut Frame) {

        if self.user.username == "admin" && self.user.password == "admin" {
            if self.toggle.add_employee == true {
                POS::add_employee_screen(self, ctx, frame );
            }
            else if self.toggle.list_employees == true {
                self.toggle.employees = get_employees();
                POS::employee_list_screen(self, ctx, frame);
            }
            else { POS::menu_screen(self, ctx, frame)}
        }
        else { POS::login_screen(self, ctx, frame)}


    }
}










    #[derive(Default)]
    struct Toggle {
        add_employee: bool,
        list_employees: bool,
        employees: Vec<Employee>,
    }
