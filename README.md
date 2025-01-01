# zed-lean4

- [Language Extensions](https://zed.dev/docs/extensions/languages.html)
- [Zed Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/)

## How to type unicode symbols in

- Use simple-completion extension
- Use snippet (CONFIG/snippets/lean4.json) like this:

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
