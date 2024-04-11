use crate::models::*;
use std::result::Result::Ok;
use tantivy::collector::{Count, TopDocs};
use tantivy::query::{FuzzyTermQuery, QueryParser};
use tantivy::schema::*;
use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy};

pub fn create_verse_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("book", STRING | STORED);
    schema_builder.add_text_field("ref", STRING | STORED);
    schema_builder.add_u64_field("chapter", FAST | STORED | INDEXED);
    schema_builder.add_text_field("content", TEXT | STORED);
    schema_builder.build()
}

pub fn index_verses(
    bible: &Bible,
    index_writer: &mut IndexWriter,
    schema: &Schema,
) -> anyhow::Result<()> {
    for ch in &bible.chapters {
        for v in &ch.verses {
            index_writer.add_document(doc!(
                schema.get_field("book").unwrap() => ch.book.as_str(),
                schema.get_field("ref").unwrap() => v.r#ref.as_str(),
                schema.get_field("content").unwrap() => v.text.as_str(),
            ))?;
        }
    }
    index_writer.commit()?;

    Ok(())
}

pub fn query_verses(
    index_reader: &IndexReader,
    schema: &Schema,
    query: &str,
) -> anyhow::Result<Vec<String>> {
    let searcher = index_reader.searcher();

    let query_parser = QueryParser::for_index(
        &searcher.index(),
        vec![
            schema.get_field("ref").unwrap(),
            schema.get_field("book").unwrap(),
            schema.get_field("content").unwrap(),
        ],
    );

    let query = query_parser.parse_query(query)?;

    let top_docs = searcher.search(&query, &TopDocs::with_limit(50))?;

    let mut results = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        if let Some(ref_field) = retrieved_doc.get_first(schema.get_field("ref").unwrap()) {
            if let Some(ref_value) = ref_field.as_str() {
                results.push(ref_value.to_string());
            }
        }
    }
    Ok(results)
}
