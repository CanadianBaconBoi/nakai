fn main() {
    match std::env::consts::OS {
        "windows" | "osx" => {
            panic!("WARNING! While this program *may* work correctly on Garbage, we offer NO SUPPORT WHATSOEVER.\nThis program does, however, run perfectly fine in Docker / on WSL.\nPlease do not open any GitHub issues if you disable this compile guard. You are ON YOUR OWN");
        }

        _ => {
            //Smooth sailing
        }
    }

    //TODO: I love building, build stuff here eventually and stop writing CSS like it's 2008
}