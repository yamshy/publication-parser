use pdf::file::{File, FileOptions, NoCache};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for arg in &args.as_slice()[1..] {
        let path = std::path::Path::new(arg);
        println!("Parsing file: {}", arg);

        let file = FileOptions::uncached().open(path).unwrap();
        let doi = find_doi(&file);

        if doi.is_some() {
            println!("Found DOI: {}", doi.unwrap());
        } else {
            println!("No DOI found");
        }
    }
}

type PdfFile = File<Vec<u8>, NoCache, NoCache>;

fn find_doi(file: &PdfFile) -> Option<String> {
    let mut fn_array: Vec<&dyn Fn(&PdfFile) -> Option<String>> =
        vec![&find_doi_in_metadata, &find_doi_in_text];
    let mut doi: Option<String> = None;

    while doi.is_none() && !fn_array.is_empty() {
        doi = fn_array[0](file);
        fn_array.remove(0);
    }

    doi
}

fn find_doi_in_metadata(file: &PdfFile) -> Option<String> {
    let mut doi: Option<String> = None;

    if let Some(ref info) = file.trailer.info_dict {
        info.iter()
            .find(|(key, _)| key.as_str() == "doi")
            .map(|(_, value)| doi = Some(value.to_string_lossy().unwrap()));
    }

    doi
}

fn find_doi_in_text(file: &PdfFile) -> Option<String> {
    let mut doi: Option<String> = None;

    for page in file.pages().take(1) {
        let page = page.unwrap();
        println!("Page: {:?}", page);
    }

    doi
}
