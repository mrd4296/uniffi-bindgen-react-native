use anyhow::bail;
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
 */
use uniffi_bindgen::{interface::{DefaultValue, Literal}, ComponentInterface};

use super::oracle::{CodeOracle, CodeType, Result};

#[derive(Debug)]
pub struct EnumCodeType {
    id: String,
}

impl EnumCodeType {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl CodeType for EnumCodeType {
    fn type_label(&self, ci: &ComponentInterface) -> String {
        let nm = CodeOracle.class_name(ci, &self.id);
        if ci.is_name_used_as_error(&self.id) {
            rewrite_error_name(&nm).to_string()
        } else {
            nm
        }
    }

    fn decl_type_label(&self, ci: &ComponentInterface) -> String {
        let nm = CodeOracle.class_name(ci, &self.id);
        if ci.is_name_used_as_error(&self.id) {
            rewrite_error_name(&nm).to_string()
        } else {
            nm
        }
    }

    fn canonical_name(&self) -> String {
        format!("Type{}", self.id)
    }

    fn default(&self, default: &DefaultValue, ci: &ComponentInterface) -> Result<String> {
        if let DefaultValue::Literal(Literal::Enum(v, _)) = default {
            Ok(format!(
                "{}.{}",
                self.type_label(ci),
                CodeOracle.enum_variant_name(v)
            ))
        } else {
            bail!("Invalid literal for enum type: {default:?}")
        }
    }
}

fn rewrite_error_name(nm: &str) -> &str {
    if nm == "Error" {
        "Exception"
    } else {
        nm
    }
}
