use adw::prelude::*;
use relm4::prelude::*;
use locales::t;

struct App {
    counter: u8,
    app_language: String,
}

#[derive(Debug)]
enum Msg {
    Increment,
    Decrement,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = Msg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some(t!("app.title", &model.app_language).as_str()),
            set_default_size: (500, 200),


            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    set_title_widget = Some(&window_title),
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Label {
                        set_margin_all: 5,
                        #[watch]
                        set_label: &format!(
                            "{} {}",
                            t!("ui.counter", &model.app_language),
                            model.counter
                        ),
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 5,
                        set_halign: gtk::Align::Center,

                        gtk::Button {
                            set_label: "-",
                            connect_clicked => Msg::Decrement
                        },

                        gtk::Button {
                            set_label: "+",
                            connect_clicked => Msg::Increment
                        }
                    }
                }
            }
        }
    }

    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App { 
            counter,
            /* 
            * Valid choices are:
            * "ar" = Arabic
            * "en" = English
            * TODO: Choose language by reading LANG env. variable, And allow the user to modify it inside the app,
            * E.g. a parameter or a setting inside the app.
            */
            app_language: "en".to_string(),
         };


        let window_title = adw::WindowTitle::new(
            t!("app.title", &model.app_language).as_str(),
            t!("app.subtitle", &model.app_language).as_str()
        );

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            Msg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("org.example.relm4.counter");
    app.run::<App>(0);
}