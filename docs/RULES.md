# Rules Catalog

All 71 rules the scanner checks against, organized by category. Rules with a 🔧 icon support `--fix`.

---

## Code Quality (13 rules)

| Rule ID | Severity | Description |
|---------|----------|-------------|
| `no-console` 🔧 | warn | Avoid console.log in production code |
| `no-empty-blocks` 🔧 | warn | No empty catch/if/function blocks |
| `no-var` 🔧 | error | Use const/let instead of var |
| `max-params` | warn | Functions should have at most 3 parameters |
| `no-long-functions` | warn | Functions should not exceed 40 lines |
| `prefer-early-return` | warn | Prefer early returns to reduce nesting |
| `no-commented-code` | warn | Remove dead commented-out code |
| `no-deep-nesting` | warn | Avoid nesting deeper than 4 levels |
| `no-magic-numbers` | warn | Prefer named constants over magic numbers/strings |
| `consistent-return` | warn | Functions should consistently return a value or not |
| `no-unused-vars` | error | No unused variables, imports, or parameters |
| `no-shadow` | warn | No variable shadowing in nested scopes |
| `complexity` | warn | Cyclomatic complexity should not exceed 10 |

## React (19 rules)

| Rule ID | Severity | Description |
|---------|----------|-------------|
| `no-missing-key` | error | List items should have a `key` prop |
| `no-inline-styles` | warn | Avoid inline `style` prop — use CSS classes |
| `consistent-component-naming` | warn | Component names should be PascalCase, hooks camelCase |
| `no-index-key` | warn | Prefer stable IDs over array index as key |
| `no-inline-functions` | warn | Avoid inline function definitions in JSX props |
| `prefer-function-components` 🔧 | warn | Prefer function components over class components |
| `no-unnecessary-memo` | warn | Avoid useMemo/useCallback for trivial computations |
| `no-multiple-render-methods` | warn | Component should not have multiple render methods |
| `no-side-effects-in-render` | error | No side effects during render |
| `hook-rules` | error | Hooks must follow Rules of Hooks |
| `no-missing-deps` | warn | useEffect/useCallback/useMemo should include a dependency array |
| `no-set-state-in-effect` | warn | Avoid setState inside useEffect without dependency |
| `no-set-state-in-render` | error | No setState or dispatch during render |
| `no-direct-mutation` | error | Detect direct props/state mutation outside setState |
| `jsx-no-duplicate-props` | error | Flag repeated props like `<div id="a" id="b" />` |
| `no-ref-in-component-name` | warn | Component names shouldn't contain "Ref" |
| `no-forward-ref` | warn | Use forwardRef when passing refs to child components |
| `no-array-index-key` | warn | Avoid using array index as key — prefer stable ID |
| `no-state-in-default-props` | warn | Don't derive default props from component state |

## TypeScript (9 rules)

| Rule ID | Severity | Description |
|---------|----------|-------------|
| `no-any` | error | Avoid `any` — use `unknown` or proper types |
| `no-non-null-assertion` | warn | Avoid `!` non-null assertion operator |
| `no-type-assertion` | warn | Prefer type inference over explicit `as` casts |
| `no-empty-interface` | warn | No empty interfaces |
| `consistent-type-imports` | warn | Use `import type` for type-only imports |
| `explicit-return-type` | warn | Functions should have explicit return types |
| `strict-null-checks` | error | Variables should handle null/undefined explicitly |
| `prefer-interface` | warn | Prefer `interface` over `type` for object shapes |
| `no-explicit-any` | error | Stricter any-catcher — catches `as any`, `<any>`, type annotations |

## Security (7 rules)

| Rule ID | Severity | Description |
|---------|----------|-------------|
| `no-dangerously-set-innerhtml` | error | Avoid `dangerouslySetInnerHTML` |
| `no-eval` | error | No eval, Function(), or setTimeout with string args |
| `no-script-url` | error | No `javascript:` URLs in links |
| `no-hardcoded-secrets` | error | Flag potential API keys, tokens, passwords |
| `no-unsanitized-input` | error | Input should be sanitized before rendering |
| `no-insecure-protocol` | warn | Prefer https:// over http:// |
| `no-unsafe-iframe` | warn | Warn on `<iframe>` without `sandbox` or `title` |

## Next.js (5 rules)

| Rule ID | Severity | Description |
|---------|----------|-------------|
| `no-img-element` | warn | Use `<Image />` instead of `<img>` |
| `no-script-tag-in-head` | warn | Use `<Script />` instead of `<script>` inside `<Head>` |
| `no-page-link` | warn | Use `<Link>` instead of `<a>` for internal navigation |
| `no-head-element` | warn | Use `<Head />` instead of `<head>` |
| `no-sync-script` | warn | Flag synchronous `<Script>` without `strategy="afterInteractive"` |

## Performance (5 rules)

| Rule ID | Severity | Description |
|---------|----------|-------------|
| `prefer-fragments` 🔧 | warn | Use `<></>` or `<Fragment>` over unnecessary wrapper divs |
| `no-bind-in-jsx` | warn | Avoid `.bind()` or arrow functions in JSX props |
| `no-heavy-computation-in-render` | warn | Heavy computations should be memoized |
| `lazy-load-components` | warn | Lazy-load heavy components with `React.lazy` |
| `no-large-libraries` | warn | Warn on importing heavy libraries (moment, lodash) when lighter alternatives exist |

## Accessibility (11 rules)

| Rule ID | Severity | Description |
|---------|----------|-------------|
| `img-alt` | error | Images must have `alt` text |
| `button-has-type` | warn | Buttons should have explicit `type` attribute |
| `label-associated` | warn | Form inputs should have associated labels |
| `aria-valid` | error | ARIA attributes should be valid and properly used |
| `heading-levels` | warn | Headings should follow a logical hierarchy without skipping levels |
| `a-has-content` | warn | `<a>` or `<button>` should have text content or `aria-label` |
| `no-ambiguous-labels` | warn | Flag duplicate or ambiguous label text |
| `tabindex-no-positive` | error | Avoid positive `tabindex` values |
| `click-events-have-key-events` | error | Clickable elements should have keyboard event handlers |
| `html-has-lang` | error | `<html>` element should have a `lang` attribute |
| `no-autofocus` | warn | Avoid `autoFocus` for accessibility concerns |

## Testing (2 rules)

| Rule ID | Severity | Description |
|---------|----------|-------------|
| `no-skipped-tests` | warn | Flag `it.skip`, `describe.skip`, `xit`, `xdescribe` in test files |
| `assert-includes-message` | warn | Assertions should include a descriptive message |

---

> Run `react-auditor --docs` to generate full per-rule documentation in `docs/rules/`.
