use rscript::ModuleInfo;
use serde::{Deserialize, Serialize};
use serde_json::{from_str as from_json, Result as JsonResult};

#[derive(Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub body: String,
    pub children: Vec<Module>,
}

pub fn module_from_json(json: &str) -> JsonResult<ModuleInfo> {
    let parsed: Module = from_json(json)?;
    let info: ModuleInfo = parsed.into();
    Ok(info)
}

impl Into<ModuleInfo> for Module {
    fn into(self) -> ModuleInfo {
        let Module {
            name,
            body,
            children,
        } = self;
        let children: Vec<_> = children.into_iter().map(|module| module.into()).collect();
        ModuleInfo {
            name,
            body,
            children,
        }
    }
}
