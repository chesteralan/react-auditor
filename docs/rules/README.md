# Rules

All rules organized by category.

## Accessibility

| Rule | Default | Description |
|------|---------|-------------|
| [img-alt](img-alt.md) | error | Images must have `alt` text |
| [button-has-type](button-has-type.md) | error | Buttons should have explicit `type` attribute |
| [label-associated](label-associated.md) | warning | Form inputs should have associated labels |
| [aria-valid](aria-valid.md) | error | ARIA attributes must be valid |
| [heading-levels](heading-levels.md) | warning | Heading levels should not be skipped |
| [a-has-content](a-has-content.md) | warning | Anchor and button elements should have text content or an aria-label |
| [no-ambiguous-labels](no-ambiguous-labels.md) | warning | No duplicate or ambiguous label text |
| [tabindex-no-positive](tabindex-no-positive.md) | error | Avoid positive tabIndex values; only 0 and -1 are valid |
| [click-events-have-key-events](click-events-have-key-events.md) | warning | Elements with onClick must have a keyboard event handler |
| [html-has-lang](html-has-lang.md) | error | <html> element must have a lang attribute |
| [no-autofocus](no-autofocus.md) | warning | Avoid autoFocus attribute; it can cause usability issues |

## Nextjs

| Rule | Default | Description |
|------|---------|-------------|
| [no-img-element](no-img-element.md) | warning | Use `next/image` instead of `<img>` for optimized images |
| [no-script-tag-in-head](no-script-tag-in-head.md) | warning | Use `next/script` instead of `<script>` inside `<Head>` |
| [no-page-link](no-page-link.md) | warning | Use `next/link` instead of `<a>` for internal navigation |
| [no-head-element](no-head-element.md) | warning | Use `next/head` (`<Head>`) instead of `<head>` |
| [no-sync-script](no-sync-script.md) | warning | Use `strategy="afterInteractive"` on `<Script>` from `next/script` |

## Performance

| Rule | Default | Description |
|------|---------|-------------|
| [prefer-fragments](prefer-fragments.md) | warning | Use `<></>` over unnecessary wrapper divs |
| [no-bind-in-jsx](no-bind-in-jsx.md) | warning | Avoid `.bind()` or arrow functions in JSX props |
| [no-heavy-computation-in-render](no-heavy-computation-in-render.md) | warning | Avoid heavy synchronous computation in render |
| [lazy-load-components](lazy-load-components.md) | warning | Heavy components should be lazy-loaded with React.lazy |
| [no-large-libraries](no-large-libraries.md) | warning | Avoid importing large libraries when lighter alternatives exist |

## Quality

| Rule | Default | Description |
|------|---------|-------------|
| [no-console](no-console.md) | warning | Avoid console.log in production code |
| [no-empty-blocks](no-empty-blocks.md) | warning | No empty if/while/for/try/catch/finally blocks |
| [no-var](no-var.md) | error | Use const or let instead of var |
| [max-params](max-params.md) | warning | Functions should have at most 3 parameters |
| [no-long-functions](no-long-functions.md) | warning | Functions should not exceed 40 lines |
| [prefer-early-return](prefer-early-return.md) | warning | Prefer early returns over nested if-else |
| [no-commented-code](no-commented-code.md) | warning | Remove dead commented-out code |
| [no-deep-nesting](no-deep-nesting.md) | warning | Avoid nesting deeper than 4 levels |
| [no-magic-numbers](no-magic-numbers.md) | warning | Prefer named constants over magic numbers |
| [consistent-return](consistent-return.md) | warning | Functions should consistently return a value or not |
| [no-unused-vars](no-unused-vars.md) | error | No unused variables, imports, or parameters |
| [no-shadow](no-shadow.md) | warning | No variable shadowing in nested scopes |
| [complexity](complexity.md) | warning | Cyclomatic complexity should not exceed 10 |

## React

| Rule | Default | Description |
|------|---------|-------------|
| [no-missing-key](no-missing-key.md) | error | List items should have a `key` prop |
| [no-inline-styles](no-inline-styles.md) | warning | Avoid inline `style` prop — use CSS classes |
| [consistent-component-naming](consistent-component-naming.md) | warning | Component names should be PascalCase, hooks should be camelCase |
| [no-index-key](no-index-key.md) | warning | Prefer stable IDs over array index as key |
| [no-inline-functions](no-inline-functions.md) | warning | Avoid inline function definitions in JSX props |
| [prefer-function-components](prefer-function-components.md) | warning | Prefer function components over class components |
| [no-unnecessary-memo](no-unnecessary-memo.md) | warning | Avoid useMemo/useCallback for trivial computations |
| [no-multiple-render-methods](no-multiple-render-methods.md) | warning | Component should not have multiple render methods |
| [no-side-effects-in-render](no-side-effects-in-render.md) | error | No side effects (subscriptions, mutations) during render |
| [hook-rules](hook-rules.md) | error | Hooks must follow the Rules of Hooks |
| [no-missing-deps](no-missing-deps.md) | warning | useEffect/useMemo/useCallback should have a dependency array |
| [no-set-state-in-effect](no-set-state-in-effect.md) | warning | Avoid calling setState in useEffect without conditions |
| [no-set-state-in-render](no-set-state-in-render.md) | error | No setState calls directly in render body |
| [jsx-no-duplicate-props](jsx-no-duplicate-props.md) | error | Duplicate props are not allowed in JSX |
| [no-direct-mutation](no-direct-mutation.md) | warning | Avoid direct mutation of state or props — use setState or dispatch |
| [no-ref-in-component-name](no-ref-in-component-name.md) | warning | Component names should not contain 'Ref' |
| [no-forward-ref](no-forward-ref.md) | warning | forwardRef is deprecated in React 19; use ref as a prop instead |

## Security

| Rule | Default | Description |
|------|---------|-------------|
| [no-dangerously-set-innerhtml](no-dangerously-set-innerhtml.md) | error | Avoid `dangerouslySetInnerHTML` |
| [no-eval](no-eval.md) | error | No eval, Function(), or setTimeout with string args |
| [no-script-url](no-script-url.md) | error | No `javascript:` URLs in links |
| [no-hardcoded-secrets](no-hardcoded-secrets.md) | error | Flag potential API keys, tokens, passwords |
| [no-unsanitized-input](no-unsanitized-input.md) | error | Sanitize user input before DOM insertion |
| [no-insecure-protocol](no-insecure-protocol.md) | warning | Avoid `http://` URLs, use `https://` |
| [no-unsafe-iframe](no-unsafe-iframe.md) | warning | iframes should include `sandbox` and `title` attributes |

## Typescript

| Rule | Default | Description |
|------|---------|-------------|
| [no-any](no-any.md) | error | Avoid `any` — use `unknown` or proper types |
| [no-non-null-assertion](no-non-null-assertion.md) | warning | Avoid `!` non-null assertion operator |
| [no-type-assertion](no-type-assertion.md) | warning | Prefer type inference over explicit `as` casts |
| [no-empty-interface](no-empty-interface.md) | warning | No empty interfaces |
| [consistent-type-imports](consistent-type-imports.md) | warning | Use `import type` for type-only imports |
| [explicit-return-type](explicit-return-type.md) | warning | Functions should have explicit return types |
| [strict-null-checks](strict-null-checks.md) | warning | Prefer optional chaining and null checks on nullable values |
| [prefer-interface](prefer-interface.md) | warning | Prefer `interface` over `type` for object types |
| [no-explicit-any](no-explicit-any.md) | error | Avoid explicit `any` type annotations or assertions |

