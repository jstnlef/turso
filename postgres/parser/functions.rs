//! What the translator knows about PostgreSQL's built-in functions before
//! the engine has resolved anything: whether a function always returns the
//! same value for the same arguments, whether it returns a set of rows, and
//! whether it is an aggregate.

use phf::phf_map;

/// A PostgreSQL built-in function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgFunc {
    /// Whether the function returns the same value for the same arguments
    /// within and across statements. STABLE and VOLATILE functions are not.
    pub immutable: bool,
    /// Whether the function returns a set of rows.
    pub set_returning: bool,
    /// Whether the function is an aggregate.
    pub aggregate: bool,
}

/// The built-in function called `name`, compared without regard to case.
/// A name not in the registry is an immutable, non-set-returning function.
pub fn pg_func(name: &str) -> Option<&'static PgFunc> {
    PG_FUNCS.get(name.to_ascii_lowercase().as_str())
}

#[rustfmt::skip]
static PG_FUNCS: phf::Map<&'static str, PgFunc> = phf_map! {
    "abs" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "age" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "array_append" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "array_cat" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "array_length" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "array_lower" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "array_position" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "array_prepend" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "array_remove" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "array_to_string" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "array_upper" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "ascii" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "bit_length" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "btrim" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "cbrt" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "ceil" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "ceiling" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "char_length" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "character_length" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "chr" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "clock_timestamp" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "coalesce" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "concat" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "concat_ws" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "current_catalog" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "current_database" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "current_date" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "current_query" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "current_role" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "current_schema" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "current_schemas" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "current_setting" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "current_time" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "current_timestamp" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "current_user" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "currval" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "date_part" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "date_trunc" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "decode" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "degrees" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "div" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "encode" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "exp" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "extract" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "floor" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "format" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "gen_random_uuid" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "generate_series" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "generate_subscripts" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "greatest" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "has_column_privilege" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "has_database_privilege" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "has_function_privilege" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "has_schema_privilege" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "has_sequence_privilege" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "has_table_privilege" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "inet_client_addr" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "inet_client_port" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "inet_server_addr" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "inet_server_port" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "initcap" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "json_array_elements" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "json_array_elements_text" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "json_array_length" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "json_build_array" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "json_build_object" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "json_each" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "json_each_text" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "json_extract_path" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "json_extract_path_text" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "json_object_keys" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "json_populate_recordset" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "json_to_recordset" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "json_typeof" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "jsonb_array_elements" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "jsonb_array_elements_text" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "jsonb_array_length" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "jsonb_build_array" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "jsonb_build_object" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "jsonb_each" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "jsonb_each_text" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "jsonb_extract_path" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "jsonb_extract_path_text" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "jsonb_object_keys" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "jsonb_path_query" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "jsonb_populate_recordset" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "jsonb_set" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "jsonb_strip_nulls" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "jsonb_to_recordset" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "jsonb_typeof" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "lastval" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "least" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "left" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "length" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "ln" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "localtime" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "localtimestamp" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "log" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "lower" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "lpad" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "ltrim" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "md5" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "mod" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "nextval" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "now" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "nullif" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "octet_length" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "overlay" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "pg_advisory_lock" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_advisory_unlock" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_advisory_unlock_all" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_advisory_xact_lock" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_backend_pid" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_cancel_backend" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_conf_load_time" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_current_wal_lsn" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_database_size" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_get_constraintdef" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_get_expr" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_get_indexdef" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_get_userbyid" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_get_viewdef" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_has_role" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_is_in_recovery" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_listening_channels" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "pg_notify" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_postmaster_start_time" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_relation_size" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_sleep" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_sleep_for" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_sleep_until" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_table_is_visible" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_table_size" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_terminate_backend" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_total_relation_size" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_try_advisory_lock" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pg_typeof" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "pi" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "position" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "power" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "quote_ident" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "quote_literal" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "quote_nullable" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "radians" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "random" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "random_normal" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "regexp_matches" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "regexp_replace" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "regexp_split_to_array" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "regexp_split_to_table" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "repeat" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "replace" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "reverse" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "right" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "round" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "rpad" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "rtrim" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "session_user" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "set_config" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "setseed" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "setval" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "sign" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "split_part" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "sqrt" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "statement_timestamp" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "string_to_array" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "string_to_table" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "strpos" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "substr" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "substring" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "timeofday" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "to_char" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "to_date" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "to_hex" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "to_number" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "to_timestamp" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "transaction_timestamp" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "translate" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "trim" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "trunc" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "txid_current" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "unnest" => PgFunc { immutable: true, set_returning: true, aggregate: false },
    "upper" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "user" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "uuid_generate_v1" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "uuid_generate_v4" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "version" => PgFunc { immutable: false, set_returning: false, aggregate: false },
    "width_bucket" => PgFunc { immutable: true, set_returning: false, aggregate: false },
    "array_agg" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "avg" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "bit_and" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "bit_or" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "bit_xor" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "bool_and" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "bool_or" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "count" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "every" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "json_agg" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "json_object_agg" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "jsonb_agg" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "jsonb_object_agg" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "max" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "min" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "stddev" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "stddev_pop" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "stddev_samp" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "string_agg" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "sum" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "var_pop" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "var_samp" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "variance" => PgFunc { immutable: true, set_returning: false, aggregate: true },
    "xmlagg" => PgFunc { immutable: true, set_returning: false, aggregate: true },
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_registered_name_is_lowercase() {
        for name in PG_FUNCS.keys() {
            assert_eq!(*name, name.to_ascii_lowercase());
        }
    }

    #[test]
    fn lookup_ignores_case() {
        let func = pg_func("NOW").unwrap();
        assert!(!func.immutable);
        assert!(!func.set_returning);
        assert_eq!(pg_func("Generate_Series"), pg_func("generate_series"));
    }

    #[test]
    fn volatile_functions_are_not_immutable() {
        for name in [
            "now",
            "random",
            "currval",
            "nextval",
            "clock_timestamp",
            "timeofday",
            "statement_timestamp",
            "transaction_timestamp",
            "pg_sleep",
            "inet_client_addr",
            "inet_server_addr",
            "pg_postmaster_start_time",
            "uuid_generate_v4",
        ] {
            let func = pg_func(name).unwrap_or_else(|| panic!("{name} is not registered"));
            assert!(!func.immutable, "{name} must not be immutable");
            assert!(!func.set_returning, "{name} must not return a set");
        }
    }

    #[test]
    fn set_returning_functions_return_sets() {
        for name in [
            "generate_series",
            "generate_subscripts",
            "unnest",
            "json_array_elements",
            "json_array_elements_text",
            "json_each",
            "json_each_text",
            "json_object_keys",
            "jsonb_array_elements",
            "jsonb_array_elements_text",
            "jsonb_each",
            "jsonb_each_text",
            "jsonb_object_keys",
            "regexp_matches",
            "regexp_split_to_table",
        ] {
            let func = pg_func(name).unwrap_or_else(|| panic!("{name} is not registered"));
            assert!(func.set_returning, "{name} must return a set");
        }
    }

    #[test]
    fn unknown_function_is_not_registered() {
        assert_eq!(pg_func("my_function"), None);
        assert_eq!(pg_func(""), None);
    }
}
