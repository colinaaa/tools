# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > statement-if > migrated_0000`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "esprima/statement-if/migrated_0000/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "esprima/statement-if/migrated_0000/input.js"
		end: Object {
			column: 0
			line: 2
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	body: Array [
		JSIfStatement {
			alternate: undefined
			loc: Object {
				filename: "esprima/statement-if/migrated_0000/input.js"
				end: Object {
					column: 26
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			test: JSReferenceIdentifier {
				name: "morning"
				loc: Object {
					filename: "esprima/statement-if/migrated_0000/input.js"
					identifierName: "morning"
					end: Object {
						column: 11
						line: 1
					}
					start: Object {
						column: 4
						line: 1
					}
				}
			}
			consequent: JSExpressionStatement {
				loc: Object {
					filename: "esprima/statement-if/migrated_0000/input.js"
					end: Object {
						column: 26
						line: 1
					}
					start: Object {
						column: 13
						line: 1
					}
				}
				expression: JSCallExpression {
					arguments: Array []
					loc: Object {
						filename: "esprima/statement-if/migrated_0000/input.js"
						end: Object {
							column: 26
							line: 1
						}
						start: Object {
							column: 13
							line: 1
						}
					}
					callee: JSReferenceIdentifier {
						name: "goodMorning"
						loc: Object {
							filename: "esprima/statement-if/migrated_0000/input.js"
							identifierName: "goodMorning"
							end: Object {
								column: 24
								line: 1
							}
							start: Object {
								column: 13
								line: 1
							}
						}
					}
				}
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```