mod readme;
use readme::Readme;

fn main() {
    let mut readme = Readme::new();
    readme.survey();
    readme.save();
}
