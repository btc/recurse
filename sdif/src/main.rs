use opencv::features2d::{SIFT, Feature2DTrait};

fn main() -> opencv::Result<(), anyhow::Error> {
    let mut sift = SIFT::create(1, 1, 1.0, 1.0, 1.0)?;
    let mut keypoints = Vec::new();
    let mask = Vec::new();
    sift.detect(image, &mut keypoints, &mask)?;
    Ok(())
}