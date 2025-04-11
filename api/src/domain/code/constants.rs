use super::CodeEntity;

pub const UNEXPECTED_ERROR: CodeEntity = CodeEntity {
    code: 1000,
    name: "Unexpected",
};

pub const CLAIMS_ERROR: CodeEntity = CodeEntity {
    code: 1001,
    name: "Claims error",
};

pub const PARSE_JSON_ERROR: CodeEntity = CodeEntity {
    code: 1001,
    name: "Parse json error",
};

pub const HEADER_ERROR: CodeEntity = CodeEntity {
    code: 1002,
    name: "Header error",
};

pub const PATH_ERROR: CodeEntity = CodeEntity {
    code: 1003,
    name: "Path error",
};

pub const QUERY_ERROR: CodeEntity = CodeEntity {
    code: 1004,
    name: "Query error",
};

pub const UTF8_ENCODING_ERROR: CodeEntity = CodeEntity {
    code: 1003,
    name: "UTF8 encoding error",
};
