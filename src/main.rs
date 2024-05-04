//! This is a very poorly written AUR helper in rust.
//! Would one ever want to use this? of course not!

#[cfg(not(feature = "let_me_compile"))]
compile_error!("Repeat after me: DO NOT use this as your daily AUR helper.\nI have forbidden you from doing so, in the readme.\nIn case you **REALLY, REALLY** want to do so, run:\ncargo b --features let_me_compile");

mod args;
mod filesystem;
use std::sync::Mutex;

lazy_static::lazy_static!(
    static ref HOME_DIR: Mutex<String> = Mutex::new(Default::default());
    static ref REQUESTED_MAKEPKG_ARGS: Mutex<String> = Mutex::new(String::from("-si")); // TODO:
);

// fn print_help() {
//     println!("figure it out");
// }

/// this is slow lol (unneeded allocs)
fn __format_package_to_aur(pkg: &str) -> String {
    format!("https://aur.archlinux.org/{}.git", pkg)
}

fn _init_fetch_home_dir() -> Result<(), std::io::Error> {
    // TODO: error  handling
    *HOME_DIR.lock().unwrap() = format!("{}/.cache/nyaur", std::env::var("HOME").unwrap());

    Ok(())
}

fn fs_makepkg(pkg: &str) -> Result<(), std::io::Error> {
    use std::process::Command;
    pub use std::io::{Error, ErrorKind};

    let home = HOME_DIR.lock().unwrap();
    dbg!(&home);
    let requested_makepkg_args = REQUESTED_MAKEPKG_ARGS.lock().unwrap().to_string(); // wtf?
    let build_dir = format!("{home}/{pkg}");
    dbg!(&build_dir);
    let makepkg = Command::new("/bin/makepkg")
        .args([requested_makepkg_args])
        .current_dir(build_dir)
        .spawn();
    dbg!(&makepkg);

    // TODO: this is repetitive (see clone_package). why not centralize this?
    if let Err(err) = &makepkg {
        return Err(Error::new(ErrorKind::NotFound, format!("meowmeow no makepkg installed?\n{err}\npacman -S base-devel --needed")));
    }

    let exit_code = makepkg?.wait()?.code().unwrap_or(-1);
    if exit_code != 0 {
        return Err(Error::new(ErrorKind::Other, format!("makepkg exited irregularly! (code {exit_code})")))
    }

    Ok(())
}

fn clone_package(pkg: &str) -> Result<(), std::io::Error> {
    use std::process::Command;
    pub use std::io::{Error, ErrorKind};

    let home_dir = format!("{}/{}", HOME_DIR.lock().unwrap(), pkg);
    let pkg = __format_package_to_aur(pkg);

    let git = Command::new("/bin/git")
        .args(["clone", &pkg, &home_dir])
        .spawn();

    if let Err(err) = &git {
        return Err(Error::new(ErrorKind::NotFound, format!("meowmeow no Git installed?\n{err}\npacman -S base-devel git --needed")));
    }

    // already checked. should never fail. (we returned Err above.)
    // if it does, please alert the world that you have successfully broken computing and that no
    // bit manuplation is safe.
    let exit = git?.wait().unwrap(); // we can be sure this value is not an Err.
    let exit_code = exit.code().unwrap_or(-1);
    if exit_code != 0 {
        if exit_code != 128 { // 128 for alr exists. next.
            return Err(Error::new(ErrorKind::Other, format!("Git exited irregularly! (code {exit_code})")))
        }
    }


    Ok(())
}

fn main() -> std::io::Result<()> {
    use std::env;
    use std::process::Command;

    let args: Vec<String> = env::args().collect();

    let privilege = if let Ok(ok) = filesystem::determine_privilege_esclation_tactic() {
        ok
    } else {
        panic!("Can't find privilege esclation. Do you have sudo or doas?");
    };

    if args.len() == 1 { // 1 because first arg is always our location.
        let pacman = Command::new(&privilege)
            .args(["pacman", "-Syu"])
            .spawn();

        let _ = pacman?.wait();
        println!("TODO: update AUR");
        return Ok(());
    }
    let (args, pkgs) = args::parse_args(args);

    if args.should_exit {
        return Ok(());
    }

    if !args.ignore_db_lock {
        filesystem::freeze_until_pacman_unlocks_db(true);
    }

    if let Err(err) = _init_fetch_home_dir() {
        panic!("{err}");
    }

    if args.pacman_operation {
        // damn ownership; we're getting it again.
        let mut a: Vec<String> = env::args().collect();
        a.remove(0);

        let pacman = Command::new(&privilege)
            .arg("/bin/pacman")
            .args(a)
            .spawn();

        let _ = pacman?.wait();
        return Ok(());
    }

    // add aur pkgs to an "array" so incase it cant be found it wont be makepkg'd
    let mut aur_packages: Vec<&str> = Vec::new();

    // download phase
    for pkg in &pkgs {
        let a = clone_package(&pkg);
        match a {
            Ok(_) => println!("Cloned package {pkg}"),
            Err(err) => eprintln!("Well this aint good, no? {err}"),
        }

        // check for if we cloned empty repo
        // if so, then it doesn't exist. delete the folder and invoke pacman.
        if let Err(err) = filesystem::pkg_has_pkgbuild(HOME_DIR.lock().unwrap().to_string(), pkg) {
            // TODO: implement
            println!("{pkg} doesnt exist in aur: {err}");
            aur_packages.push(pkg);
        }
    }

    // makepkg phase
    for pkg in &aur_packages {
        match fs_makepkg(pkg) {
            Ok(_) => println!("Successfully built {pkg}"),
            Err(err) => eprintln!("Well... this aint good, no? {err}"),
        }
    }

    // then install remaining pacman pkgs
    dbg!(&aur_packages);
    for pkg in &aur_packages {
        use nyaur::pacman::invoke_pacman;

        println!("{pkg}");
        if let Err(err) = invoke_pacman(&["-S", pkg]) {
            eprintln!("failed to install package {pkg} from pacman because {err}");
        }
    }

    Ok(())
}

