pub struct PersonDocumentTypeSelector {
    pub person_document_type_id: i32,
    pub name: String,
    pub summary: Option<String>,
    pub signature: String,
}

#[derive(Debug, Clone)]
pub struct PersonDocumentTypePageSelector {
    pub person_document_type_id: i32,
    pub name: String,
    pub summary: Option<String>,
    pub signature: String,
}
