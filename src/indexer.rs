use ignore::Walk;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::DocAddress;
use tantivy::Index;
use tantivy::ReloadPolicy;
use tantivy::Score;
use tempfile::TempDir;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn index() -> tantivy::Result<()> {
    let index_path = TempDir::new()?;
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("file_name", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    schema_builder.add_text_field("path", TEXT | STORED);
    schema_builder.add_text_field("line", STRING | STORED);

    let schema = schema_builder.build();
    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let mut index_writer = index.writer(50_000_000)?;
    let name = schema.get_field("file_name").unwrap();
    let body = schema.get_field("body").unwrap();
    let path = schema.get_field("path").unwrap();

    for entry in Walk::new(".").flat_map(|v| v.ok()) {
        let file_path = entry.path();
        if file_path.is_dir() {
            continue;
        }

        let file = File::open(file_path);
        if let Ok(file) = file {
            let mut reader = BufReader::new(file);
            let mut doc = Document::default();
            doc.add_text(
                name,
                file_path.file_name().unwrap().to_str().unwrap().to_owned(),
            );
            doc.add_text(path, file_path.display().to_string());
            let mut data = String::new();

            let mut index = 0;
            loop {
                index += 1;
                let mut raw = String::new();
                match reader.read_line(&mut raw) {
                    Ok(_) => {
                        if raw.is_empty() {
                            break;  
                        }
                        data.push_str(&format!("{}: {}\n", index, raw));
                    }
                    _ => break
                }
            }

            if data.is_empty() {
                continue;  
            }

            doc.add_text(body, data);
            index_writer.add_document(doc)?;
            index_writer.commit()?;
        }
    }

    println!("Index stored to: {}", index_path.path().display());
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![name, body]);
    let query = query_parser.parse_query("loop")?;
    let top_docs: Vec<(Score, DocAddress)> = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_, doc) in top_docs {
        let doc = searcher.doc(doc)?;
        println!("{}", schema.to_json(&doc));
    }
    Ok(())
}

