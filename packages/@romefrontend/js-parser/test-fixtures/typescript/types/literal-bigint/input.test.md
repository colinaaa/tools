# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > types > literal-bigint`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: true
	directives: Array []
	filename: "input.ts"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "module"
	syntax: Array ["ts"]
	loc: Object {
		filename: "input.ts"
		end: Object {
			column: 0
			index: 11
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
				message: PARTIAL_BLESSED_DIAGNOSTIC_MESSAGE {value: "Unknown TS non array type start"}
			}
			location: Object {
				filename: "input.ts"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 6
					index: 6
					line: 1
				}
				start: Object {
					column: 7
					index: 7
					line: 1
				}
			}
		}
	]
	body: Array [
		JSVariableDeclarationStatement {
			loc: Object {
				filename: "input.ts"
				end: Object {
					column: 10
					index: 10
					line: 1
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			declaration: JSVariableDeclaration {
				kind: "let"
				loc: Object {
					filename: "input.ts"
					end: Object {
						column: 10
						index: 10
						line: 1
					}
					start: Object {
						column: 0
						index: 0
						line: 1
					}
				}
				declarations: Array [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "x"
							loc: Object {
								filename: "input.ts"
								end: Object {
									column: 9
									index: 9
									line: 1
								}
								start: Object {
									column: 4
									index: 4
									line: 1
								}
							}
							meta: JSPatternMeta {
								definite: undefined
								loc: Object {
									filename: "input.ts"
									end: Object {
										column: 9
										index: 9
										line: 1
									}
									start: Object {
										column: 4
										index: 4
										line: 1
									}
								}
								typeAnnotation: TSTypeReference {
									loc: Object {
										filename: "input.ts"
										end: Object {
											column: 9
											index: 9
											line: 1
										}
										start: Object {
											column: 9
											index: 9
											line: 1
										}
									}
									typeName: JSReferenceIdentifier {
										name: "INVALID_PLACEHOLDER"
										loc: Object {
											filename: "input.ts"
											end: Object {
												column: 9
												index: 9
												line: 1
											}
											start: Object {
												column: 9
												index: 9
												line: 1
											}
										}
									}
								}
							}
						}
						init: undefined
						loc: Object {
							filename: "input.ts"
							end: Object {
								column: 9
								index: 9
								line: 1
							}
							start: Object {
								column: 4
								index: 4
								line: 1
							}
						}
					}
				]
			}
		}
	]
}
```

### `diagnostics`

```

 input.ts:1:7 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unknown TS non array type start

    let x: 0n;
           ^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```