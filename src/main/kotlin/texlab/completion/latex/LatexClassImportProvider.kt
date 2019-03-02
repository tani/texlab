package texlab.completion.latex

import org.eclipse.lsp4j.CompletionItem
import org.eclipse.lsp4j.CompletionParams
import texlab.completion.CompletionItemFactory
import texlab.provider.FeatureRequest
import texlab.resolver.LatexResolver
import texlab.syntax.latex.LatexCommandSyntax

class LatexClassImportProvider(resolver: LatexResolver) : LatexArgumentProvider() {
    private val items = resolver.filesByName.values
            .filter { it.extension == "cls" }
            .map { CompletionItemFactory.createClass(it.nameWithoutExtension) }

    override val commandNames = listOf("""\documentclass""")

    override val argumentIndex = 0

    override fun complete(request: FeatureRequest<CompletionParams>,
                          command: LatexCommandSyntax): List<CompletionItem> {
        return items
    }
}
