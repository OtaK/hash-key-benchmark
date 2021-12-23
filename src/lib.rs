
#[derive(Debug, Default)]
pub struct ByteMap<'a> {
    indexes: Vec<&'a [u8]>,
    values: Vec<&'a [u8]>,
}

impl<'a> ByteMap<'a> {
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

    pub fn get<K: AsRef<[u8]>>(&self, k: K) -> Option<&[u8]>{
        self.indexes.iter().position(|k1| *k1 == k.as_ref()).map(|index| self.values[index])
    }

    #[inline(always)]
    pub fn set<K: 'a + AsRef<[u8]>, V: 'a + AsRef<[u8]>>(&'a mut self, k: &'a K, v: &'a V) -> &mut Self {
        if let Some(found_index) = self.indexes.iter().position(|k1| *k1 == k.as_ref()) {
            self.values[found_index] = v.as_ref();
        } else {
            self.indexes.push(k.as_ref());
            self.values.push(v.as_ref());
        }

        self
    }
}

impl<S: AsRef<[u8]>> std::ops::Index<S> for ByteMap<'_> {
    type Output = [u8];

    fn index(&self, index: S) -> &Self::Output {
        &self.get(index).unwrap()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn stuff_works() {
        let mut bytemap = super::ByteMap::new();
        let _t = &bytemap.get(b"key1");
        let bytemap = bytemap
            .set(b"customkey1", b"customvalue1")
            .set(b"customkey2", b"customvalue2");

        bytemap.set(b"customkey3", b"customvalue3");
    }
}
