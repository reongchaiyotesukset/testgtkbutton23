extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;

use gtk::{Application,Window, ApplicationWindow,Entry,HeaderBar,Button,Grid,Stack,StackSidebar,Layout};

fn main() {
	
         let app = Application::builder()
        .application_id("Rust-on-gtk")
        .build();
        
app.connect_activate(|app| {
	         //text1
	         let text1 = Entry::builder()
            .placeholder_text("input")
            .can_default(true)
            .can_focus(true)
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(70)            
            .margin_end(20)
            .build();
            
	         let text2 = Entry::new();
             //button
             let button1 = Button::builder()
            .label("Click1")
            .margin(60)
            .margin_bottom(60)
            .margin_top(60)
            .margin_start(60)
            .margin_end(60)
            .build();
            
             let button_cancle = Button:: from_icon_name(Some("cancle"),gtk::IconSize::Menu);
             let button_edit = Button:: from_icon_name(Some("edit"),gtk::IconSize::Menu);
             //Layout
              
			let layout = Layout::builder()
		     .height_request(400)
             .width_request(300)	
             .build();
            
             //Grid
             let grid = Grid::builder() 
             .halign(gtk::Align::Start)         
             .build();
			 
             //stack
             let stack = Stack::builder()
             .name("Test1")
             .build();
             
              //StackSidebar
             let stacksidebar = StackSidebar::builder()   
              .expand(true)
              .halign(gtk::Align::Fill)
             .build();
             
             //Headerbar
             let header = HeaderBar::builder()
             .title("Hello")
             .subtitle("Powered by GTK and Rust")
             .show_close_button(true)
             .build();
            
             //Windows Application
             
             let window = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)	
            .build();
            
            			
			 let screen = gtk::prelude::GtkWindowExt::screen(&window).unwrap();
			 
			 
			  //Grid  Attach
			 grid.attach(&stacksidebar, 0, 0, 1, 1);
			 grid.attach(&stack, 1, 0, 1, 2);
			 
			 stack.add_titled(&layout, "page1", "Stack Page 1");
			 stack.add_titled(&text2, "page2", "Stack Page 2");
			 stack.add_titled(&button_cancle, "page3", "Stack Page 3");
			  
			 stacksidebar.set_stack(&stack);
			 window.set_titlebar(Some(&header));  
			 
			layout.add(&text1); 
		    layout.add(&button1); 
			grid.add(&stacksidebar); 
			window.add(&grid);
            window.show_all();
    });
    app.run();

}

