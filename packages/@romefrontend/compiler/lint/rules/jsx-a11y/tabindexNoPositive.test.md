# `tabindexNoPositive.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/compiler/lint/rules/jsx-a11y/tabindexNoPositive.test.ts --update-snapshots` to update.

## `jsx-a11y tabIndex no positive`

### `0`

```

 lint/jsx-a11y/tabindexNoPositive/reject/1/file.tsx:1:6 lint/jsx-a11y/tabindexNoPositive  FIXABLE  ━

  ✖ Avoid positive integer values for the tabIndex attribute.

    <span tabIndex='5'>foo</span>
          ^^^^^^^^^^^^

  ℹ Elements with a positive tab index override natural page content order. This causes elements
    without a positive tab index to come last when navigating using a keyboard.

  ℹ Recommended fix

    1   │ - tabIndex="5"
      1 │ + <span>
      2 │ +   foo
      3 │ + </span>

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `0: formatted`

```
<span tabIndex="5">
	foo
</span>;

```

### `1`

```

 lint/jsx-a11y/tabindexNoPositive/reject/2/file.tsx:1:6 lint/jsx-a11y/tabindexNoPositive  FIXABLE  ━

  ✖ Avoid positive integer values for the tabIndex attribute.

    <span tabIndex={5}>foo</span>
          ^^^^^^^^^^^^

  ℹ Elements with a positive tab index override natural page content order. This causes elements
    without a positive tab index to come last when navigating using a keyboard.

  ℹ Recommended fix

    1   │ - tabIndex={5}
      1 │ + <span>
      2 │ +   foo
      3 │ + </span>

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `1: formatted`

```
<span tabIndex={5}>
	foo
</span>;

```

### `2`

```

 lint/jsx-a11y/tabindexNoPositive/reject/3/file.tsx:1:6 lint/jsx-a11y/tabindexNoPositive  FIXABLE  ━

  ✖ Avoid positive integer values for the tabIndex attribute.

    <span tabIndex={'5'}>foo</span>
          ^^^^^^^^^^^^^^

  ℹ Elements with a positive tab index override natural page content order. This causes elements
    without a positive tab index to come last when navigating using a keyboard.

  ℹ Recommended fix

    1   │ - tabIndex={"5"}
      1 │ + <span>
      2 │ +   foo
      3 │ + </span>

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `2: formatted`

```
<span tabIndex={"5"}>
	foo
</span>;

```

### `3`

```
✔ No known problems!

```

### `3: formatted`

```
<span tabIndex={0}>
	baz
</span>;

```

### `4`

```
✔ No known problems!

```

### `4: formatted`

```
<span tabIndex={-1}>
	baz
</span>;

```

### `5`

```
✔ No known problems!

```

### `5: formatted`

```
<span tabIndex="-1">
	baz
</span>;

```

### `6`

```
✔ No known problems!

```

### `6: formatted`

```
<span tabIndex="0">
	baz
</span>;

```

### `7`

```
✔ No known problems!

```

### `7: formatted`

```
<span tabIndex={dynamic}>
	baz
</span>;

```

### `8`

```
✔ No known problems!

```

### `8: formatted`

```
<span tabIndex={undefined}>
	baz
</span>;

```