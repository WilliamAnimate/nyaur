// #[derive(Debug)]
#[allow(unused)]
#[derive(Debug)]
// TODO: drop this monstroncity and use an enum
pub struct Args {
    pub download: bool,
    pub pacman_operation: bool,
    pub should_exit: bool,
    pub should_cleanup: bool,
    pub ignore_db_lock: bool,
}

pub fn parse_args(args: Vec<String>) -> (Args, Vec<String>) {
    let mut ret = Args {
        download: false,
        pacman_operation: false,
        should_exit: false,
        should_cleanup: false,
        ignore_db_lock: false,
    };
    let mut pkgs: Vec<&String> = Vec::new(); // possibly. if -R or -S.

    let mut index = 2;
    let mut skip_next = false;

    for arg in &args[1..] {
        if skip_next {
            println!("skip");
            skip_next = false; // reset value
            continue;
        }
        match arg.as_str() {
            "-S" => {
                if &args.len() == &index {
                    println!("you need option after -S");
                    ret.should_exit = true;
                    break;
                }
                let mut presumed_pkgs: Vec<&String> = Vec::new();
                loop {
                    index += 1;
                    if args.len() == index - 1 {
                        break;
                    }
                    // println!("pushing {}", &args[index - 1]);
                    presumed_pkgs.push(&args[index - 1]);
                }
                pkgs = presumed_pkgs;
                break;
            },
            "-C" => {
                ret.should_exit = true;

                let home_dir = &crate::HOME_DIR.lock().unwrap();
                println!("Cleaning {home_dir}");
                if let Err(err) = crate::filesystem::clean_nyaur_working_dir(home_dir) {
                    eprintln!("Failed to clean: {}", err);
                }
                // std::process::Command::new("/bin/rm")
                //     .args(["-rf", "~/.cache/aurd"]); // scary!
            }
            _ => ret.pacman_operation = true,
        }
        index += 1;
    }

    (ret, pkgs.iter().map(|s| s.to_string()).collect())
}
// nice
