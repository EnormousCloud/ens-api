pub struct HexReader {
    pub data: Vec<u8>,
    // mutable data offset
    pub data_offset: usize,
}

impl HexReader {
    pub fn new(src: Vec<u8>) -> anyhow::Result<Self> {
        Ok(Self {
            data: src.clone(),
            data_offset: 0,
        })
    }

    fn has_data(&self) -> bool {
        self.data.len() > self.data_offset
    }

    fn next32(&mut self) -> String {
        let hex_str = hex::encode(&self.data);
        let offs: usize = 2 * self.data_offset;
        let res: String = hex_str.chars().skip(offs).take(64).collect();
        self.data_offset += 32;
        res
    }

    // pop meta data as text
    pub fn text(&mut self) -> String {
        let _hex_size = self.next32();
        let mut s = String::from("");
        while self.has_data() {
            let nextword = self.next32();
            let bts: Vec<u8> = hex::decode(nextword).unwrap();
            bts.iter().filter(|ch| **ch != 0).for_each(|ch| {
                if *ch > 0x1F && *ch < 0x80 {
                    s.push(*ch as char);
                }
            });
        }
        s
    }
}
