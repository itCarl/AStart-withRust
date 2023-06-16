use egui::Color32;

pub fn generate_grid() -> [Color32;17*17]{
    let mut array: [Color32; 17*17] = [Color32::GRAY;17*17];
    for x in 0..array.len(){
            array[x] = Color32::GRAY;
    }
    array
}