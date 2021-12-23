#[derive(Debug, Default)]
pub struct ByteMap<'a> {
    indexes: Vec<&'a [u8]>,
    values: Vec<&'a [u8]>,
}

impl ByteMap<'_> {
    pub fn new() -> Self {
        Self {
            indexes: vec![
                b"key1",
                b"key2",
                b"key3",
                b"key4",
                b"key5",
            ],
            values: vec![
                b"value1",
                b"value2",
                b"value3",
                b"value4",
                b"value5",
            ],
        }
    }
}


impl<S> std::ops::Index<S> for ByteMap<'_> where S: AsRef<[u8]> {
    type Output = [u8];
    fn index(&self, k: S) -> &Self::Output {
        let index = self.indexes.iter().position(|k1| *k1 == k.as_ref()).unwrap();
        self.values[index]
    }
}


fn test() {
    let bytemap = ByteMap::new();
    let _t = &bytemap[b"key1"];
    let mut hashmap = std::collections::HashMap::new();
    hashmap.insert(b"key1", b"value1");
    hashmap.insert(b"key2", b"value2");
    hashmap.insert(b"key3", b"value3");
    hashmap.insert(b"key4", b"value4");
    hashmap.insert(b"key5", b"value5");


}
