/*
 * use gtk::prelude::*;
use gtk::{
    Button, Entry, Grid, Stack, StackSidebar, Window,
};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(gtk::WindowType::Toplevel);
    window.set_title("GtkStackSidebar Example");
    window.set_default_size(400, 300);

    let grid = Grid::new();
    let button = Button::with_label("Stack Page 1");
    let entry = Entry::new();
    entry.set_text("Stack Page 2");
    grid.attach(&entry, 0, 0, 1, 1);

    let stack = Stack::new();
    stack.add_titled(&button, "page1", "Stack Page 1");
    stack.add_titled(&grid, "page2", "Stack Page 2");

    let sidebar = StackSidebar::new();
    sidebar.set_stack(&stack);

    grid.attach(&sidebar, 1, 0, 1, 2);
    window.add(&grid);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

 */
extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;

use gtk::{Application,Window, ApplicationWindow,Entry,HeaderBar,Button,Grid,Stack,StackSidebar};

fn main() {
	
         let app = Application::builder()
        .application_id("Rust-on-gtk")
        .build();
        
app.connect_activate(|app| {
	         //Control
	         let text = Entry::new();
             let button = Button:: from_icon_name(Some("add"),gtk::IconSize::Menu);
             
             //Grid
             let grid = Grid::builder()
             .width_request(20)
             .height_request(170)             
             .build();
			 
             //stack
             let stack = Stack::builder()
             .width_request(400)
             .height_request(170)
             .name("Test1")
             .build();
             
              //StackSidebar
             let stacksidebar = StackSidebar::builder()  
             . margin_bottom(170)           
             .build();
             
             //Headerbar
             let header = HeaderBar::builder()
             .title("Hello")
             .subtitle("Powered by GTK and Rust")
             .show_close_button(true)
             .build();
            
             //Windows
             
              let window = ApplicationWindow::builder()
            .application(app)
            .default_width(640)
            .default_height(480)	
            .build();	

		
			 let screen = gtk::prelude::GtkWindowExt::screen(&window).unwrap();
			 grid.attach(&stacksidebar, 1, 70, 1, 1);
			 grid.attach(&stack, 1, 0, 1, 2);

			 
			 stack.add_titled(&button, "page1", "Stack Page 1");
			 stack.add_titled(&text, "page2", "Stack Page 2");
			 stacksidebar.set_stack(&stack);
			 window.set_titlebar(Some(&header));  
			grid.add(&stacksidebar); 
			window.add(&grid);
            window.show_all();
    });
    app.run();

}

