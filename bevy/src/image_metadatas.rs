use bevy::math::Vec2;

#[derive(Clone, Debug)]
pub struct ImageMetadata {
    pub file_name: String,
    pub dims: Vec2,
    pub title: String,
    pub author: String,
}

pub fn select_random_image() -> ImageMetadata {
    let images: Vec<ImageMetadata> = vec![ImageMetadata {
        file_name: String::from("23364494180_b99e33a74d_k"),
        dims: Vec2::new(2048., 1135.) * 0.33,
        title: String::from("title"),
        author: String::from("author"),
    }];
    return images[0].clone();
}
