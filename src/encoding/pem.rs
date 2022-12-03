// TODO: implement Go's internal/bytealg first

// use std::collections::HashMap;

// const PEM_START: &[u8] = b"\n-----BEGIN";
// const PEM_END: &[u8] = b"\n-----END ";
// const PEM_END_OF_LINE: &[u8] = b"-----";
// const COLON: u8 = b':';


// /// A Block represents a PEM encoded structure.
// ///
// /// The encoded form is:
// /// 
// /// ```text
// ///	-----BEGIN Type-----
// ///	Headers
// ///	base64-encoded Bytes
// ///	-----END Type-----
// /// ```
// ///
// /// where Headers is a possibly empty sequence of Key: Value lines.
// pub struct Block {
//     ty: String,
//     headers: HashMap<String, String>,
//     bytes: Vec<u8>,
// }

// #[inline]
// fn get_line(data: &str) -> (&str, &str) {
//     let i = data.find(|x| x == (b'\n' as char));

//     match i {
//         Some(mut i) => {
//             let j = i + 1;
//             if data.as_bytes()[i - 1] == b'\r' {
//                 i -= 1;
//             }
//             (data[0..i].trim_end_matches(" \t"), &data[j..]) 
//         },
//         None => {
//             let ii = data.len();
//             let j = ii;
//             (data[0..ii].trim_end_matches(" \t"), &data[j..])
//         },
//     }
// }

// fn remove_space_and_tabs(data: &str) -> &str {
//     if data.find(|x| *x == " \t") {

//     }
// }

