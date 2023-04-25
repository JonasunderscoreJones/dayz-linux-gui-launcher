use eframe::egui;
use dialog::DialogBox;
use directories::UserDirs;
use std::fs;
use std::path::Path;
use std::process::Command;
use toml;
use std::fmt;


fn main() -> Result<(), eframe::Error> {
    let config_dir = UserDirs::new().unwrap().home_dir().to_str().unwrap().to_string() + "/.config/dayz-linux-gui-launcher";
    let dayz_cli_script = "/usr/bin";
    // run the setup function
    setup(&config_dir, &dayz_cli_script.to_string());
    let options = eframe::NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        decorated: false,
        // To have rounded corners we need transparency:
        transparent: true,
        min_window_size: Some(egui::vec2(500.0, 260.0)),
        max_window_size: Some(egui::vec2(500.0, 260.0)),
        initial_window_size: Some(egui::vec2(1000.0, 340.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Dayz Linux Gui Launcher", // unused title
        options,
        Box::new(|_cc| Box::new(DayzLinuxGuiLauncher::default())),
    )
}

fn setup(config_dir: &String, local_dir: &String) {
    // create config directory and run file if not exist
    

    if !Path::new(config_dir).exists() {
        println!("Config directory missing! Creating directory: {}", config_dir);
        create_dir(config_dir);
        create_config_file(config_dir);
    }
    if !Path::new(local_dir).exists() {
        println!("Local directory missing! Creating directory: {}", local_dir);
        create_dir(local_dir);
        //TODO: fetch ./dayzslauncher.sh from github
    }

}

fn create_dir(dir: &String) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    Ok(())
}

fn create_config_file(config_dir: &String) {
    let config_file = config_dir.to_string() + "/config.toml";
    // create toml config file with table called root and version number inside
    let config = toml::to_string_pretty(&toml::value::Table::new()).unwrap();
    let mut config: toml::value::Table = toml::from_str(&config).unwrap();
    config.insert("main".to_string(), toml::Value::Table(toml::value::Table::new()));

    let mut main = config.get_mut("main").unwrap().as_table_mut().unwrap();
    main.insert("version".to_string(), toml::Value::String("0.1.0".to_string()));
    main.insert("dayz-cli-launcher-version".to_string(), toml::Value::String("0.5.3".to_string()));

    // create a table called last_values
    main.insert("last_values".to_string(), toml::Value::Table(toml::value::Table::new()));
    let mut last_values = main.get_mut("last_values").unwrap().as_table_mut().unwrap();
    last_values.insert("playername".to_string(), toml::Value::String("".to_string()));
    last_values.insert("serverip".to_string(), toml::Value::String("".to_string()));
    last_values.insert("serverport".to_string(), toml::Value::String("".to_string()));
    last_values.insert("queryport".to_string(), toml::Value::String("".to_string()));
    last_values.insert("steamexe".to_string(), toml::Value::String("default".to_string()));
    last_values.insert("customsteamexe".to_string(), toml::Value::String("".to_string()));
    last_values.insert("profilename".to_string(), toml::Value::String("".to_string()));

    // write config file
    fs::write(config_file, config.to_string()).expect("Unable to write file");
}

fn save_new_profile(config_dir: &String, profile_name: &String, playername: &String, serverip: &String, serverport: &String, queryport: &String, steamexe: &String, customsteamexe: &String) {
    let config_file = config_dir.to_string() + "/config.toml";
    // read config file
    let config = fs::read_to_string(&config_file).expect("Unable to read file");

    // parse config file
    let mut config: toml::value::Table = toml::from_str(&config).unwrap();

    // get profiles table
    let mut main = config.get_mut("main").unwrap().as_table_mut().unwrap();
    main.insert("profiles".to_string(), toml::Value::Table(toml::value::Table::new()));
    let mut profiles = main.get_mut("profiles").unwrap().as_table_mut().unwrap();

    // create a table called profile_name
    profiles.insert(profile_name.to_string(), toml::Value::Table(toml::value::Table::new()));
    let mut profile = profiles.get_mut(profile_name).unwrap().as_table_mut().unwrap();

    // add values to profile_name
    profile.insert("playername".to_string(), toml::Value::String(playername.to_string()));
    profile.insert("serverip".to_string(), toml::Value::String(serverip.to_string()));
    profile.insert("serverport".to_string(), toml::Value::String(serverport.to_string()));
    profile.insert("queryport".to_string(), toml::Value::String(queryport.to_string()));
    profile.insert("steamexe".to_string(), toml::Value::String(steamexe.to_string()));
    profile.insert("customsteamexe".to_string(), toml::Value::String(customsteamexe.to_string()));

    // write config file
    fs::write(config_file, config.to_string()).expect("Unable to write file");
}

fn load_profile(config_dir: &String, profile_name: &String) -> (String, String, String, String, String, String) {
    let config_file = config_dir.to_string() + "/config.toml";
    // read config file
    let config = fs::read_to_string(&config_file).expect("Unable to read file");

    // parse config file
    let config: toml::value::Table = toml::from_str(&config).unwrap();

    // get profiles table
    let main = config.get("main").unwrap().as_table().unwrap();
    let profiles = main.get("profiles").unwrap().as_table().unwrap();

    // get profile_name table
    let profile = profiles.get(profile_name).unwrap().as_table().unwrap();

    // get values from profile_name
    let _playername = profile.get("playername").unwrap().as_str().unwrap();
    let _serverip = profile.get("serverip").unwrap().as_str().unwrap();
    let _serverport = profile.get("serverport").unwrap().as_str().unwrap();
    let _queryport = profile.get("queryport").unwrap().as_str().unwrap();
    let _steamexe = profile.get("steamexe").unwrap().as_str().unwrap();
    let _customsteamexe = profile.get("customsteamexe").unwrap().as_str().unwrap();

    return (_playername.to_owned(), _serverip.to_owned(), _serverport.to_owned(), _queryport.to_owned(), _steamexe.to_owned(), _customsteamexe.to_owned());
    
    
}


fn launch(playername: &String, serverip: &String, serverport: &String, queryport: &String, steamexe: &String, customsteamexe: &String) {
    // command structure: ./dayz-launcher.sh --steam <"" if default | flatpak if Flatpak | "/path/to/executable" if custom> --launch --name <playername> --server serverip:serverport --port <queryport>
    let dayz_cli_script = "/usr/bin";
    // create command
    let mut command = Command::new(dayz_cli_script.to_string() + "/dayz-launcher");
    if steamexe == "default" {
        command.arg("\"\"");
    } else if steamexe == "flatpak" {
        command.arg("--steam");
        command.arg("flatpak");
    } else if steamexe == "custom" {
        command.arg("--steam");
        command.arg("\"".to_owned() + customsteamexe + "\"");
    }
    command.arg("--launch");
    command.arg("--name");
    command.arg(playername);
    command.arg("--server");
    command.arg(serverip.to_owned() + ":" + serverport);
    command.arg("--port");
    command.arg(queryport);


    let answer = format!("{:?}", command);
    println!("Launching DayZ with the following command: {:?}", answer);

    // execute command
    let output = command.output().expect("failed to execute process");
    println!("status: {}", output.status);
}

#[derive(PartialEq)]
#[derive(Debug)]
enum SteamDir {
    Default,
    Flatpak,
    Custom
}

impl fmt::Display for SteamDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SteamDir::Default => write!(f, "Default"),
            SteamDir::Flatpak => write!(f, "Flatpak"),
            SteamDir::Custom => write!(f, "Custom"),
        }
    }
}

//#[derive(Default)]
struct DayzLinuxGuiLauncher {
    playername: String, // ingame playername
    serverip: String, // server IP
    serverport: String, // server port
    queryport: String, // server query port
    steamexe: SteamDir, // path for steam executable
    customsteamexe: Option<String>, // custom directory for path for steam executable
    _customsteamexe: Option<String>, // fallback custom directory for path for steam executable
    steamexehelp: core::cell::Cell<bool>, // display help text for path for steam executable
    profilename: String, // profile name
}
    
impl Default for DayzLinuxGuiLauncher {
    fn default() -> Self {
        match return_defaults() {
            Ok(report) => {
                // read config file
                let config_dir = UserDirs::new().unwrap().home_dir().to_str().unwrap().to_string() + "/.config/dayz-linux-gui-launcher";
                let config_file = config_dir.to_string() + "/config.toml";
                let config = fs::read_to_string(config_file).expect("Unable to read file");

                // parse config file
                let mut config: toml::value::Table = toml::from_str(&config).unwrap();

                // get last_values table
                let mut main = config.get_mut("main").unwrap().as_table_mut().unwrap();
                let mut last_values = main.get_mut("last_values").unwrap().as_table_mut().unwrap();

                // get last_values
                let playername = last_values.get("playername").unwrap().as_str().unwrap().to_string();
                let serverip = last_values.get("serverip").unwrap().as_str().unwrap().to_string();
                let serverport = last_values.get("serverport").unwrap().as_str().unwrap().to_string();
                let queryport = last_values.get("queryport").unwrap().as_str().unwrap().to_string();
                let steamexe = last_values.get("steamexe").unwrap().as_str().unwrap().to_string();
                let customsteamexe = last_values.get("customsteamexe").unwrap().as_str().unwrap().to_string();
                let profilename = last_values.get("profilename").unwrap().as_str().unwrap().to_string();


                Self {
                    playername: playername,
                    serverip: serverip,
                    serverport: serverport,
                    queryport: queryport,
                    steamexe: match steamexe.as_str() {
                        "default" => SteamDir::Default,
                        "flatpak" => SteamDir::Flatpak,
                        "custom" => SteamDir::Custom,
                        _ => SteamDir::Default,
                    },
                    customsteamexe: Some(customsteamexe),
                    _customsteamexe: Some("".to_owned()),
                    steamexehelp: core::cell::Cell::new(false),
                    profilename: profilename,
                }
                          }
            Err(err) => {
                Self {
                    playername: "".to_owned(),
                    serverip: "".to_owned(),
                    serverport: "".to_owned(),
                    queryport: "".to_owned(),
                    steamexe: SteamDir::Default,
                    customsteamexe: Some("Click \"Browse\" or enter path to steam executable".to_owned()),
                    _customsteamexe: Some("".to_owned()),
                    steamexehelp: core::cell::Cell::new(false),
                    profilename: "".to_owned(),
                }
            }
         }

    }
}

fn return_defaults() -> Result<(), String> {
    // read config file
    let config_dir = UserDirs::new().unwrap().home_dir().to_str().unwrap().to_string() + "/.config/dayz-linux-gui-launcher";
    let config_file = config_dir.to_string() + "/config.toml";
    let config = fs::read_to_string(&config_file).expect("Unable to read file");

    // parse config file
    let mut config: toml::value::Table = toml::from_str(&config).unwrap();

    // get last_values table
    let mut main = config.get_mut("main").unwrap().as_table_mut().unwrap();
    
    // check if last_values table exists
    if !Path::new(&config_file).exists() {
        return Err("Config file does not exist".to_string());
    } else {
        if main.contains_key("last_values") {
            Ok(())
        } else {
            Err("last_values table does not exist".to_string())
        }
    }

}

impl eframe::App for DayzLinuxGuiLauncher {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }
    fn on_close_event(&mut self) -> bool {
        println!("on_close_event");
        // read config file
        let config_dir = UserDirs::new().unwrap().home_dir().to_str().unwrap().to_string() + "/.config/dayz-linux-gui-launcher";
        let config_file = config_dir.to_string() + "/config.toml";
        let config = fs::read_to_string(config_file).expect("Unable to read file");

        // parse config file
        let mut config: toml::value::Table = toml::from_str(&config).unwrap();

        // get last_values table
        let mut main = config.get_mut("main").unwrap().as_table_mut().unwrap();
        let mut last_values = main.get_mut("last_values").unwrap().as_table_mut().unwrap();

        // update last_values table
        last_values.insert("playername".to_string(), toml::Value::String(self.playername.to_string()));
        last_values.insert("serverip".to_string(), toml::Value::String(self.serverip.to_string()));
        last_values.insert("serverport".to_string(), toml::Value::String(self.serverport.to_string()));
        last_values.insert("queryport".to_string(), toml::Value::String(self.queryport.to_string()));
        last_values.insert("steamexe".to_string(), toml::Value::String(self.steamexe.to_string()));
        last_values.insert("customsteamexe".to_string(), toml::Value::String(self.customsteamexe.as_ref().unwrap().to_string()));
        last_values.insert("profilename".to_string(), toml::Value::String(self.profilename.to_string()));
        

        true
    }


    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        custom_window_frame(ctx, _frame, "", |ui| {
            ui.heading("DayZ Linux Launcher");
            egui::Grid::new("main-grid")
                .max_col_width(180.0)
                .show(ui, |ui| {
                    let name_label = ui.label("Name: ");
                ui.text_edit_singleline(&mut self.playername)
                    .labelled_by(name_label.id);
                ui.end_row();
                
                let serverip_label = ui.label("Server IP and port: ");
                ui.text_edit_singleline(&mut self.serverip)
                        .labelled_by(serverip_label.id);
                ui.horizontal(|ui| {
                    
                    let serverport_label = ui.label(":");
                    ui.text_edit_singleline(&mut self.serverport)
                        .labelled_by(serverport_label.id);
                });
                ui.end_row();

                let queryport_label = ui.label("Query Port: ");
                ui.text_edit_singleline(&mut self.queryport)
                    .labelled_by(queryport_label.id);
                ui.end_row();

                ui.label("Steam executable: ");
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", &mut self.steamexe))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.steamexe, SteamDir::Default, "Default");
                            ui.selectable_value(&mut self.steamexe, SteamDir::Flatpak, "Flatpak");
                            ui.selectable_value(&mut self.steamexe, SteamDir::Custom, "Custom");
                        });
                        if ui.button("?").clicked() {
                            self.steamexehelp.set(!self.steamexehelp.get());
                        }
                        if self.steamexe == SteamDir::Custom {
                            if ui.button("Browse").clicked() {
                        let _ = &mut self._customsteamexe.insert(self.customsteamexe.as_ref().unwrap().to_string());
                                self.customsteamexe = dialog::FileSelection::new("Please select a file")
                                    .title("File Selection")
                                    .path(UserDirs::new().unwrap().home_dir().to_str().unwrap())
                                    .show()
                                    .expect("Could not display dialog box");
                            };
                        }
                    });
                if self.steamexe == SteamDir::Custom {
                    if self.customsteamexe.is_none() {
                        let _ = &mut self.customsteamexe.insert(self._customsteamexe.as_ref().unwrap().to_string());
                    }
                    ui.text_edit_singleline(self.customsteamexe.as_mut().unwrap());
                }
                ui.end_row();
                
                ui.label("");
                if self.steamexehelp.get() {
                    ui.label("DEFAULT: will attempt to launch from the default steam install location.\nFLATPAK: will attempt to launch the steam flatpak.\nCUSTOM: will attempt to launch the steam executable by the user (not the DayZ executable)");
                }
                ui.end_row();

                ui.label("Profile name: ");
                ui.text_edit_singleline(&mut self.profilename);
                ui.end_row();

                if ui.button("Launch").clicked() {
                    launch(&self.playername, &self.serverip, &self.serverport, &self.queryport, &self.steamexe.to_string(), &self.customsteamexe.as_ref().unwrap());
                }
                if ui.button("Load Profile").clicked() {
                    egui::Window::new("Select Profile")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Profile name: ");
                            // get all profiles from config file
                            let mut profiles: Vec<String> = Vec::new();
                            // read config file
                            let config_dir = UserDirs::new().unwrap().home_dir().to_str().unwrap().to_string() + "/.config/dayz-linux-gui-launcher";
                            let config_file = config_dir.to_string() + "/config.toml";
                            let config = fs::read_to_string(config_file).expect("Unable to read file");

                            // parse config file
                            let mut config: toml::value::Table = toml::from_str(&config).unwrap();

                            let mut profilename = "empty".to_string();

                            // get profiles
                            let profiles_table = config.get_mut("profiles").unwrap().as_table_mut().unwrap();
                            for (key, value) in profiles_table {
                                profiles.push(key.to_string());
                            }

                            // add profiles to dropdown
                            egui::ComboBox::from_id_source("profile").selected_text(&profiles[0]).show_ui(ui, |ui| {
                                for profile in profiles.iter() {
                                    ui.selectable_value(&mut profilename, profile.to_string(), &*profile);
                                }
                            });
                        });
                    });
                }
                if ui.button("Save Profile").clicked() {
                    let config_dir = UserDirs::new().unwrap().home_dir().to_str().unwrap().to_string() + "/.config/dayz-linux-gui-launcher";
                    save_new_profile(&config_dir, &self.profilename, &self.playername, &self.serverip, &self.serverport, &self.queryport, &self.steamexe.to_string(), &self.customsteamexe.as_ref().unwrap())
                }
            });
    });
    }
}


fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, frame, title_bar_rect, title, ctx);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    title: &str,
    ctx: &egui::Context,
) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui, frame);
            ui.add_space(370.0);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {

                
                if ui.add(egui::Button::new("Load Config")).clicked() {
                    egui::Window::new("Select Profile")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Profile name: ");
                            // get all profiles from config file
                            let mut profiles: Vec<String> = Vec::new();
                            // read config file
                            let config_dir = UserDirs::new().unwrap().home_dir().to_str().unwrap().to_string() + "/.config/dayz-linux-gui-launcher";
                            let config_file = config_dir.to_string() + "/config.toml";
                            let config = fs::read_to_string(config_file).expect("Unable to read file");

                            // parse config file
                            let mut config: toml::value::Table = toml::from_str(&config).unwrap();

                            let mut profilename = "empty".to_string();

                            // get profiles
                            let profiles_table = config.get_mut("profiles").unwrap().as_table_mut().unwrap();
                            for (key, value) in profiles_table {
                                profiles.push(key.to_string());
                            }

                            // add profiles to dropdown
                            egui::ComboBox::from_id_source("profile").selected_text(&profiles[0]).show_ui(ui, |ui| {
                                for profile in profiles.iter() {
                                    ui.selectable_value(&mut profilename, profile.to_string(), &*profile);
                                }
                            });
                        });
                    });
                }
            })
            
        });
    });
}


/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    use egui::{Button, RichText};

    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        frame.close();
    }

    if frame.info().window_info.maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            frame.set_maximized(false);
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            frame.set_maximized(true);
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        frame.set_minimized(true);
    }
}

