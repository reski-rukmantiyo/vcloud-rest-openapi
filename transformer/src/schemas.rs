use indexmap::IndexMap;
use openapiv3::{ReferenceOr, Schema};
use std::convert::TryFrom;
use std::io::{Read, Seek};
use zip::read::ZipArchive;

pub fn schemas<R: Read + Seek>(
    zip: &mut ZipArchive<R>,
) -> Result<IndexMap<String, ReferenceOr<Schema>>, Box<dyn std::error::Error>> {
    let mut output = IndexMap::new();
    let mut type_file_names = zip
        .file_names()
        .filter(|n| n.starts_with("doc/etc/"))
        .filter(|n| n.ends_with(".xsd"))
        .filter(|&n| n != "doc/etc/schemas/external/xml.xsd")
        .map(|n| n.into())
        .collect::<Vec<String>>();

    type_file_names.sort();

    for type_file_name in type_file_names {
        let mut bytes = Vec::new();
        zip.by_name(&type_file_name)?.read_to_end(&mut bytes)?;

        let xsd_schema = crate::parsers::doc::etc::schema::Schema::try_from(&bytes as &[u8])?;
        output.extend(
            Vec::<Schema>::from(&xsd_schema)
                .into_iter()
                .filter_map(|s| {
                    s.schema_data
                        .title
                        .clone()
                        .map(|title| (title, ReferenceOr::Item(s)))
                }),
        );
    }

    Ok(output)
}
