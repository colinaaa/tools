# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > types > template-literal`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	hasHoistedVars: false
	integrity: undefined
	interpreter: undefined
	sourceType: "module"
	loc: SourceLocation typescript/types/template-literal/input.ts 1:0-2:0
	path: UIDPath<typescript/types/template-literal/input.ts>
	syntax: Array ["ts"]
	body: Array [
		TSTypeAlias {
			id: JSBindingIdentifier {
				name: "A"
				loc: SourceLocation typescript/types/template-literal/input.ts 1:5-1:6 (A)
			}
			typeParameters: undefined
			loc: SourceLocation typescript/types/template-literal/input.ts 1:0-1:25
			right: TSTemplateLiteralTypeAnnotation {
				loc: SourceLocation typescript/types/template-literal/input.ts 1:9-1:24
				quasis: Array [
					TSTemplateElement {
						cooked: ""
						raw: ""
						tail: false
						loc: SourceLocation typescript/types/template-literal/input.ts 1:10-1:10
					}
					TSTemplateElement {
						cooked: "_"
						raw: "_"
						tail: true
						loc: SourceLocation typescript/types/template-literal/input.ts 1:22-1:23
					}
				]
				expressions: Array [
					TSUnionTypeAnnotation {
						loc: SourceLocation typescript/types/template-literal/input.ts 1:12-1:21
						types: Array [
							TSTypeReference {
								typeParameters: undefined
								loc: SourceLocation typescript/types/template-literal/input.ts 1:12-1:15
								typeName: JSReferenceIdentifier {
									name: "Foo"
									loc: SourceLocation typescript/types/template-literal/input.ts 1:12-1:15 (Foo)
								}
							}
							TSTypeReference {
								typeParameters: undefined
								loc: SourceLocation typescript/types/template-literal/input.ts 1:18-1:21
								typeName: JSReferenceIdentifier {
									name: "Bar"
									loc: SourceLocation typescript/types/template-literal/input.ts 1:18-1:21 (Bar)
								}
							}
						]
					}
				]
			}
		}
	]
}
```

### `diagnostics`

```

```