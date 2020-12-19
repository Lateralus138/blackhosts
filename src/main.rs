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
    },
    collections::VecDeque
};
use md5::compute;
use curl::{easy::Easy};
use regex::Regex;
use recolored::Colorize;
fn main() {
    let mut arg_vector: Vec<String> = args().collect();
    arg_vector.remove(0);
    let mut arg_vector_new: VecDeque<String> = VecDeque::new();
    let mut affirm_bool: bool = false;
    let mut quiet_bool: bool = false;
    let mut skip_bool: bool = false;
    let affirm_regex = Regex::new(r"^-([yY]|-[yY][eE][sS])$").unwrap();
    let quiet_regex = Regex::new(r"^-([qQ]|-[qQ][uU][iI][eE][tT])$").unwrap();
    let help_regex = Regex::new(r"^-([hH]|-[hH][eE][lL][pP])$").unwrap();
    let help_func = |code: u8, msg: &str| {
        let mesg;
        let extra_line: &str;
        if code == 0 {
            mesg = "".black();
            extra_line = "";
        } else {
            mesg = msg.red().bold();
            extra_line = "\n";
        }
        let opts_help_txt = String::from("OPTIONS").green().bold();
        let usage_help_txt = String::from("USAGE").magenta().bold(); 
        let alt_help_txt = String::from("ALTERNATE").blue().bold();
        let err_help_txt = String::from("ERRORS").red().bold();
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
    };
    if &arg_vector.len() != &0 {
        for arg_iter in &arg_vector {
            if help_regex.is_match(&arg_iter) {
                skip_bool = true;
                help_func(0,"");
            }
            if affirm_regex.is_match(&arg_iter) {
                skip_bool = true;
                affirm_bool = true;
            }
            if quiet_regex.is_match(&arg_iter) {
                skip_bool = true;
                quiet_bool = true;
                affirm_bool = true;
            }
            if ! skip_bool {
                arg_vector_new.push_back(
                    (&arg_iter as &str).to_string()
                );
            } else {
                skip_bool = false;
            }
        }
    }
    let mut data_api = Easy::new();
    let mut  data_vector = Vec::new();
    let mut file_string = String::from("https://cdn.jsdelivr.net/gh/StevenBlack/hosts@master/");
    let mut file_file = String::new();
    if &arg_vector_new.len() != &0 {
        let slice: String = arg_vector_new.into_iter().collect::<Vec<String>>().join(" ");
        file_file.push_str("alternates/");
        file_file.push_str(&slice);
        file_file.push_str("/");
    }
    file_file.push_str("hosts");
    file_string.push_str(&file_file);
    let file_string = file_string;
    let url_string: &str = &file_string;
    data_api.url(url_string).unwrap();
    {
        let mut data_transfer = data_api.transfer();
        data_transfer.write_function(|data| {
             data_vector.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        data_transfer.perform().unwrap();
    }
    let data_string = match str::from_utf8(&data_vector[..]) {
        Ok(output) => output,
        Err(_) => {
            if ! quiet_bool {
                println!("\n {}.\n","Invalid UTF-8 input.".red().bold());
            }
            exit(1);
        }
    };
    let error_regex = Regex::new(r"^Couldn't find the requested file").unwrap();
    if error_regex.is_match(data_string) {
        if ! quiet_bool {
            println!("\n {}\n",data_string.red().bold());
        }
        exit(2);
    }
    let borrow: String;
    let mut win_path = String::new();
    let hosts: &str = match OS {
        "windows" => {
            let sys_root: &str = match var("SystemRoot") {
                Ok(value) => {
                    borrow = value;
                    &borrow[..]
                },
                Err(_) => "C:\\Windows"
            };
            win_path.push_str(sys_root);
            win_path.push_str("\\System32\\drivers\\etc\\hosts");
            &win_path[..]
        },
        "macos" => "/private/etc/hosts",
        _ => "/etc/hosts"
    };
    let data_hosts = match read_to_string(hosts) {
        Ok(data) => data,
        Err(error) => {
            if ! quiet_bool {
                println!(
                    "\n {}: {}.\n",
                    error.to_string().red().bold(),
                    hosts.yellow().bold()
                );
            }
            exit(3);
        }
    };
    let data_vector_md5 = compute(&data_vector);
    let data_hosts_md5 = compute(&data_hosts);
    if data_vector_md5 == data_hosts_md5 {
        if ! quiet_bool {
            println!(
                "\n {}.\n",
                "Your hosts file's MD5 match and does not need to be updated"
                    .green()
                    .bold()
                );
        }
        exit(0);
    }
    if ! quiet_bool {
        if ! affirm_bool {
            println!(
                "\n Your hosts file does not match the new file:\n\n {}:\n MD5: {:x}\n\n {}:\n MD5: {:x}\n",
                hosts.green().bold(),
                data_hosts_md5,
                url_string.green().bold(),
                data_vector_md5
            );
            println!(
                " Would you like to update your {} file:\n ([y]es,[n]o)?\n",
                hosts.yellow().bold()
            );
            let yes_regex = Regex::new(r"^([yY]|[yY][eE][sS])$").unwrap();
            let mut input_string = String::new();
            while input_string.trim().is_empty() {
                if stdin().read_line(&mut input_string).is_err() {
                    println!("\n {}.\n","Invalid input".red().bold());
                    exit(4);
                }
            }
            if ! yes_regex.is_match(&input_string.trim()) {
                println!(
                    "\n You refused to update your {} file.\n",
                    hosts.yellow().bold()
                );
                exit(0);
            }
        }
    }
    let mut hosts_file = match File::create(hosts) {
        Ok(hosts_file) => hosts_file,
        Err(_) => {
            if ! quiet_bool {
                println!(
                    "\n {}: {}.\n You might possibly need to run this as administrator or root.\n",
                    "Could not open".red().bold(),
                    hosts.yellow().bold()
                );
            }
            exit(5);
        }
    };
    match hosts_file.write_all(&data_vector.to_owned()) {
        Ok(_) => {
            if ! quiet_bool {
                println!(
                    "\n {} {}.\n",
                    hosts.yellow().bold(),
                    "successfully updated".green().bold()
                );
            }
            exit(0);
        },
        Err(_) => {
            if ! quiet_bool {
                println!(
                    "\n {}: {}.\n",
                    "Could not update".red().bold(),
                    hosts.yellow().bold()
                );
            }
            exit(6);
        }
    }
}
