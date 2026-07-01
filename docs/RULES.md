# Rules Catalog

All rules the scanner checks against, organized by category.

---

## Code Quality & Clean Code

| Rule ID | Description |
|---------|-------------|
| `no-long-functions` | Functions should not exceed 40 lines |
| `no-deep-nesting` | Avoid nesting deeper than 4 levels |
| `no-magic-numbers` | Prefer named constants over magic numbers/strings |
| `no-duplicate-code` | Flag duplicate or near-duplicate code blocks |
| `no-commented-code` | Remove dead commented-out code |
| `no-console` | Avoid console.log in production code |
| `no-empty-blocks` | No empty catch/if/function blocks |
| `prefer-early-return` | Prefer early returns to reduce nesting |
| `max-params` | Functions should have at most 3 parameters |
| `consistent-return` | Functions should consistently return a value or not |
| `no-unused-vars` | No unused variables, imports, or parameters |
| `no-var` | Use const/let instead of var |
| `prefer-const` | Prefer const for bindings never reassigned |
| `no-shadow` | No variable shadowing in nested scopes |
| `complexity` | Cyclomatic complexity should not exceed 10 |

## React Best Practices

| Rule ID | Description |
|---------|-------------|
| `use-state-immutably` | State must be updated immutably — no direct mutation |
| `no-set-state-in-effect` | Avoid setState inside useEffect without dependency |
| `effect-deps-complete` | useEffect/useCallback/useMemo should include all deps |
| `no-missing-key` | List items should have a `key` prop |
| `no-index-key` | Prefer stable IDs over array index as key |
| `no-inline-functions` | Avoid inline function definitions in JSX props |
| `no-inline-styles` | Avoid inline `style` prop — use CSS classes |
| `hook-rules` | Hooks must follow Rules of Hooks (top-level, called in same order) |
| `no-set-state-in-render` | No setState or dispatch during render |
| `no-direct-ref-access` | Use `forwardRef` instead of accessing `ref` directly |
| `prop-types` | Components should define PropTypes (or use TypeScript) |
| `no-string-refs` | Use createRef or callback refs, not string refs |
| `no-children-prop` | Use JSX children instead of explicit `children` prop |
| `prefer-function-components` | Prefer function components over class components |
| `no-unnecessary-memo` | Avoid `useMemo`/`useCallback` for trivial computations |
| `no-state-in-default-props` | Don't derive state from default props |
| `no-multiple-render-methods` | Component should not have multiple render methods |
| `consistent-component-naming` | Component names should be PascalCase, hooks camelCase |
| `no-side-effects-in-render` | No side effects (subscriptions, mutations) during render |

## TypeScript Strictness

| Rule ID | Description |
|---------|-------------|
| `no-any` | Avoid `any` — use `unknown` or proper types |
| `no-non-null-assertion` | Avoid `!` non-null assertion operator |
| `no-type-assertion` | Prefer type inference over explicit `as` casts |
| `strict-null-checks` | Variables should handle null/undefined explicitly |
| `prefer-interface` | Prefer `interface` over `type` for object shapes |
| `prefer-type-alias` | Prefer `type` over `interface` for unions/intersections |
| `no-empty-interface` | No empty interfaces |
| `consistent-type-imports` | Use `import type` for type-only imports |
| `no-unused-types` | No unused type definitions |
| `explicit-return-type` | Functions should have explicit return types |
| `no-optional-property-in-params` | Avoid optional properties in function parameter objects |
| `no-parameter-properties` | Avoid constructor parameter properties |

## Security

| Rule ID | Description |
|---------|-------------|
| `no-dangerously-set-innerhtml` | Avoid `dangerouslySetInnerHTML` |
| `no-script-url` | No `javascript:` URLs in links |
| `no-unsanitized-input` | Input should be sanitized before rendering |
| `no-hardcoded-secrets` | Flag potential API keys, tokens, passwords |
| `no-insecure-protocol` | Prefer https:// over http:// |
| `no-unsafe-url` | Flag user-provided URLs used directly in navigation |
| `sanitize-html` | Use DOMPurify or similar for raw HTML rendering |
| `no-eval` | No eval, Function(), or setTimeout with string args |

## Performance

| Rule ID | Description |
|---------|-------------|
| `no-large-dependencies` | Avoid importing large libraries for trivial use |
| `no-unnecessary-renders` | Components re-rendering unnecessarily (missing React.memo) |
| `inline-css-animations` | Avoid CSS-in-JS animations — prefer CSS classes |
| `no-lodash-chaining` | Avoid lodash chain — use native methods |
| `no-heavy-computation-in-render` | Heavy computations should be memoized |
| `prefer-fragments` | Use `<></> or `<Fragment>` over unnecessary wrapper divs |
| `no-expensive-lists` | Large lists should be virtualized |
| `lazy-load-components` | Lazy-load heavy components with `React.lazy` |
| `no-bind-in-jsx` | Avoid `.bind()` or arrow functions in JSX props |
| `image-optimization` | Images should have width/height attributes to prevent layout shift |

## Accessibility

| Rule ID | Description |
|---------|-------------|
| `img-alt` | Images must have `alt` text |
| `button-has-type` | Buttons should have explicit `type` attribute |
| `aria-valid` | ARIA attributes should be valid and properly used |
| `heading-levels` | Headings should follow a logical hierarchy without skipping levels |
| `label-associated` | Form inputs should have associated labels |
| `no-missing-form-role` | Interactive elements should have appropriate roles |
| `keyboard-handlers` | Clickable elements should have keyboard event handlers |
| `color-contrast` | Text colors should meet WCAG contrast ratios |
| `focus-visible` | Focusable elements should have visible focus indicators |

## Testing

| Rule ID | Description |
|---------|-------------|
| `test-description-meaningful` | Test descriptions should be descriptive, not generic |
| `no-test-id-in-production` | Remove `data-testid` from production builds |
| `test-coverage` | Core logic should have corresponding tests |
| `no-skip-tests` | No skipped or disabled tests committed |
| `no-mocking-core` | Avoid mocking core utilities — test real behavior |
| `assert-includes-message` | Assertions should include descriptive messages |
