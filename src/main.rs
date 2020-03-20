mod graphics;
use graphics::parser::DWScript;

fn main() {
    let mut script = DWScript::new("gallery_test");
    script.do_parse();
}
