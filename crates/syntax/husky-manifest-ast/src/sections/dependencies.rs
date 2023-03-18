use husky_toml_ast::TomlSection;
use vec_like::VecMap;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestDependenciesSectionAst {
    idx: TomlSectionAstIdx,
    dependencies: VecMap<ManifestDependencyAst>,
}

impl ManifestDependenciesSectionAst {
    pub fn dependencies(&self) -> &[ManifestDependencyAst] {
        self.dependencies.as_ref()
    }
}

impl<'a> ManifestAstBuilder<'a, TomlSection> {
    pub(crate) fn build_dependencies_section(
        &mut self,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestAstResult<Option<ManifestDependenciesSectionAst>> {
        let Some((idx, dependencies_section_ast)) = self
            .visit_new_normal_section_ast(self.menu().dependencies_section_tile())? else {
            return Ok(None)
        };
        Ok(Some(ManifestDependenciesSectionAst {
            idx,
            dependencies: dependencies_section_ast
                .entries()
                .iter()
                .map(|entry| ManifestDependencyAst::from_toml_section_entry(entry, self, errors))
                .collect(), // ad hoc
        }))
    }
}
