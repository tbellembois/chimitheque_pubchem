// XML shema available at:
// https://pubchem.ncbi.nlm.nih.gov/pug_rest/pug_rest.xsd
// https://pubchem.ncbi.nlm.nih.gov/pug_view/pug_view.xsd

use serde::{Deserialize, Serialize};

// Autocomplete
#[derive(Serialize, Deserialize, Debug)]
pub struct AutocompleteTerm {
    compound: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Autocomplete {
    total: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    dictionary_terms: Option<AutocompleteTerm>,
}

// PUG REST
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Markup {
    #[serde(rename = "Start")]
    start: f64,

    #[serde(rename = "Length")]
    length: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "URL")]
    pub(crate) url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Type")]
    the_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Extra")]
    extra: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StringWithMarkup {
    #[serde(rename = "String")]
    string: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Markup")]
    pub(crate) markup: Option<Vec<Markup>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Value {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Number")]
    number: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "DateISO8601")]
    date_iso_8601: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Boolean")]
    boolean: Option<Vec<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Binary")]
    binary: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "BinaryToStore")]
    binary_to_store: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ExternalDataURL")]
    external_data_url: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ExternalTableName")]
    external_table_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Unit")]
    unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "MimeType")]
    mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ExternalTableNumRows")]
    external_table_num_rows: Option<isize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "StringWithMarkup")]
    pub(crate) string_with_markup: Option<Vec<StringWithMarkup>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Information {
    #[serde(rename = "ReferenceNumber")]
    reference_number: isize,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Name")]
    pub(crate) name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Description")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Reference")]
    reference: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "LicenseNote")]
    license_note: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "LicenseURL")]
    license_url: Option<String>,

    #[serde(rename = "Value")]
    pub(crate) value: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "TOCHeading")]
    pub(crate) toc_heading: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "TOCID")]
    pub(crate) toc_id: Option<isize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Description")]
    pub(crate) description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "URL")]
    pub(crate) url: Option<String>,

    #[serde(rename = "Section")]
    pub(crate) section: Option<Vec<Section>>,

    #[serde(rename = "Information")]
    pub(crate) information: Option<Vec<Information>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "RecordType")]
    record_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "RecordNumber")]
    record_number: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "RecordAccession")]
    record_accession: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "RecordTitle")]
    pub(crate) record_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "RecordExternalURL")]
    record_external_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Section")]
    pub(crate) section: Option<Vec<Section>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Information")]
    information: Option<Vec<Information>>,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename = "Prop_value")]
// enum PropValue {
//     #[serde(rename = "ival")]
//     Ival(isize),
//     #[serde(rename = "fval")]
//     Fval(f64),
//     #[serde(rename = "binary")]
//     Binary(String),
//     #[serde(rename = "sval")]
//     Sval(String),
// }

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename = "Prop_URN")]
// pub struct PropURN {
//     label: String,

//     #[serde(skip_serializing_if = "Option::is_none")]
//     name: Option<String>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Prop {
//     urn: PropURN,
//     value: PropValue,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename = "Record")]
    pub(crate) record: RecordContent,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Cid {
//     cid: usize,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ID {
//     id: Cid,
// }

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename = "PC_Compound")]
// pub struct PCCompound {
//     id: ID,
//     props: Vec<Prop>,
//     record: Option<Record>,
// }

// #[derive(Serialize, Deserialize, Debug, Default)]
// pub struct Compounds {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub(crate) record: Option<Record>,

//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub(crate) base64_png: Option<String>,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    #[serde(rename = "CID")]
    pub(crate) cid: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "MolecularFormula")]
    molecular_formula: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    #[serde(rename = "Properties")]
    pub(crate) properties: Vec<Property>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyTable {
    #[serde(rename = "PropertyTable")]
    pub(crate) property_table: Properties,
}
