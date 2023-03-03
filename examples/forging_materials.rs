use fe_engage_save_editor::{File, SomnielItem};

fn main() {
    let mut f = File::from("Manual0");
    let count = f.get_simple_u32(SomnielItem::IronIngot.to_id());
    println!("count: {}", count);
    f.set_simple_u32(SomnielItem::IronIngot.to_id(), count + 1);
    f.save();
}