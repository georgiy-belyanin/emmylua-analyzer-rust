use std::ops::Deref;

use crate::{
    DbIndex, LuaMemberKey, LuaMemberOwner, LuaObjectType, LuaTupleType, LuaType, RenderLevel,
    TypeCheckFailReason, TypeCheckResult, VariadicType, humanize_type,
    semantic::type_check::{check_general_type_compact, type_check_guard::TypeCheckGuard},
};

pub fn check_tuple_type_compact(
    db: &DbIndex,
    tuple: &LuaTupleType,
    compact_type: &LuaType,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    match compact_type {
        LuaType::Tuple(compact_tuple) => {
            return check_tuple_type_compact_tuple(
                db,
                tuple,
                compact_tuple,
                check_guard.next_level()?,
            );
        }
        LuaType::Array(array_type) => {
            for source_type in tuple.get_types() {
                check_general_type_compact(
                    db,
                    array_type.get_base(),
                    source_type,
                    check_guard.next_level()?,
                )?;
            }

            return Ok(());
        }
        LuaType::TableConst(inst) => {
            let table_member_owner = LuaMemberOwner::Element(inst.clone());
            return check_tuple_type_compact_table(
                db,
                tuple,
                table_member_owner,
                check_guard.next_level()?,
            );
        }
        LuaType::Object(object) => {
            return check_tuple_type_compact_object_type(
                db,
                tuple,
                object,
                check_guard.next_level()?,
            );
        }
        // for any untyped table
        LuaType::Table => return Ok(()),
        _ => {}
    }

    Err(TypeCheckFailReason::DonotCheck)
}

fn check_tuple_type_compact_tuple(
    db: &DbIndex,
    source_tuple: &LuaTupleType,
    compact_tuple: &LuaTupleType,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    let source_tuple_members = source_tuple.get_types();
    let compact_tuple_members = compact_tuple.get_types();

    check_tuple_types_compact_tuple_types(
        db,
        0,
        source_tuple_members,
        compact_tuple_members,
        check_guard,
    )
}

fn check_tuple_types_compact_tuple_types(
    db: &DbIndex,
    source_start: usize,
    sources: &[LuaType],
    compacts: &[LuaType],
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    let source_size = sources.len();
    let compact_size = compacts.len();

    for i in 0..source_size {
        let source_tuple_member_type = &sources[i];
        if i >= compact_size {
            if source_tuple_member_type.is_optional() {
                continue;
            } else {
                return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                    t!("missing tuple member %{idx}", idx = i + source_start + 1).to_string(),
                ));
            }
        }
        let compact_tuple_member_type = &compacts[i];
        match (source_tuple_member_type, compact_tuple_member_type) {
            (LuaType::Variadic(variadic), _) => {
                if let VariadicType::Base(inner) = variadic.deref() {
                    let compact_rest_len = compact_size - i;
                    if compact_rest_len == 0 {
                        return Ok(());
                    }
                    let mut new_source_types = vec![];
                    for _ in 0..compact_rest_len {
                        new_source_types.push(inner.clone());
                    }
                    return check_tuple_types_compact_tuple_types(
                        db,
                        i,
                        &new_source_types,
                        &compacts[i..],
                        check_guard.next_level()?,
                    );
                }
            }
            (_, LuaType::Variadic(variadic)) => {
                if let VariadicType::Base(compact_inner) = variadic.deref() {
                    let source_rest_len = source_size - i;
                    if source_rest_len == 0 {
                        return Ok(());
                    }
                    let mut new_compact_types = vec![];
                    for _ in 0..source_rest_len {
                        new_compact_types.push(compact_inner.clone());
                    }
                    return check_tuple_types_compact_tuple_types(
                        db,
                        i,
                        &sources[i..],
                        &new_compact_types,
                        check_guard.next_level()?,
                    );
                }
            }
            _ => {
                match check_general_type_compact(
                    db,
                    source_tuple_member_type,
                    compact_tuple_member_type,
                    check_guard.next_level()?,
                ) {
                    Ok(_) => {}
                    Err(TypeCheckFailReason::TypeNotMatch) => {
                        return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                            t!(
                                "tuple member %{idx} not match, expect %{typ}, but got %{got}",
                                idx = i + source_start + 1,
                                typ = humanize_type(
                                    db,
                                    source_tuple_member_type,
                                    RenderLevel::Simple
                                ),
                                got = humanize_type(
                                    db,
                                    compact_tuple_member_type,
                                    RenderLevel::Simple
                                )
                            )
                            .to_string(),
                        ));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }

    Ok(())
}

fn check_tuple_type_compact_table(
    db: &DbIndex,
    source_tuple: &LuaTupleType,
    table_owner: LuaMemberOwner,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    let member_index = db.get_member_index();
    let tuple_members = source_tuple.get_types();
    let size = tuple_members.len();
    for i in 0..size {
        let source_tuple_member_type = &tuple_members[i];
        let key = LuaMemberKey::Integer((i + 1) as i64);
        if let Some(member_item) = member_index.get_member_item(&table_owner, &key) {
            let member_type = member_item
                .resolve_type(db)
                .map_err(|_| TypeCheckFailReason::TypeNotMatch)?;
            match check_general_type_compact(
                db,
                source_tuple_member_type,
                &member_type,
                check_guard.next_level()?,
            ) {
                Ok(_) => {}
                Err(TypeCheckFailReason::TypeNotMatch) => {
                    return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                        t!(
                            "tuple member %{idx} not match, expect %{typ}, but got %{got}",
                            idx = i + 1,
                            typ = humanize_type(db, source_tuple_member_type, RenderLevel::Simple),
                            got = humanize_type(db, &member_type, RenderLevel::Simple)
                        )
                        .to_string(),
                    ));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if source_tuple_member_type.is_optional() {
            continue;
        } else {
            return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                t!("missing tuple member %{idx}", idx = i + 1).to_string(),
            ));
        }
    }

    Ok(())
}

fn check_tuple_type_compact_object_type(
    db: &DbIndex,
    source_tuple: &LuaTupleType,
    object_type: &LuaObjectType,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    let object_members = object_type.get_fields();

    let tuple_members = source_tuple.get_types();
    let size = tuple_members.len();
    for i in 0..size {
        let source_tuple_member_type = &tuple_members[i];
        let key = LuaMemberKey::Integer((i + 1) as i64);
        if let Some(object_member_type) = object_members.get(&key) {
            match check_general_type_compact(
                db,
                source_tuple_member_type,
                object_member_type,
                check_guard.next_level()?,
            ) {
                Ok(_) => {}
                Err(TypeCheckFailReason::TypeNotMatch) => {
                    return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                        t!(
                            "tuple member %{idx} not match, expect %{typ}, but got %{got}",
                            idx = i + 1,
                            typ = humanize_type(db, source_tuple_member_type, RenderLevel::Simple),
                            got = humanize_type(db, object_member_type, RenderLevel::Simple)
                        )
                        .to_string(),
                    ));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if source_tuple_member_type.is_nullable() || source_tuple_member_type.is_any() {
            continue;
        } else {
            return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                t!("missing tuple member %{idx}", idx = i + 1).to_string(),
            ));
        }
    }

    Ok(())
}

// #[derive(Debug)]
// pub struct TypeListCheckErr(TypeCheckFailReason, usize);

// pub fn check_type_list_compact(
//     db: &DbIndex,
//     source_types: &[LuaType],
//     compact_types: &[LuaType],
//     check_guard: TypeCheckGuard,
// ) -> Result<(), TypeListCheckErr> {

//     Ok(())
// }
