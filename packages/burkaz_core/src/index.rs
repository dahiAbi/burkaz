use std::{
    path::Path,
    sync::{Arc, Mutex, MutexGuard},
};

use tantivy::{
    Index, IndexBuilder, IndexReader, IndexWriter, Searcher, SegmentOrdinal, TantivyDocument,
    TantivyError,
    directory::{Directory, MmapDirectory, RamDirectory},
    indexer::IndexWriterOptions,
    query::{Query, QueryParser},
    schema::FieldType,
};

use crate::error::BurkazError;
use crate::{address::BurkazObjectAddr, schema::BurkazSchema};

#[derive(Clone)]
pub struct BurkazIndex(Arc<InnerBurkazIndex>);

struct InnerBurkazIndex {
    _name: String,
    _underlying_index: Index,
    reader: IndexReader,
    writer: Arc<Mutex<IndexWriter<TantivyDocument>>>,
    query_parser: QueryParser,
}

pub enum BurkazDirectory<'a> {
    InMemory,
    OnDisk(&'a Path),
}

impl<'a> BurkazDirectory<'a> {
    pub fn create_if_not_exists(&self) -> crate::Result<()> {
        match self {
            BurkazDirectory::InMemory => Ok(()),
            BurkazDirectory::OnDisk(path) => {
                if !path.exists() {
                    std::fs::create_dir(path)
                        .map_err(Into::<TantivyError>::into)
                        .map_err(Into::<BurkazError>::into)
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl BurkazIndex {
    pub fn new(
        name: String,
        schema: BurkazSchema,
        directory: BurkazDirectory,
    ) -> crate::Result<Self> {
        let index_builder = IndexBuilder::new().schema(schema.into());

        let index = index_builder
            .open_or_create(match directory {
                BurkazDirectory::InMemory => {
                    Box::new(RamDirectory::default()) as Box<dyn Directory>
                }
                BurkazDirectory::OnDisk(path) => Box::new(
                    MmapDirectory::open(path)
                        .map_err(Into::<TantivyError>::into)
                        .map_err(Into::<BurkazError>::into)?,
                ) as Box<dyn Directory>,
            })
            .map_err(Into::<BurkazError>::into)?;

        let writer_options = IndexWriterOptions::builder().build();
        let writer = index
            .writer_with_options::<TantivyDocument>(writer_options)
            .map(Into::<IndexWriter<TantivyDocument>>::into)?;

        let reader = index.reader().map_err(Into::<BurkazError>::into)?;

        let query_parser = {
            let schema = index.schema();
            let mut parser =
                QueryParser::for_index(&index, schema.fields().map(|(field, _)| field).collect());
            for (field, entry) in schema.fields() {
                if entry.is_indexed() {
                    if matches!(
                        entry.field_type(),
                        FieldType::Str(_) | FieldType::JsonObject(_)
                    ) {
                        parser.set_field_fuzzy(field, false, 2, true);
                    }
                }
            }
            parser
        };

        Ok(BurkazIndex(Arc::new(InnerBurkazIndex {
            _name: name,
            _underlying_index: index,
            reader: reader,
            writer: Arc::new(Mutex::new(writer)),
            query_parser: query_parser,
        })))
    }

    pub unsafe fn from_raw(ptr: *mut Self) -> Self {
        unsafe { *Box::from_raw(ptr) }
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.0._name
    }

    pub fn query_parser(&self) -> &QueryParser {
        &self.0.query_parser
    }

    pub fn get_writer(&self) -> crate::Result<MutexGuard<'_, IndexWriter<TantivyDocument>>> {
        self.0
            .writer
            .lock()
            .map_err(|e| BurkazError::UnknownError(e.to_string()))
    }

    #[inline]
    pub fn searcher(&self) -> Searcher {
        self.0.reader.searcher()
    }

    pub fn get(&self, addr: BurkazObjectAddr) -> crate::Result<TantivyDocument> {
        let searcher = self.searcher();
        let max_segment_ord = searcher.segment_readers().len();
        if addr.segment_ord() >= max_segment_ord as SegmentOrdinal {
            return Err(BurkazError::ObjectNotFound(addr));
        }

        searcher
            .doc::<TantivyDocument>(addr.into())
            .map_err(Into::<BurkazError>::into)
    }

    pub fn add(&self, object: TantivyDocument) -> crate::Result<()> {
        let mut writer = self.get_writer()?;

        writer
            .add_document(object)
            .map_err(Into::<BurkazError>::into)?;

        writer.commit().map_err(Into::<BurkazError>::into)?;

        self.0.reader.reload().map_err(Into::<BurkazError>::into)?;

        Ok(())
    }

    pub fn add_all(&self, objects: Vec<TantivyDocument>) -> crate::Result<()> {
        if objects.is_empty() {
            return Ok(());
        }

        let mut writer = self.get_writer()?;

        for object in objects.iter() {
            writer
                .add_document(object.clone())
                .map_err(Into::<BurkazError>::into)?;
        }

        writer.commit().map_err(Into::<BurkazError>::into)?;

        self.0.reader.reload().map_err(Into::<BurkazError>::into)?;

        Ok(())
    }

    pub fn clear(&self) -> crate::Result<()> {
        let mut writer = self.get_writer()?;

        writer
            .delete_all_documents()
            .map_err(Into::<BurkazError>::into)?;

        writer.commit().map_err(Into::<BurkazError>::into)?;

        self.0.reader.reload().map_err(Into::<BurkazError>::into)?;

        Ok(())
    }

    pub fn delete_all_by_query(&self, query: Box<dyn Query>) -> crate::Result<()> {
        let mut writer = self.get_writer()?;

        writer
            .delete_query(query)
            .map_err(Into::<BurkazError>::into)?;

        writer.commit().map_err(Into::<BurkazError>::into)?;

        self.0.reader.reload().map_err(Into::<BurkazError>::into)?;

        Ok(())
    }
}
