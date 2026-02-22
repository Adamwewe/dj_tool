// use lofty::{read_from_path, TagType, Probe};

pub fn parser(path_vec: Vec<String>){

// Retrieving from file signature



// for i in path_vec{
//     println!("{}", i);
//     let mut tagged_file = read_from_path(i);

    
// 	let tag = match tagged_file.primary_tag() {
// 		Some(primary_tag) => primary_tag,
// 		// If the "primary" tag doesn't exist, we just grab the
// 		// first tag we can find. Realistically, a tag reader would likely
// 		// iterate through the tags to find a suitable one.
// 		None => tagged_file.first_tag().expect("ERROR: No tags found!"),
// 	};

    // println!("{:?}", tagged_file.title());
    
    //.write_to_path(format!("converted/{}.aiff", i))
    // let mut tag_sig = Tag::new("mp3")              
    //     .read_from_path_signature(i)
    //     .unwrap();
} 

// let mut tag_sig = Tag::new().read_from_path_signature("tests/assets/a.wav").unwrap();
// tag_sig.set_artist("Foo artist");

// You can convert the tag type and save the metadata to another file.
// tag.to_dyn_tag(TagType::Mp4).write_to_path(format!("converted/{}.aiff", i));



