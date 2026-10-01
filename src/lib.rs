mod expanded;
mod navigation;
mod outfit;
mod obody_trace;
#[skyline::main(name = "engage_outfit_enhancements")]
pub fn main() {
    if !expanded::supported() || !outfit::supported() || !navigation::supported() {
        let _ = horizon_svc::output_debug_string("[FEE Outfit Enhancements] unsupported executable; no hooks installed\n");
        return;
    }
    expanded::install();
    outfit::install();
    navigation::install();
    obody_trace::install();
}
