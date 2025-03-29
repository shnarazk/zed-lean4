# zed-lean4

- [Language Extensions](https://zed.dev/docs/extensions/languages.html)
- [Zed Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/)
- [tree-sitter-lean](https://github.com/Julian/tree-sitter-lean)

## What is this extension?

This extension provides minimal programming aids like syntax highlights, fold, hovered symbol description and so on via lsp and tree-sitter.
You can't use Zed to proof something since there is no 'info window' to show proof diagram. 

## How to type Unicode symbols in

- Use a latex_commands-to-unicode_symbols converter like [lucr](https://github.com/shnarazk/lucr)
- Use simple-completion extension (But I don't know how to configure it)
- Use snippet (CONFIG/snippets/lean4.json) like this (But a newline is inserted after completion):

```json
{
  "arrow-left": {
    "prefix": "left",
    "body": ["←", "$0"],
    "description": "left arrow: ←"
  },
  "arrow-right": {
    "prefix": "right",
    "body": ["→", "$0"],
    "description": "right arrow: →"
  },
  "exists": {
    "prefix": "exists",
    "body": ["∃ $1, $2", "$0"],
    "description": "exists: ∃"
  },
  "forall": {
    "prefix": "forall",
    "body": ["∀ $1, $2", "$0"],
    "description": "forall: ∀"
  },
  "function": {
    "prefix": "fun",
    "body": ["(fun $1 ($2) ↦ $3)", "$0"],
    "description": "function: (fun f () ↦ )"
  },
  "namespace": {
    "prefix": "namespace",
    "body": ["namespace $1\n\nend $1", "$0"],
    "description": "namespace block"
  },
  "pair": {
    "prefix": "pair",
    "body": ["⟨$1, $2⟩", "$0"],
    "description": "pair: ⟨,⟩"
  },
  "times": {
    "prefix": "times",
    "body": ["×", "$0"],
    "description": "times: ×"
  }
}
```
