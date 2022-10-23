mod data;

use data::data::get_words;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;

    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_n_words(n: u8) -> String {
    // log("Started executing!!!!");
    let mut i = 0;
    let words = get_words();
    // log("Read words!!!!");
    let word_list: Vec<&str> = words.trim().split('\n').collect();

    let mut selected_words = Vec::<String>::new();

    // factor for getting random values bw range
    let len = word_list.len();

    while i != n {
        let indx = (random() * len as f64) as usize;
        let word = word_list.get(indx);

        println!("{} = {:?}", indx, word.clone().to_owned());

        match word {
            Some(val) => {
                let value = val.to_string();
                if selected_words.contains(&value) {
                    continue;
                } else {
                    selected_words.push(value);
                    i += 1;
                }
            }
            None => continue,
        }
    }

    println!("{:?}", selected_words);
    // log("Completed!!!!");

    selected_words.join(":")
}
