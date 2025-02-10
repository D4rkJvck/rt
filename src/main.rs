use {
    rt::{
        welcome,
        Result,
        Scene,
    },
    std::io,
};

/// Promps the user to choose a scene by typing a ID corresponding
/// to a scene and Display the scene on
fn main() -> Result<()> {
    welcome();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse() {
            Err(e) => {
                eprintln!("{e}\nPlease choose a valid integer!");
                continue;
            }
            Ok(id) => match Scene::gen(id) {
                Err(e) => {
                    eprintln!("{e}Try again");
                    continue;
                }
                Ok(mut scene) => return scene.draw(),
            },
        };
    }
}
