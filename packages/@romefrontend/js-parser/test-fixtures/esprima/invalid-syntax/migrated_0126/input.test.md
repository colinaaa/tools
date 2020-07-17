# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0126`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: true
	directives: Array []
	filename: "input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "input.js"
		end: Object {
			column: 0
			index: 16
			line: 2
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	diagnostics: Array [
		Object {
			origins: Array [Object {category: "parse/js"}]
			description: Object {
				advice: Array []
				category: "parse/js"
				message: PARTIAL_BLESSED_DIAGNOSTIC_MESSAGE {value: "Invalid left-hand side in for-in statement"}
			}
			location: Object {
				filename: "input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 7
					index: 7
					line: 1
				}
				start: Object {
					column: 5
					index: 5
					line: 1
				}
			}
		}
	]
	body: Array [
		JSForInStatement {
			loc: Object {
				filename: "input.js"
				end: Object {
					column: 15
					index: 15
					line: 1
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			body: JSEmptyStatement {
				loc: Object {
					filename: "input.js"
					end: Object {
						column: 15
						index: 15
						line: 1
					}
					start: Object {
						column: 14
						index: 14
						line: 1
					}
				}
			}
			left: JSAssignmentIdentifier {
				name: "INVALID_PLACEHOLDER"
				loc: Object {
					filename: "input.js"
					end: Object {
						column: 7
						index: 7
						line: 1
					}
					start: Object {
						column: 8
						index: 8
						line: 1
					}
				}
			}
			right: JSObjectExpression {
				properties: Array []
				loc: Object {
					filename: "input.js"
					end: Object {
						column: 13
						index: 13
						line: 1
					}
					start: Object {
						column: 11
						index: 11
						line: 1
					}
				}
			}
		}
	]
}
```

### `diagnostics`

```

 input.js:1:5 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Invalid left-hand side in for-in statement

    for (+i in {});
         ^^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```