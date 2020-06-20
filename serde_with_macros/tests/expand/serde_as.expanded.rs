use serde_with_macros::serde_as;
/// Test correct replacement of the infer type
struct DataInferTypeReplacement {
    #[serde(with = "::serde_with::As::<:: serde_with :: Same>")]
    a: u32,
    #[serde(with = "::serde_with::As::<std :: vec :: Vec < :: serde_with :: Same >>")]
    b: Vec<u32>,
    #[serde(with = "::serde_with::As::<Vec < (:: serde_with :: Same, :: serde_with :: Same) >>")]
    c: Vec<(u32, String)>,
    #[serde(with = "::serde_with::As::<[:: serde_with :: Same ; 2]>")]
    d: [u32; 2],
    #[serde(with = "::serde_with::As::<Box < [:: serde_with :: Same] >>")]
    e: Box<[u32]>,
}
/// Test different variants of *as annotation
struct DataVariants {
    #[serde(with = "::serde_with::As::<:: serde_with :: Same>")]
    a: u32,
    #[serde(
        deserialize_with = "::serde_with::As::<std :: vec :: Vec < :: serde_with :: Same >>::deserialize"
    )]
    #[serde(serialize_with = "::serde_with::As::<:: serde_with :: Same>::serialize")]
    b: Vec<u32>,
    #[serde(
        serialize_with = "::serde_with::As::<HashMap < :: serde_with :: Same, :: serde_with :: Same >>::serialize"
    )]
    c: Vec<(u32, String)>,
    #[serde(deserialize_with = "::serde_with::As::<[:: serde_with :: Same ; 2]>::deserialize")]
    d: [u32; 2],
    #[serde(with = "::serde_with::As::<Box < [:: serde_with :: Same] >>")]
    e: Box<[u32]>,
}
/// Test replacement in enums
enum DataEnumWithStructVariants {
    Var1 {
        #[serde(with = "::serde_with::As::<:: serde_with :: Same>")]
        a: u32,
    },
    Var2 {
        #[serde(with = "::serde_with::As::<:: serde_with :: Same>")]
        b: u32,
        #[serde(deserialize_with = "::serde_with::As::<Abc>::deserialize")]
        #[serde(serialize_with = "::serde_with::As::<Def>::serialize")]
        c: String,
    },
    Unit,
}
