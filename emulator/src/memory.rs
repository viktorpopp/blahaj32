pub struct Memory {
    inner: Vec<u8>,
}

impl Memory {
    pub fn new(capacity: u32) -> Self {
        Self {
            inner: vec![0; capacity as usize],
        }
    }

    pub fn write_at(&mut self, at: u32, data: &[u8]) {
        let at = at as usize;
        self.inner[at..at + data.len()].copy_from_slice(data);
    }

    pub fn read_u32(&self, at: u32) -> u32 {
        let at = at as usize;
        let end = at + 4;

        if end > self.inner.len() {
            panic!(
                "tried to access more memory than available (requested range {}..{}, memory capacity is {})",
                at,
                end,
                self.inner.len()
            );
        }

        u32::from_le_bytes(self.inner[at..end].try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn out_of_bounds_read() {
        let memory = Memory::new(0);
        memory.read_u32(8);
    }

    #[test]
    fn read_write() {
        let mut memory = Memory::new(8);
        let bytes = [42, 67, 68, 69];

        memory.write_at(1, &bytes);

        let val = memory.read_u32(1);
        let expected = u32::from_le_bytes(bytes);

        assert_eq!(val, expected);
    }
}
