use super::{get_class_list, Converter};
use crate::parser::{ImageEntry, NDJSONData};
use std::collections::HashMap;

pub struct YoloConverter;

impl YoloConverter {
    pub fn new() -> Self {
        Self
    }

    fn create_data_yaml(&self, data: &NDJSONData) -> String {
        let class_names = get_class_list(data);

        let mut yaml = String::new();
        yaml.push_str("path: .\n");
        yaml.push_str("train: train/images\n");
        yaml.push_str("val: valid/images\n");
        yaml.push_str("test: test/images\n");
        yaml.push_str(&format!("nc: {}\n", class_names.len()));
        yaml.push_str("names:\n");

        for (i, name) in class_names.iter().enumerate() {
            yaml.push_str(&format!("  {}: {}\n", i, name));
        }

        yaml
    }

    fn create_detection_label(&self, img: &ImageEntry) -> String {
        img.get_bboxes()
            .iter()
            .map(|bbox| {
                format!(
                    "{} {:.6} {:.6} {:.6} {:.6}",
                    bbox.class_id, bbox.x, bbox.y, bbox.width, bbox.height
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Converter for YoloConverter {
    fn convert(&self, data: &NDJSONData, downloaded_images: &HashMap<String, Vec<u8>>) -> HashMap<String, Vec<u8>> {
        let mut files: HashMap<String, Vec<u8>> = HashMap::new();

        files.insert("data.yaml".to_string(), self.create_data_yaml(data).into_bytes());
        let class_list = get_class_list(data);
        files.insert("classes.txt".to_string(), class_list.join("\n").into_bytes());

        let splits = [
            ("train", data.train_images()),
            ("valid", data.valid_images()),
            ("test", data.test_images()),
        ];

        for (split, images) in splits {
            for img in images {
                let label_content = self.create_detection_label(img);
                let label_filename = img.file.rsplit_once('.').map(|(name, _)| name).unwrap_or(&img.file);

                files.insert(
                    format!("{}/labels/{}.txt", split, label_filename),
                    label_content.into_bytes(),
                );
                if let Some(image_data) = downloaded_images.get(&img.file) {
                    files.insert(format!("{}/images/{}", split, img.file), image_data.clone());
                }
            }
        }

        files
    }
}
