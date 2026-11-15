use lopdf::{Document, Object, ObjectId, Stream};

fn main() {
    let input_path = "/home/krushna/Project/PaperPilot/ManaulTestingFixtures/pdf_dedup_streams/Input/duplicate_streams_doc.pdf";
    let mut doc = Document::load(input_path).expect("Failed to load PDF");

    let mut image_stream: Option<Stream> = None;
    for (id, obj) in doc.objects.iter() {
        if let Object::Stream(s) = obj {
            if s.dict.get(b"Subtype").map(|st| st == &Object::Name(b"Image".to_vec())).unwrap_or(false) {
                image_stream = Some(s.clone());
                println!("Found image stream at object {:?}", id);
                break;
            }
        }
    }

    if let Some(img) = image_stream {
        let pages: Vec<ObjectId> = doc.page_iter().collect();
        println!("Document has {} pages", pages.len());

        for (page_idx, &page_id) in pages.iter().enumerate().skip(1) {
            let new_img_id = doc.add_object(Object::Stream(img.clone()));
            println!("Page {} assigned duplicate image stream {:?}", page_idx + 1, new_img_id);

            if let Ok(res_id) = doc.get_object(page_id).and_then(|o| o.as_dict()).and_then(|d| d.get(b"Resources")).and_then(|r| r.as_reference()) {
                if let Ok(res_obj) = doc.get_object_mut(res_id) {
                    if let Ok(res_dict) = res_obj.as_dict_mut() {
                        if let Ok(xobj) = res_dict.get_mut(b"XObject") {
                            if let Ok(xdict) = xobj.as_dict_mut() {
                                for (_k, v) in xdict.iter_mut() {
                                    *v = Object::Reference(new_img_id);
                                }
                            }
                        }
                    }
                }
            } else if let Ok(page_dict) = doc.get_object_mut(page_id).and_then(|o| o.as_dict_mut()) {
                if let Ok(res_obj) = page_dict.get_mut(b"Resources") {
                    if let Ok(res_dict) = res_obj.as_dict_mut() {
                        if let Ok(xobj) = res_dict.get_mut(b"XObject") {
                            if let Ok(xdict) = xobj.as_dict_mut() {
                                for (_k, v) in xdict.iter_mut() {
                                    *v = Object::Reference(new_img_id);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    doc.save(input_path).expect("Failed to save updated PDF");
    println!("Successfully injected true duplicate streams into {}", input_path);
}

