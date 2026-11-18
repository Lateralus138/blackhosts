use std::{
    io::{
        stdin,
        Write
    },
    process::exit,
    str,
    fs::{read_to_string,File},
    env::{
        args,
        consts::OS,
        var
    }
};
use md5::compute;
use curl::{easy::Easy};
use openssl as _;
fn main() {
    let arg_vector: Vec<String> = args().skip(1).collect();
    let (arg_vector_new, affirm_bool, quiet_bool) = parse_arguments(&arg_vector);

    let mut file_string = String::from("https://cdn.jsdelivr.net/gh/StevenBlack/hosts@master/");
    let mut file_file = String::new();
    if !arg_vector_new.is_empty() {
        let slice: String = arg_vector_new.into_iter().collect::<Vec<String>>().join(" ");
        file_file.push_str("alternates/");
        file_file.push_str(&slice);
        file_file.push('/');
    }
    file_file.push_str("hosts");
    file_string.push_str(&file_file);
    let file_string = file_string;
    let url_string: &str = &file_string;
    let data_vector = download(url_string);
    let data_string = match str::from_utf8(&data_vector[..]) {
        Ok(output) => output,
        Err(_) => {
            if ! quiet_bool {
                println!("\n {}.\n",red_bold("Invalid UTF-8 input."));
            }
            exit(1);
        }
    };
    if data_string.starts_with("Couldn't find the requested file") {
        if ! quiet_bool {
            println!("\n {}\n",red_bold(data_string));
        }
        exit(2);
    }
    let hosts_string = hosts_path();
    let hosts: &str = &hosts_string;
    let data_hosts = match read_to_string(hosts) {
        Ok(data) => data,
        Err(error) => {
            if ! quiet_bool {
                println!(
                    "\n {}: {}.\n",
                    red_bold(&error.to_string()),
                    yellow_bold(hosts)
                );
            }
            exit(3);
        }
    };
    if data_vector[..] == *data_hosts.as_bytes() {
        if ! quiet_bool {
            println!(
                "\n {}.\n",
                green_bold("Your hosts file's MD5 match and does not need to be updated")
            );
        }
        exit(0);
    }
    if ! quiet_bool && ! affirm_bool {
        let data_vector_md5 = compute(&data_vector);
        let data_hosts_md5 = compute(&data_hosts);
        println!(
            "\n Your hosts file does not match the new file:\n\n {}:\n MD5: {:x}\n\n {}:\n MD5: {:x}\n",
            green_bold(hosts),
            data_hosts_md5,
            green_bold(url_string),
            data_vector_md5
        );
        println!(
            " Would you like to update your {} file:\n ([y]es,[n]o)?\n",
            yellow_bold(hosts)
        );
        let mut input_string = String::new();
        while input_string.trim().is_empty() {
            if stdin().read_line(&mut input_string).is_err() {
                println!("\n {}.\n",red_bold("Invalid input"));
                exit(4);
            }
        }
        if ! is_yes(input_string.trim()) {
            println!(
                "\n You refused to update your {} file.\n",
                yellow_bold(hosts)
            );
            exit(0);
        }
    }
    update_hosts(hosts, &data_vector, quiet_bool);
}

fn colored(color_code: &str, text: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m",color_code,text)
}
fn red_bold(text: &str) -> String {
    colored("1;31",text)
}
fn green_bold(text: &str) -> String {
    colored("1;32",text)
}
fn yellow_bold(text: &str) -> String {
    colored("1;33",text)
}
fn blue_bold(text: &str) -> String {
    colored("1;34",text)
}
fn magenta_bold(text: &str) -> String {
    colored("1;35",text)
}

fn help(code: u8, msg: &str) -> ! {
    let mesg: String;
    let extra_line: &str;
    if code == 0 {
        mesg = String::new();
        extra_line = "";
    } else {
        mesg = red_bold(msg);
        extra_line = "\n";
    }
    let opts_help_txt = green_bold("OPTIONS");
    let usage_help_txt = magenta_bold("USAGE");
    let alt_help_txt = blue_bold("ALTERNATE");
    let err_help_txt = red_bold("ERRORS");
    print!(
        "\n blackhosts - 'hosts' installer\n \
        Install custom 'hosts' files from the\n \
        'StevenBlack' GitHub repository.\n\
        \n \
        @{}:\n\
        \tblackhosts\n\
        \tblackhosts [{}...]\n \
        \tblackhosts {}...\n \
        \tblackhosts [{}...] {}...\n\n \
        @{}:\n\
        \t-h,--help\tThis help screen.\n\
        \t-q,--quiet\tDon't print output -\n\
        \t\t\tnon-verbose. Assumes '-y'\n\
        \t\t\tto skip affirmation.\n\
        \t-y,--yes\tSkip affirmation -\n\
        \t\t\tauto-yes.\n\n \
        @{}:\n \
        \tAlternate hosts file from list in\n\
        \t'alternates' directory at the repository.\n\n \
        @{}:\tExit Codes\n\
        \t0\tNo errors.\n\
        \t1\tRemote 'hosts' is not properly\n\
        \t\tUTF encoded.\n\
        \t2\tRemote 'hosts' file not found.\n\
        \t3\tCould not read local 'hosts'\n\
        \t\tfile.\n\
        \t4\tError while reading user\n\
        \t\tinput.\n\
        \t5\tCould not open local 'hosts'\n\
        \t\tfile.\n\
        \t6\tCould not write local 'hosts'\n\
        \t\tfile.\n\
        {} {}{}\n",
        usage_help_txt,opts_help_txt, alt_help_txt, opts_help_txt,
        alt_help_txt,opts_help_txt,alt_help_txt,
        err_help_txt,
        extra_line,mesg,extra_line);
    exit(code as i32);
}

fn parse_arguments(arg_vector: &[String]) -> (Vec<String>, bool, bool) {
    let mut arg_vector_new: Vec<String> = Vec::with_capacity(arg_vector.len());
    let mut affirm_bool: bool = false;
    let mut quiet_bool: bool = false;
    for arg_iter in arg_vector {
        if arg_iter.eq_ignore_ascii_case("-h") || arg_iter.eq_ignore_ascii_case("--help") {
            help(0,"");
        }
        if arg_iter.eq_ignore_ascii_case("-y") || arg_iter.eq_ignore_ascii_case("--yes") {
            affirm_bool = true;
        } else if arg_iter.eq_ignore_ascii_case("-q") || arg_iter.eq_ignore_ascii_case("--quiet") {
            quiet_bool = true;
            affirm_bool = true;
        } else {
            arg_vector_new.push(arg_iter.clone());
        }
    }
    (arg_vector_new, affirm_bool, quiet_bool)
}

fn is_yes(input_string: &str) -> bool {
    input_string.eq_ignore_ascii_case("y") || input_string.eq_ignore_ascii_case("yes")
}

fn download(url_string: &str) -> Vec<u8> {
    let mut data_api = Easy::new();
    data_api.url(url_string).expect("valid request URL");
    let mut data_vector = Vec::new();
    {
        let mut data_transfer = data_api.transfer();
        data_transfer.write_function(|data| {
            data_vector.extend_from_slice(data);
            Ok(data.len())
        }).expect("register write callback");
        data_transfer.perform().expect("perform request");
    }
    data_vector
}

fn hosts_path() -> String {
    match OS {
        "windows" => {
            let mut win_path = match var("SystemRoot") {
                Ok(value) => value,
                Err(_) => String::from("C:\\Windows")
            };
            win_path.push_str("\\System32\\drivers\\etc\\hosts");
            win_path
        },
        "macos" => String::from("/private/etc/hosts"),
        _ => String::from("/etc/hosts")
    }
}

fn update_hosts(hosts: &str, data_vector: &[u8], quiet_bool: bool) -> ! {
    let mut hosts_file = match File::create(hosts) {
        Ok(hosts_file) => hosts_file,
        Err(_) => {
            if ! quiet_bool {
                println!(
                    "\n {}: {}.\n You might possibly need to run this as administrator or root.\n",
                    red_bold("Could not open"),
                    yellow_bold(hosts)
                );
            }
            exit(5);
        }
    };
    match hosts_file.write_all(data_vector) {
        Ok(_) => {
            if ! quiet_bool {
                println!(
                    "\n {} {}.\n",
                    yellow_bold(hosts),
                    green_bold("successfully updated")
                );
            }
            exit(0);
        },
        Err(_) => {
            if ! quiet_bool {
                println!(
                    "\n {}: {}.\n",
                    red_bold("Could not update"),
                    yellow_bold(hosts)
                );
            }
            exit(6);
        }
    }
}
