use crate::images::bwImg::BwImg;

pub fn generateImg() -> BwImg {
    return BwImg { width: 10, height: 10, pixels: [0,0,0,0].to_vec() }
}
