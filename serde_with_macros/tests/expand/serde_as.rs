use serde_with_macros::serde_as;

/// Test correct replacement of the infer type
#[serde_as]
struct DataInferTypeReplacement {
    #[serde_as(as = "_")]
    a: u32,
    #[serde_as(as = "std::vec::Vec<_>")]
    b: Vec<u32>,
    #[serde_as(as = "Vec<(_, _)>")]
    c: Vec<(u32, String)>,
    #[serde_as(as = "[_; 2]")]
    d: [u32; 2],
    #[serde_as(as = "Box<[_]>")]
    e: Box<[u32]>,
}

/// Test different variants of *as annotation
#[serde_as]
struct DataVariants {
    #[serde_as(as = "_")]
    a: u32,
    #[serde_as(deserialize_as = "std::vec::Vec<_>", serialize_as = "_")]
    b: Vec<u32>,
    #[serde_as(serialize_as = "HashMap<_, _>")]
    c: Vec<(u32, String)>,
    #[serde_as(deserialize_as = "[_; 2]")]
    d: [u32; 2],
    #[serde_as(as = "Box<[_]>")]
    e: Box<[u32]>,
}

/// Test replacement in enums
#[serde_as]
enum DataEnumWithStructVariants {
    Var1 {
        #[serde_as(as = "_")]
        a: u32,
    },
    Var2 {
        #[serde_as(as = "_")]
        b: u32,
        #[serde_as(deserialize_as = "Abc", serialize_as = "Def")]
        c: String,
    },
    Unit,
}
