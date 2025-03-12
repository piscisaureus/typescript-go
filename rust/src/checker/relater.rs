// Corresponds to internal/checker/relater.go in the Go implementation
// This module handles type relationship checking (assignability, etc.)
// In Go, this functionality is spread across multiple files within the checker package

use crate::checker::types::Type;

/// Helper function to check if a type is a primitive type
/// Corresponds to internal/checker/relater.go isPrimitiveType() function
fn is_primitive_type(typ: &Type) -> bool {
    matches!(
        typ,
        Type::String | Type::Number | Type::Boolean | Type::_Void | Type::_Null | Type::_Undefined
    )
}

/// Helper function to check if a property is known in a given type
/// Corresponds to isKnownProperty() in internal/checker/relater.go
pub fn is_known_property(target_type: &Type, property_name: &str) -> bool {
    match target_type {
        Type::Object(Some(props)) => {
            // Check if property exists in object properties
            props.iter().any(|(name, _)| name == property_name)
        }
        Type::Interface(_, props) => {
            // Check if property exists in interface properties
            props.iter().any(|(name, _)| name == property_name)
        }
        Type::GenericInterface(_, _, props) => {
            // Check if property exists in generic interface properties
            props.iter().any(|(name, _)| name == property_name)
        }
        Type::Union(types) => {
            // For union types, property must exist in at least one constituent type
            types.iter().any(|t| is_known_property(t, property_name))
        }
        Type::TypeParameter(_) => {
            // Type parameters don't have known properties
            false
        }
        _ => false,
    }
}

/// Checks if source_type is assignable to target_type
/// Corresponds to isAssignableTo() in internal/checker/relater.go
/// This is a key function in the type checker that determines type compatibility
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

    // Object is never assignable to primitive types
    if let Type::Object(_) = source_type {
        if is_primitive_type(target_type) {
            return false;
        }

        // Special case: also check if target is a union consisting only of primitive types
        if let Type::Union(target_types) = target_type {
            if target_types.iter().all(|t| is_primitive_type(t)) {
                return false; // Object can't be assigned to a union of only primitive types
            }
        }
    }

    // Handle union types first - this is the core of union type assignment rules
    match (source_type, target_type) {
        // Union target type: source must be assignable to ANY of the union's constituent types
        (_, Type::Union(target_types)) => {
            // Source is assignable to a union if it's assignable to any of its members
            target_types
                .iter()
                .any(|t| is_assignable_to(source_type, t))
        }

        // Union source type: ALL of the union's constituent types must be assignable to target
        (Type::Union(source_types), _) => {
            // A union is assignable to a target if all of its members are assignable to the target
            source_types
                .iter()
                .all(|t| is_assignable_to(t, target_type))
        }

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
                    // First check that all required interface properties exist in the object
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

                    // Then check that the object doesn't have properties not in the interface
                    // This is needed for the case of object literals assigned to interface types
                    for (prop_name, _) in props {
                        if !interface_props
                            .iter()
                            .any(|(iface_name, _)| iface_name == prop_name)
                        {
                            return false; // Object has a property not in the interface
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
/// Corresponds to internal/checker/types.go String() methods on various type implementations
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
        Type::Interface(name, _) => {
            // Only return the name if it's a real interface name (not an empty string)
            if name.is_empty() {
                // This is probably an anonymous interface
                "interface{}".to_string()
            } else {
                name.clone()
            }
        }
        Type::TypeParameter(name) => {
            // Format as the type parameter name
            name.clone()
        }
        Type::GenericInterface(name, type_params, _) => {
            // Format as "Array<T>" or "Map<K, V>"
            if type_params.is_empty() {
                name.clone()
            } else {
                let params = type_params.join(", ");
                format!("{}<{}>", name, params)
            }
        }
        Type::Union(types) => {
            // Format as "T1 | T2 | T3"
            types
                .iter()
                .map(|t| format_type(t))
                .collect::<Vec<_>>()
                .join(" | ")
        }
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
