pub struct BwImg {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>
}

/*impl BwImg {
    pub fn to_string() -> String {
        let result = String::new();
        for x in 0..width {
            if pixels[x] == 0 {
                result.push('.')
            } else {
                result.push('#')
            }
        }
        return result;
    }
}*/