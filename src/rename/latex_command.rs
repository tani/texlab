use futures_boxed::boxed;
use std::collections::HashMap;
use std::sync::Arc;
use texlab_protocol::*;
use texlab_syntax::*;
use texlab_workspace::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LatexCommandPrepareRenameProvider;

impl FeatureProvider for LatexCommandPrepareRenameProvider {
    type Params = TextDocumentPositionParams;
    type Output = Option<Range>;

    #[boxed]
    async fn execute<'a>(
        &'a self,
        request: &'a FeatureRequest<TextDocumentPositionParams>,
    ) -> Option<Range> {
        let position = request.params.position;
        find_command(&request.document().tree, position).map(|cmd| cmd.range())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LatexCommandRenameProvider;

impl FeatureProvider for LatexCommandRenameProvider {
    type Params = RenameParams;
    type Output = Option<WorkspaceEdit>;

    #[boxed]
    async fn execute<'a>(
        &'a self,
        request: &'a FeatureRequest<RenameParams>,
    ) -> Option<WorkspaceEdit> {
        let command = find_command(
            &request.document().tree,
            request.params.text_document_position.position,
        )?;
        let mut changes = HashMap::new();
        for document in request.related_documents() {
            if let SyntaxTree::Latex(tree) = &document.tree {
                let edits: Vec<TextEdit> = tree
                    .commands
                    .iter()
                    .filter(|cmd| cmd.name.text() == command.name.text())
                    .map(|cmd| {
                        TextEdit::new(cmd.name.range(), format!("\\{}", request.params.new_name))
                    })
                    .collect();
                changes.insert(document.uri.clone().into(), edits);
            }
        }
        Some(WorkspaceEdit::new(changes))
    }
}

fn find_command(tree: &SyntaxTree, position: Position) -> Option<Arc<LatexCommand>> {
    if let SyntaxTree::Latex(tree) = tree {
        tree.find_command_by_name(position)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texlab_protocol::RangeExt;
    use texlab_protocol::{Position, Range};

    #[test]
    fn latex() {
        let edit = test_feature(
            LatexCommandRenameProvider,
            FeatureSpec {
                files: vec![
                    FeatureSpec::file("foo.tex", "\\include{bar.tex}\n\\baz"),
                    FeatureSpec::file("bar.tex", "\\baz"),
                ],
                main_file: "foo.tex",
                position: Position::new(1, 2),
                new_name: "qux",
                ..FeatureSpec::default()
            },
        );
        let mut changes = HashMap::new();
        changes.insert(
            FeatureSpec::uri("foo.tex"),
            vec![TextEdit::new(Range::new_simple(1, 0, 1, 4), "\\qux".into())],
        );
        changes.insert(
            FeatureSpec::uri("bar.tex"),
            vec![TextEdit::new(Range::new_simple(0, 0, 0, 4), "\\qux".into())],
        );
        assert_eq!(edit, Some(WorkspaceEdit::new(changes)));
    }

    #[test]
    fn bibtex() {
        let edit = test_feature(
            LatexCommandRenameProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", "@article{foo, bar = baz}")],
                main_file: "foo.bib",
                position: Position::new(0, 14),
                new_name: "qux",
                ..FeatureSpec::default()
            },
        );
        assert_eq!(edit, None);
    }
}
