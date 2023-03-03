use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};

#[allow(dead_code)]
pub enum SomnielItem {
    IronIngot,
    SteelIngot,
    SilverIngot,
    MtCrystal,
    CrtCrystal,
    DexCrystal,
    SpdCrystal,
    DefCrystal,
    ResCrystal,
    HitCrystal,
    AvoCrystal,
    CritAvoidCrystal,
    DragonBane,
    RiderBane,
    ArmorBane,
    FlierBane,
    CorruptedBane
}

impl SomnielItem {
    pub fn to_id(&self) -> &'static str {
        match self {
            SomnielItem::IronIngot => "G_所持_IID_てつの晶石",
            SomnielItem::SteelIngot => "G_所持_IID_はがねの晶石",
            SomnielItem::SilverIngot => "G_所持_IID_ぎんの晶石",
            SomnielItem::MtCrystal => "G_所持_IID_素材_威力",
            SomnielItem::CrtCrystal => "G_所持_IID_素材_必殺",
            SomnielItem::DexCrystal => "G_所持_IID_素材_技",
            SomnielItem::SpdCrystal => "G_所持_IID_素材_速さ",
            SomnielItem::DefCrystal => "G_所持_IID_素材_守備",
            SomnielItem::ResCrystal => "G_所持_IID_素材_魔防",
            SomnielItem::HitCrystal => "G_所持_IID_素材_命中",
            SomnielItem::AvoCrystal => "G_所持_IID_素材_回避",
            SomnielItem::CritAvoidCrystal => "G_所持_IID_素材_必避",
            SomnielItem::DragonBane => "G_所持_IID_素材_竜特効",
            SomnielItem::RiderBane => "G_所持_IID_素材_騎馬特効",
            SomnielItem::ArmorBane => "G_所持_IID_素材_重装特効",
            SomnielItem::FlierBane => "G_所持_IID_素材_飛行特効",
            SomnielItem::CorruptedBane => "G_所持_IID_素材_異形特効",
        }
    }
}

pub struct File {
    name: String,
    data: Vec<u8>,
    sommie_name: String,
}

impl File {
    pub fn from(name: &str, sommie_name: &str) -> File {
        File {
            data: {
                let f = std::fs::read(&name)
                .expect("did not find file");
                f[..f.len() - 4].to_vec()
            },
            name: name.to_string(),
            sommie_name: sommie_name.to_string(),
        }
    }

    pub fn save(&mut self) {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&self.data);
        let crc32 = hasher.finalize();
        self.data = [self.data.clone(), crc32.to_le_bytes().to_vec()].concat();
        std::fs::write(&self.name, &self.data).expect("could not write file");
    }

    pub fn set_simple_u32(&mut self, name: &str, value: u32) {
        let b = encode(name);
        let idx = self.search_data(&b).unwrap();
        let mut cursor = std::io::Cursor::new(&mut self.data[idx..]);
        cursor.write_u32::<LittleEndian>(value).unwrap();
    }

    pub fn get_simple_u32(&self, name: &str) -> u32 {
        let b = encode(name);
        let idx = self.search_data(&b).unwrap();
        let mut cursor = std::io::Cursor::new(&self.data[idx..]);
        cursor.read_u32::<LittleEndian>().unwrap()
    }

    pub fn get_bond_fragments(&self) -> u32 {
        let b = encode(&self.sommie_name);
        let idx = self.search_data(&b).unwrap() - b.len() - 1;
        let mut cursor = std::io::Cursor::new(&self.data[idx-12..]);
        cursor.read_u32::<LittleEndian>().unwrap()
    }

    pub fn set_bond_fragments(&mut self, value: u32) {
        let b = encode(&self.sommie_name);
        let idx = self.search_data(&b).unwrap() - b.len() - 1;
        let mut cursor = std::io::Cursor::new(&mut self.data[idx-12..]);
        cursor.write_u32::<LittleEndian>(value).unwrap()
    }

    pub fn get_money(&self) -> u32 {
        let b = encode(&self.sommie_name);
        let idx = self.search_data(&b).unwrap() - b.len() - 1;
        let mut cursor = std::io::Cursor::new(&self.data[idx-12-0x20..]);
        cursor.read_u32::<LittleEndian>().unwrap()
    }

    pub fn set_money(&mut self, value: u32) {
        let b = encode(&self.sommie_name);
        let idx = self.search_data(&b).unwrap() - b.len() - 1;
        let mut cursor = std::io::Cursor::new(&mut self.data[idx-12-0x20..]);
        cursor.write_u32::<LittleEndian>(value).unwrap()
    }


    fn search_data(&self, pattern: &[u8]) -> Option<usize> {
        search_bytes(&self.data, pattern)
    }
}

fn encode(text: &str) -> Vec<u8> {
    let mut b: Vec<u8> = text.encode_utf16().
        map(|i|
            vec![i as u8, (i >> 8) as u8]
        )
        .flatten()
        .collect();
    if b.len() % 2 == 1 {
        // odd length
        b.push(0);
    }
    b
}

fn search_bytes(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    let idx = bytes
        .windows(pattern.len())
        .position(|window| window == pattern);
    if let Some(idx) = idx {
        Some(idx + pattern.len() + 1)
    } else {
        None
    }
}