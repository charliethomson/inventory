const SKU_MASK: u64 = 0xFFFFFFFC00000000;
const ID_MASK: u64 = 0x3FFFFFFFF;

/* Top 30 bytes is SKU, bottom 34 is the item id */
pub struct Item(u64);
impl Item {
    pub fn new(sku: u64, id: u64) -> Self {
        Item((sku << 34) | id)
    }

    pub fn try_from(s: String) -> Result<Self, String> {
        let mut parts = s.split('-');
        let sku = match parts.next().and_then(|sku| sku.parse::<u64>().ok()) {
            Some(sku) => sku,
            None => return Err(format!("Failed to get sku from {}", s)),
        };
        let id = match parts.next().and_then(|id| id.parse::<u64>().ok()) {
            Some(id) => id,
            None => return Err(format!("Failed to get id from {}", s)),
        };
        Ok(Self::new(sku, id))
    }

    pub fn sku(&self) -> u64 {
        (self.0 & SKU_MASK) >> 34
    }

    pub fn id(&self) -> u64 {
        self.0 & ID_MASK
    }
}
impl ToString for Item {
    fn to_string(&self) -> String {
        format!("{}-{}", self.sku(), self.id())
    }
}
