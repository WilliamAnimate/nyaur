use std::path::Path;
use std::fs::remove_dir_all;

pub fn clean_nyaur_working_dir(nyaur_path: &str) -> Result<(), std::io::Error> {
    if let Err(err) = remove_dir_all(nyaur_path) {
        return Err(err); // 💀
    }

    Ok(())
}

pub fn pkg_has_pkgbuild(nyaur_path: String, pkg: &str) -> Result<bool, std::io::Error> {
    let fmt = format!("{nyaur_path}/{pkg}/PKGBUILD");
    if Path::new(&fmt).exists() {
        return Ok(true);
    }
    Ok(false)
}

/// okay fine. this thing is better named "remove package in nyaur" lmao
pub fn delete_folder_in_folder(first_folder: String, second_folder: &str) -> Result<(), std::io::Error> {
    let folder = format!("{first_folder}/{second_folder}");
    if let Err(err) = remove_dir_all(folder) {
        return Err(err);
    }

    Ok(())
}

pub fn determine_privilege_esclation_tactic() -> Result<String, std::io::Error> {
    pub use std::io::{Error, ErrorKind};

    if Path::new("/bin/sudo").exists() {
        Ok("sudo".to_string())
    } else if Path::new("/bin/doas").exists() {
        Ok("doas".to_string())
    } else {
        return Err(Error::new(ErrorKind::NotFound, "No suitable privilege esclation tool found"))
    }
}

pub fn is_pacman_is_in_use() -> bool {
    if Path::new("/var/lib/pacman/db.lck").exists() {
        return true;
    }

    false
}

/// spinlock at home
pub fn freeze_until_pacman_unlocks_db(print_msg: bool) {
    use std::{thread::sleep, time::Duration};

    let mut is_first_time = false;
    // hey mom, can we have spinlock?
    // we have spinlock at home
    // spinlock at home:
    loop {
        if !is_pacman_is_in_use() {
            break;
        }

        if print_msg && !is_first_time {
            println!("> Pacman is currently in use!\nrm /var/lib/pacman/db.lck if this is in error.");
        }
        is_first_time = true;
        sleep(Duration::from_secs(1));
    }
} // nice
