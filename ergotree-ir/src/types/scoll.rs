use crate::serialization::types::TypeCode;
use crate::types::stuple::STuple;

use super::sfunc::SFunc;
use super::smethod::MethodId;
use super::smethod::SMethod;
use super::smethod::SMethodDesc;
use super::stype::SType;
use super::stype_companion::STypeCompanion;
use super::stype_companion::STypeCompanionHead;
use super::stype_param::STypeVar;
use lazy_static::lazy_static;

/// type id
pub const TYPE_ID: TypeCode = TypeCode::COLLECTION;
/// Coll.indexOf
pub const INDEX_OF_METHOD_ID: MethodId = MethodId(26);
/// Coll.flatmap
pub const FLATMAP_METHOD_ID: MethodId = MethodId(15);
/// Coll.zip
pub const ZIP_METHOD_ID: MethodId = MethodId(14);
/// Coll.indices
pub const INDICES_METHOD_ID: MethodId = MethodId(29);

static S_COLL_TYPE_COMPANION_HEAD: STypeCompanionHead = STypeCompanionHead {
    type_id: TYPE_ID,
    type_name: "Coll",
};

lazy_static! {
    /// Coll object type companion
    pub static ref S_COLL_TYPE_COMPANION: STypeCompanion = STypeCompanion::new(
        &S_COLL_TYPE_COMPANION_HEAD,
        vec![
            &INDEX_OF_METHOD_DESC,
            &FLATMAP_METHOD_DESC,
            &ZIP_METHOD_DESC,
            &INDICES_METHOD_DESC,
        ]
    );
}

lazy_static! {
    static ref INDEX_OF_METHOD_DESC: SMethodDesc = SMethodDesc {
        method_id: INDEX_OF_METHOD_ID,
        name: "indexOf",
        tpe: SFunc {
            t_dom: vec![SType::SColl(SType::STypeVar(STypeVar::t()).into()), STypeVar::t().into(), SType::SInt],
            t_range: SType::SInt.into(),
            tpe_params: vec![],
        },
    };
    /// Coll.indexOf
    pub static ref INDEX_OF_METHOD: SMethod = SMethod::new(&S_COLL_TYPE_COMPANION, INDEX_OF_METHOD_DESC.clone());
}

lazy_static! {
    static ref FLATMAP_METHOD_DESC: SMethodDesc = SMethodDesc {
        method_id: FLATMAP_METHOD_ID,
        name: "flatMap",
        tpe: SFunc::new(
            vec![
                SType::SColl(SType::STypeVar(STypeVar::iv()).into()),
                SFunc::new(
                    vec![STypeVar::iv().into()],
                    SType::SColl(Box::new(STypeVar::ov().into())),
                ).into()
                ],
            SType::SColl(SType::STypeVar(STypeVar::ov()).into()),
        ),
    };
    /// Coll.flatMap
    pub static ref FLATMAP_METHOD: SMethod = SMethod::new(&S_COLL_TYPE_COMPANION, FLATMAP_METHOD_DESC.clone());
}

lazy_static! {
    static ref ZIP_METHOD_DESC: SMethodDesc = SMethodDesc {
        method_id: ZIP_METHOD_ID,
        name: "zip",
        tpe: SFunc::new(
            vec![
                SType::SColl(SType::STypeVar(STypeVar::t()).into()),
                SType::SColl(SType::STypeVar(STypeVar::iv()).into())
            ],
            SType::SColl(SType::STuple(STuple::pair(
                STypeVar::t().into(), STypeVar::iv().into()
            )).into())
        )
    };
    /// Coll.zip
    pub static ref ZIP_METHOD: SMethod = SMethod::new(&S_COLL_TYPE_COMPANION, ZIP_METHOD_DESC.clone());
}

lazy_static! {
    static ref INDICES_METHOD_DESC: SMethodDesc = SMethodDesc {
        method_id: INDICES_METHOD_ID,
        name: "indices",
        tpe: SFunc::new(
            vec![
                SType::SColl(SType::STypeVar(STypeVar::t()).into()),
            ],
            SType::SColl(SType::SInt.into())
        )
    };
    /// Coll.indices
    pub static ref INDICES_METHOD: SMethod = SMethod::new(&S_COLL_TYPE_COMPANION, INDICES_METHOD_DESC.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ids() {
        assert!(SMethod::from_ids(TYPE_ID, INDEX_OF_METHOD_ID).map(|e| e.name()) == Ok("indexOf"));
        assert!(SMethod::from_ids(TYPE_ID, FLATMAP_METHOD_ID).map(|e| e.name()) == Ok("flatMap"));
        assert!(SMethod::from_ids(TYPE_ID, ZIP_METHOD_ID).map(|e| e.name()) == Ok("zip"));
        assert!(SMethod::from_ids(TYPE_ID, INDICES_METHOD_ID).map(|e| e.name()) == Ok("indices"));
    }
}
