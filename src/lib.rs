pub mod pacman {
    fn __invoke_pacman(args: &[&str], hold: bool) -> Result<(), std::io::Error> {
        use std::process::Command;

        let child = Command::new("/bin/pacman")
            .args(args.to_vec())
            .spawn();
        let child = &mut child?;

        if hold {
            let _ = child.wait();
        }

        Ok(())
    }

    // pub fn try_invoke_pacman(args: Vec<String>) -> Result<(), std::io::Error> {
    //     __invoke_pacman(args, false)
    // }

    pub fn invoke_pacman(args: &[&str]) -> Result<(), std::io::Error> {
        __invoke_pacman(args, true)
    }
}

