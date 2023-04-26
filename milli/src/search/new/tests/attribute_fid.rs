use crate::{index::tests::TempIndex, Criterion, Search, SearchResult, TermsMatchingStrategy};

fn create_index() -> TempIndex {
    let index = TempIndex::new();

    index
        .update_settings(|s| {
            s.set_primary_key("id".to_owned());
            s.set_searchable_fields(vec![
                "title".to_owned(),
                "description".to_owned(),
                "plot".to_owned(),
            ]);
            s.set_criteria(vec![Criterion::Attribute]);
        })
        .unwrap();

    index
        .add_documents(documents!([
            {
                "id": 0,
                "title": "",
                "description": "",
                "plot": "the quick brown fox jumps over the lazy dog",
            },
            {
                "id": 1,
                "title": "",
                "description": "the quick brown foxes jump over the lazy dog",
                "plot": "",
            },
            {
                "id": 2,
                "title": "the quick brown fox jumps over the lazy dog",
                "description": "",
                "plot": "",
            },
            {
                "id": 3,
                "title": "the",
                "description": "quick brown fox jumps over the lazy dog",
                "plot": "",
            },
            {
                "id": 4,
                "title": "the quick",
                "description": "brown fox jumps over the lazy dog",
                "plot": "",
            },
            {
                "id": 5,
                "title": "the quick brown",
                "description": "fox jumps over the lazy dog",
                "plot": "",
            },
            {
                "id": 6,
                "title": "the quick brown fox",
                "description": "jumps over the lazy dog",
                "plot": "",
            },
            {
                "id": 7,
                "title": "the quick",
                "description": "brown fox jumps",
                "plot": "over the lazy dog",
            },
            {
                "id": 8,
                "title": "the quick brown",
                "description": "fox",
                "plot": "jumps over the lazy dog",
            },
            {
                "id": 9,
                "title": "the quick brown",
                "description": "fox jumps",
                "plot": "over the lazy dog",
            },
            {
                "id": 10,
                "title": "",
                "description": "the quick brown fox",
                "plot": "jumps over the lazy dog",
            },
            {
                "id": 11,
                "title": "the quick",
                "description": "",
                "plot": "brown fox jumps over the lazy dog",
            }
        ]))
        .unwrap();
    index
}

#[test]
fn test_attribute_fid_simple() {
    let index = create_index();

    let txn = index.read_txn().unwrap();

    let mut s = Search::new(&txn, &index);
    s.terms_matching_strategy(TermsMatchingStrategy::All);
    s.query("the quick brown fox jumps over the lazy dog");
    let SearchResult { documents_ids, .. } = s.execute().unwrap();
    insta::assert_snapshot!(format!("{documents_ids:?}"), @"[2, 6, 5, 4, 3, 9, 7, 8, 11, 10, 0]");
}