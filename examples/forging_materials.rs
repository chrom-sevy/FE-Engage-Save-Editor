use fe_engage_save_editor::{SaveFile, SomnielItem};

fn main() {
    let mut f = SaveFile::from_path("Manual0", "efficacy");
    let count = f.get_simple_u32(SomnielItem::IronIngot.to_id());
    println!("count: {}", count);
    f.set_simple_u32(SomnielItem::IronIngot.to_id(), count + 1);
    f.save();
}