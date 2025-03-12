// Corresponds to internal/checker/relater.go in the Go implementation
// This module handles type relationship checking (assignability, etc.)

use crate::checker::types::*;
use std::rc::Rc;

/// Checks if source_type is assignable to target_type
/// Corresponds to isAssignableTo in internal/checker/relater.go
pub fn is_assignable_to(source_type: &Type, target_type: &Type) -> bool {
    // Any is assignable to and from anything
    if *source_type == Type::Any || *target_type == Type::Any {
        return true;
    }

    // Error is not assignable to anything
    if *source_type == Type::Error {
        return false;
    }

    // Same types are assignable
    if source_type == target_type {
        return true;
    }

    // Handle arrays and objects
    match (source_type, target_type) {
        // Array type compatibility
        (Type::Array(src_elem_type), Type::Array(tgt_elem_type)) => {
            // Check element type compatibility
            is_assignable_to(src_elem_type, tgt_elem_type)
        }

        // Object to array is not assignable
        (Type::Object(_), Type::Array(_)) => false,

        // Object to interface
        (Type::Object(src_props), Type::Interface(_, interface_props)) => {
            match src_props {
                // Empty object cannot be assigned to an interface that requires properties
                None => interface_props.is_empty(),

                // Check if the object has all required interface properties
                Some(props) => {
                    for (iface_prop_name, iface_prop_type) in interface_props {
                        let matching_prop = props.iter().find(|(name, _)| name == iface_prop_name);

                        match matching_prop {
                            Some((_, prop_type)) => {
                                if !is_assignable_to(prop_type, iface_prop_type) {
                                    return false; // Property type doesn't match interface requirement
                                }
                            }
                            None => return false, // Required interface property missing
                        }
                    }
                    true
                }
            }
        }

        // Empty object can be assigned to any object type
        (Type::Object(None), Type::Object(_)) => true,

        // Object with properties to object with properties
        (Type::Object(Some(src_props)), Type::Object(Some(tgt_props))) => {
            // Check if source has all required properties from target with compatible types
            for (tgt_name, tgt_type) in tgt_props {
                let matching_src_prop = src_props.iter().find(|(name, _)| name == tgt_name);

                match matching_src_prop {
                    Some((_, src_type)) => {
                        if !is_assignable_to(src_type, tgt_type) {
                            return false; // Property type mismatch
                        }
                    }
                    None => return false, // Required property missing
                }
            }
            true
        }

        // Object to empty object
        (Type::Object(_), Type::Object(None)) => true,

        // Interface to interface
        (Type::Interface(_, src_props), Type::Interface(_, tgt_props)) => {
            // Check if source interface has all properties of target interface
            for (tgt_name, tgt_type) in tgt_props {
                let matching_src_prop = src_props.iter().find(|(name, _)| name == tgt_name);

                match matching_src_prop {
                    Some((_, src_type)) => {
                        if !is_assignable_to(src_type, tgt_type) {
                            return false;
                        }
                    }
                    None => return false, // Target interface requires a property not in source
                }
            }
            true
        }

        // Interface to object
        (Type::Interface(_, interface_props), Type::Object(obj_props)) => {
            match obj_props {
                // Interface to empty object - only valid if interface has no required props
                None => interface_props.is_empty(),

                // Interface to object with properties
                Some(props) => {
                    // Check if interface satisfies all required object properties
                    for (obj_prop_name, obj_prop_type) in props {
                        let matching_prop = interface_props
                            .iter()
                            .find(|(name, _)| name == obj_prop_name);

                        match matching_prop {
                            Some((_, prop_type)) => {
                                if !is_assignable_to(prop_type, obj_prop_type) {
                                    return false;
                                }
                            }
                            None => return false, // Interface doesn't have required property
                        }
                    }
                    true
                }
            }
        }

        // In TypeScript, numbers can be coerced to strings during string concatenation,
        // but a Number type is not assignable to a String parameter
        (Type::Number, Type::String) => false,

        // Similarly, booleans are not assignable to strings in TypeScript
        (Type::Boolean, Type::String) => false,

        // By default, different types are not assignable
        _ => false,
    }
}

/// Helper to format types for display in error messages
pub fn format_type(typ: &Type) -> String {
    match typ {
        Type::Any => "any".to_string(),
        Type::Error => "error".to_string(),
        Type::String => "string".to_string(),
        Type::Number => "number".to_string(),
        Type::Boolean => "boolean".to_string(),
        Type::_Void => "void".to_string(),
        Type::_Undefined => "undefined".to_string(),
        Type::_Null => "null".to_string(),
        Type::Array(elem_type) => format!("{}[]", format_type(elem_type)),
        Type::Object(None) => "{}".to_string(),
        Type::Object(Some(props)) => {
            if props.is_empty() {
                "{}".to_string()
            } else {
                let properties = props
                    .iter()
                    .map(|(name, typ)| format!("{}: {}", name, format_type(typ)))
                    .collect::<Vec<_>>()
                    .join("; ");
                format!("{{ {} }}", properties)
            }
        }
        Type::Interface(name, _) => name.clone(),
        Type::Function(signature) => {
            let params = signature
                .parameters
                .iter()
                .map(|param| format_type(param))
                .collect::<Vec<_>>()
                .join(", ");
            let return_type = format_type(&signature.return_type);
            format!("({}) => {}", params, return_type)
        }
    }
}
