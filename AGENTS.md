# Style Guide

## Range-query APIs

- Public range-query methods should accept `R: RangeBounds<usize>` and use the name `query`.
- Put the half-open `[start, end)` implementation in a separate `query_bounds(start, end)` method, and have `query` normalize the supplied bounds before calling it.
- Specialized query variants should follow the same naming pattern. For example, use `query_idempotent` for the `RangeBounds` API and `query_idempotent_bounds` for its half-open bounds implementation.
- Convert excluded starts and included ends with `checked_add(1)` and provide a structure-specific overflow message.
- Interpret unbounded starts as `0` and unbounded ends as the collection length.
